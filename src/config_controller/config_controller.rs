use async_trait::async_trait;
use std::error::Error;
use crate::config_controller::cn2::cn2::CN2ConfigController;
use crate::config_controller::cli::cli::CLIConfigController;

#[async_trait]
pub trait ConfigController: Send + Sync {
    async fn run(&self) -> Result<(), Box<dyn std::error::Error + Send>>;
    fn name(&self) -> String;
}

//pub async fn start() -> Result<(), Box<dyn std::error::Error + Send>> {
//pub async fn start() -> Vec<tokio::task::JoinHandle<std::result::Result<(), std::boxed::Box<dyn std::error::Error + std::marker::Send>>>> {
pub async fn start() -> Vec<Result<Result<(), Box<dyn std::error::Error + std::marker::Send>>, tokio::task::JoinError>> {
    println!("starting config_controller");
    let mut join_handles = Vec::new();

    let cn2_config_controller = CN2ConfigController::new();
    let res = run(cn2_config_controller);
    let join_handle = tokio::task::spawn(res);
    join_handles.push(join_handle);

    let cli_config_controller = CLIConfigController::new();
    let cli_res = run(cli_config_controller);
    let cli_join_handle = tokio::task::spawn(cli_res);
    join_handles.push(cli_join_handle);
    
    futures::future::join_all(join_handles).await
    //Ok(())
    //join_handles
}

//pub async fn run<T: 'static + ConfigController>(controller: T) -> std::pin::Pin<std::boxed::Box<dyn futures::Future<Output = std::result::Result<(), std::boxed::Box<(dyn std::error::Error + std::marker::Send + 'static)>>> + std::marker::Send>> {
pub async fn run<T: 'static + ConfigController>(controller: T) -> Result<(), Box<dyn std::error::Error + Send>> {
    println!("running config_controller {}", controller.name());
    let res = controller.run();
    res.await
    //Ok(())
    //let join_handle = tokio::task::spawn(res);
    //res.await?;
    //join_handle
}

