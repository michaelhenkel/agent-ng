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
