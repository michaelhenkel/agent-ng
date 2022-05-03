use serde::Deserialize;
use figment::{Figment, providers::{Format, Yaml}};
use std::path::Path;
use clap::{Arg, Command};
use crate::config_controller::config_controller::Config as ConfigControllerConfig;
//use figment::Figment;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub name: Option<String>,
    pub config_controller: Option<ConfigControllerConfig>,

}

pub fn load()  -> Result<Config, Box<dyn std::error::Error>> {
    let matches = Command::new("AgentNG")
        .version("0.1.0")
        .author("Michael Henkel <>")
        .about("Bla")
        .arg(Arg::new("file")
                 .short('f')
                 .long("file")
                 .takes_value(true)
                 .help("config file path"))
        .get_matches();

    let config_file = matches.value_of("file").unwrap_or("config.yaml");

    if Path::new(config_file).exists(){
        println!("file: {}", config_file);
        let config: Config = Figment::new().merge(Yaml::file(config_file)).extract().unwrap();
        println!("config: {:#?}", config);
        return Ok(config)
    }

    Ok(Config{
        name: Some("bla".to_string()),
        config_controller: None,
    })
}