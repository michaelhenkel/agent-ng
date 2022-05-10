use crate::config_controller::config_controller::ConfigControllerInterface;
use async_trait::async_trait;
use serde::Deserialize;
use agent_ng::protos::github::com::michaelhenkel::config_controller::pkg::apis::v1;
use agent_ng::protos::ssd_git::juniper::net::contrail::cn2::contrail::pkg::apis::core::v1alpha1;
use agent_ng::protos::github::com::michaelhenkel::config_controller::pkg::apis::v1::cli_server::{Cli, CliServer};
use crate::cache_controller::cache::Cache;
use tonic::{transport::Server, Request, Response, Status};

#[derive(Debug)]
pub struct CliService {
    cache_client: Cache,
}

#[tonic::async_trait]
impl Cli for CliService {
    async fn get(
        &self,
        request: Request<v1::Key>, // Accept request of type HelloRequest
    ) -> Result<Response<v1::Resource>, Status> {
        println!("Got a request: {:?}", request);
        let resource = self.cache_client.get(request.into_inner());
        Ok(Response::new(resource))
    }
    async fn find(
        &self,
        request: Request<v1::FromToFilter>, // Accept request of type HelloRequest
    ) -> Result<Response<v1::ResourceList>, Status> {
        println!("Got a request: {:?}", request);
        let resource = self.cache_client.find(request.into_inner());
        Ok(Response::new(resource))
    }
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub enabled: Option<bool>,
    socket_path: Option<String>,
}
pub struct CLIConfigController {
    config: Config,
    name: String,
    cache_client: Cache,
}

impl CLIConfigController{
    pub fn new(name: String, config: Config, cache_client: Cache) -> Self {
        Self{
            config,
            name,
            cache_client,
        }
    }    
}

#[async_trait]
impl ConfigControllerInterface for CLIConfigController{
    fn name(&self) -> String{
        "CLIConfigController".to_string()
    }
    async fn run(self) -> Result<(), Box<dyn std::error::Error + Send>> {
        println!("running cli plugin");
        let addr = "[::1]:50051".parse().unwrap();
        let cli_service = CliService{
            cache_client: self.cache_client.clone(),
        };
    
        Server::builder()
            .add_service(CliServer::new(cli_service))
            .serve(addr)
            .await;
        Ok(())
    }
}
