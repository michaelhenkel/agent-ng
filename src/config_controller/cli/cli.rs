use agent_ng::protos::github::com::michaelhenkel::config_controller::pkg::apis::v1::config_controller_client::ConfigControllerClient;
use agent_ng::protos::github::com::michaelhenkel::config_controller::pkg::apis::v1::SubscriptionRequest;
use agent_ng::protos::github::com::michaelhenkel::config_controller::pkg::apis::v1;
use std::collections::HashMap;
use std::error::Error;
use std::env;
use std::vec::Vec;
use super::super::super::resources;
use tonic::transport::Endpoint;
use crossbeam_channel::unbounded;
use crate::resources::resource::{get_res, res_list, ResourceController};
use std::sync::Arc;
use tokio::sync::Mutex;
use futures;
use futures::future::TryFutureExt;
use crate::config_controller::config_controller::ConfigController;
use async_trait::async_trait;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct Config {
    pub enabled: Option<bool>,
    socket_path: Option<String>,
}
pub struct CLIConfigController {}

impl CLIConfigController{
    pub fn new(config: Config) -> Self {
        Self{}
    }    
}

#[async_trait]
impl ConfigController for CLIConfigController{
    fn name(&self) -> String{
        "CLIConfigController".to_string()
    }
    async fn run(self) -> Result<(), Box<dyn std::error::Error + Send>> {
        println!("running cli plugin");
        Ok(())
    }
}
