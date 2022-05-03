use crate::config_controller::config_controller::ConfigController;
use async_trait::async_trait;
use serde::Deserialize;
use agent_ng::protos::github::com::michaelhenkel::config_controller::pkg::apis::v1;


#[derive(Deserialize, Debug)]
pub struct Config {
    pub enabled: Option<bool>,
    socket_path: Option<String>,
}
pub struct CLIConfigController {
    config: Config,
    name: String,
}

impl CLIConfigController{
    pub fn new(name: String, config: Config) -> Self {
        Self{
            config,
            name,
        }
    }    
}

#[async_trait]
impl ConfigController for CLIConfigController{
    fn name(&self) -> String{
        "CLIConfigController".to_string()
    }
    async fn run(self, cache_channel: crossbeam_channel::Sender<v1::Resource>) -> Result<(), Box<dyn std::error::Error + Send>> {
        println!("running cli plugin");
        Ok(())
    }
}
