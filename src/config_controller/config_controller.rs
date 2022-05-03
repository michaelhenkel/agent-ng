use async_trait::async_trait;
use serde::Deserialize;
use crate::config_controller::cn2::cn2::CN2ConfigController;
use crate::config_controller::cn2::cn2::Config as CN2Config;
use crate::config_controller::cli::cli::CLIConfigController;
use crate::config_controller::cli::cli::Config as CLIConfig;


#[derive(Deserialize, Debug)]
pub struct Config {
    pub cn2: Option<CN2Config>,
    pub cli: Option<CLIConfig>,
}

#[async_trait]
pub trait ConfigController: Send + Sync {
    async fn run(self) -> Result<(), Box<dyn std::error::Error + Send>>;
    fn name(&self) -> String;
}

pub async fn start(name: String, config: Config) -> Vec<Result<Result<(), Box<dyn std::error::Error + std::marker::Send>>, tokio::task::JoinError>> {
    println!("starting config_controller");
    let mut join_handles = Vec::new();

    let cn2_config = config.cn2.unwrap();
    if cn2_config.enabled.unwrap(){
        let cn2_config_controller = CN2ConfigController::new(cn2_config);
        let res = run(cn2_config_controller);
        let join_handle = tokio::task::spawn(res);
        join_handles.push(join_handle);
    }

    let cli_config = config.cli.unwrap();
    if cli_config.enabled.unwrap(){
        let cli_config_controller = CLIConfigController::new(cli_config);
        let cli_res = run(cli_config_controller);
        let cli_join_handle = tokio::task::spawn(cli_res);
        join_handles.push(cli_join_handle);
    }
    
    futures::future::join_all(join_handles).await
}

pub async fn run<T: 'static + ConfigController>(controller: T) -> Result<(), Box<dyn std::error::Error + Send>> {
    println!("running config_controller {}", controller.name());
    let res = controller.run();
    res.await
}

/*
pub fn get_config_controller(name: String) -> Box<dyn ConfigController + Send> {
    match name.as_str() {
        "CN2" => Box::new(CN2ConfigController::new()),
        "CLI" => Box::new(CLIConfigController::new()),
        _ => Box::new(CLIConfigController::new()),
    }
}

pub fn res_list() -> Vec<String> {
    vec![
        "CN2".to_string(),
        "CLI".to_string(),
    ]
}
*/

