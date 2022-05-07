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
    ) -> Result<Response<v1::Resource>, Status> { // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);
        /*
        self.cache_channel.send(v1::Resource{

        });
        */
        let resource = self.cache_client.get(request.into_inner());
        /*
        println!("resource: {:?}", resource);
        let reply = agentcli::Resource {
            resource: agentcli::Resource{
                resource: alphav1::V
            },
        };
        */
        Ok(Response::new(resource)) // Send back our formatted greeting
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
impl ConfigControllerInterface for CLIConfigController{
    fn name(&self) -> String{
        "CLIConfigController".to_string()
    }
    /*
    async fn run(self, cache_channel: crossbeam_channel::Sender<v1::Resource>) -> Result<(), Box<dyn std::error::Error + Send>> {
        println!("running cli plugin");
        Ok(())
    }
    */
    async fn run(self, cc: Cache) -> Result<(), Box<dyn std::error::Error + Send>> {
        println!("running cli plugin");
        let addr = "[::1]:50051".parse().unwrap();
        let cli_service = CliService{
            cache_client: cc,
        };
    
        Server::builder()
            .add_service(CliServer::new(cli_service))
            .serve(addr)
            .await;
        Ok(())
    }
}

/*
use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};

pub mod hello_world {
    tonic::include_proto!("helloworld"); // The string specified here must match the proto package name
}

*/
