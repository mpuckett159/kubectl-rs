use std::env;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use kube::{
    Config,
    config::{Kubeconfig, KubeConfigOptions},
};

fn get_default_kubeconfig_path() -> PathBuf {
    let mut path = env::home_dir().unwrap();
    path.push(".kube");
    path.push("config");
    path
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, global = true, default_value = get_default_kubeconfig_path().into_os_string())]
    kubeconfig: Option<String>,

    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, global = true, default_value = "tab")]
    output: String,
}

#[derive(Subcommand)]
enum Commands {
    Get(kubectl_get::GetArgs),
    Apply(kubectl_apply::ApplyArgs),
    Delete {
        /// delete a resource
        resource: String,
    },
    Config(kubectl_config::ConfigArgs)
}

pub async fn main() -> Result<(), ExitCode> {
    let args = Cli::parse();

    // Figure out the kubeconfig either from string else from defaults outlined
    // here https://docs.rs/kube/latest/kube/config/struct.Kubeconfig.html
    let kubeconfig = match &args.kubeconfig {
        Some(in_kubeconfig) => {
            match Kubeconfig::read_from(Path::new(in_kubeconfig)) {
                Ok(kubeconfig) => kubeconfig,
                Err(read_err) => {
                    println!("Error reading kubeconfig: {}", read_err);
                    return Err(ExitCode::FAILURE)
                }
            }
        }
        _ => {
            match Kubeconfig::read() {
                Ok(config) => {
                    println!("loaded default config");
                    config
                },
                Err(infer_config_error) => {
                    println!("error loading kubeconfig from default sources: {}", infer_config_error);
                    return Err(ExitCode::FAILURE);
                }
            }
        }
    };

    let config = match Config::from_custom_kubeconfig(kubeconfig.clone(), &KubeConfigOptions::default()).await {
        Ok(config) => config,
        Err(infer_config_error) => {
            println!("error parsing kubeconfig into client Config: {}", infer_config_error);
            return Err(ExitCode::FAILURE);
        }
    };

    match args.command {
        Commands::Get(get_args) => {
            kubectl_get::get_resource(&get_args, config).await
        }
        Commands::Apply(apply_args) => {
            println!("Applying a resource");
            kubectl_apply::apply_resource(apply_args, config).await
        }
        Commands::Delete { resource } => {
            println!("Deleting resource: {resource}");
            Ok(())
        }
        Commands::Config(config_args) => {
            let config_cmd = config_args.command.clone().unwrap_or(kubectl_config::ConfigCommands::View);
            match config_cmd {
                kubectl_config::ConfigCommands::View => {
                    kubectl_config::config_view(kubeconfig).await
                }
            }
        }
    }
}