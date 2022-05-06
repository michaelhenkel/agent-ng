mod config_controller;
mod config;
mod cache_controller;
use std::borrow::Borrow;

use crossbeam_channel::unbounded;
use agent_ng::protos::github::com::michaelhenkel::config_controller::pkg::apis::v1;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::config::load().unwrap();
    
    let cache_client: cache_controller::cache::Cache = cache_controller::cache::Cache::new();
    let cache_runner = cache_client.run();

    let config_controller_client = config_controller::config_controller::ConfigController::new(config.name.unwrap(), config.config_controller.unwrap(), cache_client.clone());
    let config_controller_runner = config_controller_client.run();
    
    let (config_res, cache_res) = tokio::join!(config_controller_runner, cache_runner);
    //futures::future::join_all(cc);
    
    //let join_handle = tokio::task::spawn(cc);
    //join_handle.await?;
    Ok(())
}



