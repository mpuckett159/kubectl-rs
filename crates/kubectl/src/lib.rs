use std::process::ExitCode;

use clap::{Parser, Subcommand};
use kube::{
    Config,
    config::{KubeConfigOptions, Kubeconfig},
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, global = true, default_value = "~/.kube/config")]
    kubeconfig: String,

    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, global = true, default_value = "tab")]
    output: String,
}

#[derive(Subcommand)]
enum Commands {
    Config(kubectl_config::ConfigArgs),
    Create(kubectl_create::CreateArgs),
    Delete(kubectl_delete::DeleteArgs),
    Get(kubectl_get::GetArgs),
    Patch(kubectl_patch::PatchArgs),
}

pub async fn main() -> Result<(), ExitCode> {
    let args = Cli::parse();

    // Figure out the kubeconfig either from string else from defaults outlined
    // here https://docs.rs/kube/latest/kube/config/struct.Kubeconfig.html
    let kubeconfig =
        Kubeconfig::read_from(shellexpand::tilde(&args.kubeconfig).into_owned()).unwrap();

    let config =
        match Config::from_custom_kubeconfig(kubeconfig.clone(), &KubeConfigOptions::default())
            .await
        {
            Ok(config) => config,
            Err(infer_config_error) => {
                println!(
                    "error parsing kubeconfig into client Config: {}",
                    infer_config_error
                );
                return Err(ExitCode::FAILURE);
            }
        };

    match args.command {
        Commands::Config(config_args) => {
            let config_cmd = config_args
                .command
                .clone()
                .unwrap_or(kubectl_config::ConfigCommands::View);
            match config_cmd {
                kubectl_config::ConfigCommands::View => {
                    kubectl_config::config_view(kubeconfig).await
                }
            }
        }
        Commands::Create(create_args) => kubectl_create::create_resource(create_args, config).await,
        Commands::Delete(delete_args) => kubectl_delete::delete_resource(delete_args, config).await,
        Commands::Get(get_args) => kubectl_get::get_resource(&get_args, config).await,
        Commands::Patch(patch_args) => kubectl_patch::patch_resource(patch_args, config).await,
    }
}
