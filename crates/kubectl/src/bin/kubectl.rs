use std::process::ExitCode;
use kubectl::main as kubectl_main;

#[tokio::main]
async fn main() -> Result<(), ExitCode> {
    kubectl_main().await
}