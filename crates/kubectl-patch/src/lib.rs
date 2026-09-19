use std::fs::File;
use std::process::ExitCode;

use clap::Args;
use kube::{
    Api, Config,
    api::{Patch, PatchParams},
    core::DynamicObject,
    discovery,
};
use serde_yaml::Value;

#[derive(Debug, Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct PatchArgs {
    pub file: String,
}

pub async fn patch_resource(patch_args: PatchArgs, config: Config) -> Result<(), ExitCode> {
    let client = kube::Client::try_from(config).unwrap();
    let file_val: Value =
        serde_yaml::from_reader(File::open(patch_args.file.clone()).unwrap()).unwrap();
    let name = file_val["metadata"]["name"].as_str().unwrap();
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
    let patch_obj: DynamicObject =
        serde_yaml::from_reader(File::open(patch_args.file.clone()).unwrap()).unwrap();
    let patch = Patch::Apply(&patch_obj);
    let patch_params = PatchParams::apply("kubectl-apply-server-side").force();
    println!(
        "Patching \"{}/{}/{}\" using file \"{}\" with Patch::Apply = {:?}",
        apigroup.name(),
        kind,
        name,
        patch_args.file,
        patch
    );
    api.patch(name, &patch_params, &patch).await.unwrap();
    Ok(())
}
