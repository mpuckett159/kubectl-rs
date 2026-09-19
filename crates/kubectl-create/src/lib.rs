use std::fs::File;
use std::process::ExitCode;

use clap::Args;
use kube::{Api, Config, api::PostParams, core::DynamicObject, discovery};
use serde_yaml::Value;

#[derive(Debug, Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct CreateArgs {
    pub file: String,
}

pub async fn create_resource(create_args: CreateArgs, config: Config) -> Result<(), ExitCode> {
    let client = kube::Client::try_from(config).unwrap();
    let file_val: Value =
        serde_yaml::from_reader(File::open(create_args.file.clone()).unwrap()).unwrap();
    let api_version = file_val["apiVersion"].as_str().unwrap();
    let kind = file_val["kind"].as_str().unwrap();
    let gv = api_version.parse().unwrap();
    let apigroup = discovery::pinned_group(&client, &gv).await.unwrap();
    let (ar, _) = apigroup.recommended_kind(kind).unwrap();
    let api: Api<DynamicObject> = match file_val["metadata"]["namespace"].clone() {
        Value::Null => Api::all_with(client.clone(), &ar),
        Value::String(ns) => Api::namespaced_with(client.clone(), &ns, &ar),
        _ => return Err(ExitCode::FAILURE),
    };
    let file_obj: DynamicObject =
        serde_yaml::from_reader(File::open(create_args.file.clone()).unwrap()).unwrap();
    api.create(&PostParams::default(), &file_obj).await.unwrap();
    Ok(())
}
