use std::process::ExitCode;

use kube::config::Kubeconfig;

use clap::{Args, Subcommand};

#[derive(Debug, Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct ConfigArgs {
    /// print config file to stdout
    #[command(subcommand)]
    pub command: Option<ConfigCommands>,
}

impl ConfigArgs {
    pub fn new(command: Option<ConfigCommands>) -> ConfigArgs {
        ConfigArgs {
            command: if command.is_some() { Option::from(command) } else { Option::from(ConfigCommands::View) },
        }
    }
}

#[derive(Debug, Subcommand, Clone)]
pub enum ConfigCommands {
    /// print config file to stdout
    View,
}

pub async fn config_view(kubeconfig: Kubeconfig) -> Result<(), ExitCode> {
    let config_out = match serde_yaml::to_string(&kubeconfig) {
        Ok(config_output) => config_output,
        Err(err) => {
            println!("error converting kubeconfig at somewhere to yaml: {}", err);
            return Err(ExitCode::FAILURE);
        }
    };
    println!("{}", config_out);
    Ok(())
}