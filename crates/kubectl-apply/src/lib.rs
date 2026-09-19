use std::fs::File;
use std::process::ExitCode;

use clap::Args;
use kube::api::PostParams;
use kube::{Api, Config, core::DynamicObject, discovery};

#[derive(Debug, Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct ApplyArgs {
    pub file: String,
}

pub async fn apply_resource(apply_args: ApplyArgs, config: Config) -> Result<(), ExitCode> {
    let client: kube::Client = kube::Client::try_from(config).unwrap();
    let file_val: DynamicObject =
        serde_yaml::from_reader(File::open(apply_args.file).unwrap()).unwrap();
    let type_meta = file_val.clone().types.unwrap();
    let api_version = type_meta.api_version;
    let group: Vec<&str> = api_version.split("/").collect();
    match group.len() {
        2 => {
            let apigroup = discovery::group(&client, "").await.unwrap();
            let (ar, _) = apigroup.recommended_kind(type_meta.kind.as_str()).unwrap();
            let api: Api<DynamicObject> = Api::namespaced_with(
                client,
                file_val.metadata.namespace.clone().unwrap().as_str(),
                &ar,
            );
            api.create(&PostParams::default(), &file_val).await.unwrap();
        }
        _ => {
            let apigroup = discovery::group(&client, "").await.unwrap();
            let (ar, _) = apigroup.recommended_kind(type_meta.kind.as_str()).unwrap();
            let api: Api<DynamicObject> = Api::namespaced_with(
                client,
                file_val.metadata.namespace.clone().unwrap().as_str(),
                &ar,
            );
            api.create(&PostParams::default(), &file_val).await.unwrap();
        }
    }
    Ok(())
}
