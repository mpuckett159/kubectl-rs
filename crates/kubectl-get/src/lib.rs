use std::collections::BTreeMap;
use std::io::{Write, stdout};
use std::process::ExitCode;

use kubectl_groups::Groups;
use kubectl_printers::PrinterColumns;

use clap::Parser;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ListMeta;
use kube::{
    Client, Config, ResourceExt,
    api::{Api, ListParams, ObjectList, TypeMeta},
    config::KubeConfigOptions,
    core::{ApiResource, DynamicObject, GroupVersionKind},
};
use serde_json_path::JsonPath;
use tabprinter::{Alignment, Cell, Table, TableStyle};

#[derive(Debug)]
pub enum GetError {
    GetResourcesMissing(String),
    ContextGroupMissing(String),
}

#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
pub struct GetArgs {
    resource_type: String,
    resource_name: Option<String>,

    #[arg(short, long, default_value = "tab")]
    output: String,

    #[arg(short, long, default_value = "column")]
    namespace: String,

    #[arg(short, long)]
    cluster_group: Option<String>,

    // If present: true, if absent: false.
    #[arg(short, long, help = "Pass flag to print managed fields in output")]
    managed_fields: bool,
}

async fn get_one_resource(
    client: Client,
    get_args: &GetArgs,
    resource_type: &ApiResource,
) -> Result<DynamicObject, GetError> {
    let namespace = get_args.namespace.as_str();
    let api: Api<DynamicObject> = Api::namespaced_with(client, &namespace, resource_type);
    match api
        .get(get_args.resource_name.clone().unwrap().as_str())
        .await
    {
        Ok(mut o) => {
            // remove managed fields from object
            if !get_args.managed_fields {
                o.managed_fields_mut().clear();
            }
            Ok(o)
        }
        Err(e) => Err(GetError::ContextGroupMissing(e.to_string())),
    }
}

async fn get_multiple_resources(
    client: Client,
    get_args: &GetArgs,
    resource_type: &ApiResource,
) -> Result<ObjectList<DynamicObject>, GetError> {
    let namespace = get_args.clone().namespace;
    let api: Api<DynamicObject> = Api::namespaced_with(client, &namespace, resource_type);
    match api.list(&ListParams::default()).await {
        Ok(mut o) => {
            // remove managed fields from object
            if !get_args.managed_fields {
                o.items
                    .iter_mut()
                    .for_each(|item| item.managed_fields_mut().clear());
            }
            Ok(o)
        }
        Err(e) => Err(GetError::ContextGroupMissing(e.to_string())),
    }
}

fn print_json(tree: MultiClusterMap) -> Result<(), ExitCode> {
    for key in tree.resources.keys() {
        let value: serde_json::Value = serde_json::to_value(&tree.resources.get(key)).unwrap();
        let out = stdout();
        let mut out = out.lock();
        match colored_json::write_colored_json(&value, &mut out) {
            Ok(_) => (),
            Err(_) => {}
        }
        match out.flush() {
            Ok(_) => (),
            _ => return Err(ExitCode::FAILURE),
        }
    }
    Ok(())
}

async fn config_map_generator(
    context_group_name: String,
) -> Result<BTreeMap<String, Config>, GetError> {
    let mut config_map: BTreeMap<String, Config> = BTreeMap::new();
    let context_groups: Groups = match Groups::load() {
        Ok(context_groups) => context_groups,
        Err(e) => return Err(GetError::ContextGroupMissing(e.to_string())),
    };
    let contexts: Vec<String> = context_groups
        .context_groups
        .get(&context_group_name)
        .unwrap()
        .clone();
    for context in contexts {
        config_map.insert(
            context.clone(),
            Config::from_kubeconfig(&KubeConfigOptions {
                context: Option::from(context),
                cluster: None,
                user: None,
            })
            .await
            .unwrap(),
        );
    }
    Ok(config_map)
}

struct MultiClusterMap {
    configs: BTreeMap<String, Config>,
    clients: BTreeMap<String, Client>,
    resources: BTreeMap<String, ObjectList<DynamicObject>>,
}

impl MultiClusterMap {
    fn new() -> MultiClusterMap {
        MultiClusterMap {
            configs: Default::default(),
            clients: Default::default(),
            resources: Default::default(),
        }
    }
}

pub async fn get_resource(get_args: &GetArgs, config: Config) -> Result<(), ExitCode> {
    let mut tree = MultiClusterMap::new();
    match get_args.cluster_group.clone() {
        Some(cluster_group) => {
            tree.configs
                .append(&mut config_map_generator(cluster_group).await.unwrap());
        }
        None => {
            tree.configs.insert("default".to_string(), config);
        }
    };
    for key in tree.configs.keys() {
        tree.clients.insert(
            key.clone(),
            Client::try_from(tree.configs.get(key).cloned().unwrap()).unwrap(),
        );
    }
    let gvk_vec: Vec<&str> = get_args.resource_type.split('/').collect();
    let resource_type =
        &ApiResource::from_gvk(&GroupVersionKind::gvk(gvk_vec[0], gvk_vec[1], gvk_vec[2]));
    for key in tree.clients.keys() {
        match get_args.resource_name.clone() {
            Some(_) => {
                let resource = get_one_resource(
                    tree.clients.get(key).cloned().unwrap(),
                    get_args,
                    resource_type,
                )
                .await
                .unwrap();
                tree.resources.insert(
                    key.clone(),
                    ObjectList {
                        types: TypeMeta {
                            api_version: "v1".to_string(),
                            kind: "PodList".to_string(),
                        },
                        metadata: ListMeta::default(),
                        items: vec![resource],
                    },
                );
            }
            _ => {
                let resources = get_multiple_resources(
                    tree.clients.get(key).cloned().unwrap(),
                    get_args,
                    resource_type,
                )
                .await
                .unwrap();
                tree.resources.insert(key.clone(), resources);
            }
        }
    }
    match get_args.output.as_ref() {
        "json" => print_json(tree)?,
        "yaml" => {
            for key in tree.resources.keys() {
                let value = serde_yaml::to_string(&tree.resources.get(key)).unwrap();
                println!("{}", value);
            }
        }
        "tab" | "wide" => tab_list_printer(tree, get_args)?,
        &_ => {
            println!(
                "Output format not supported, please use one of 'json', 'yaml', 'tab'. Provided {}",
                get_args.output
            );
        }
    }

    Ok(())
}

fn tab_list_printer(tree: MultiClusterMap, get_args: &GetArgs) -> Result<(), ExitCode> {
    let printer: PrinterColumns = PrinterColumns::pods();

    let mut table = Table::new(TableStyle::Neon);

    table.add_column("context", Alignment::Left);

    for column in printer.columns.iter().clone() {
        if get_args.output == "wide" && column.priority > 0 {
            table.add_column(column.name.as_str(), Alignment::Left);
        } else if get_args.output != "wide" && column.priority == 0 {
            table.add_column(column.name.as_str(), Alignment::Left);
        }
    }

    let mut prev: &str = "";
    for key in tree.resources.keys() {
        for resource in tree.resources.get(key).clone().unwrap() {
            let resource_value = serde_json::to_value(resource.clone()).unwrap();
            let mut row: Vec<Cell> = Vec::new();
            if prev != key {
                row.push(Cell::new(key));
            } else {
                row.push(Cell::new(""));
            }
            for column in printer.columns.iter().clone() {
                if get_args.output == "wide" && column.priority > 0 {
                    let path = JsonPath::parse(column.json_path.as_str()).unwrap();
                    let node_list = path.query(&resource_value);
                    let mut cell_value_vec: Vec<String> = Vec::new();
                    for node in node_list {
                        cell_value_vec.push(node.to_string());
                    }
                    row.push(Cell::new(cell_value_vec.join(", ").as_str()));
                } else if get_args.output != "wide" && column.priority == 0 {
                    let path = JsonPath::parse(column.json_path.as_str()).unwrap();
                    let node_list = path.query(&resource_value);
                    let mut cell_value_vec: Vec<String> = Vec::new();
                    for node in node_list {
                        cell_value_vec.push(node.to_string());
                    }
                    row.push(Cell::new(cell_value_vec.join(", ").as_str()));
                }
            }
            table.add_row(row);
            prev = key;
        }
    }

    table.print().expect("TODO: panic message");
    Ok(())
}
