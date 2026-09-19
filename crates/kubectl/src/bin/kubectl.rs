use kubectl::main as kubectl_main;
use std::process::ExitCode;

#[tokio::main]
async fn main() -> Result<(), ExitCode> {
    kubectl_main().await
}
