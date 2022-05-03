use agent_ng::protos::github::com::michaelhenkel::config_controller::pkg::apis::v1::config_controller_client::ConfigControllerClient;
use agent_ng::protos::github::com::michaelhenkel::config_controller::pkg::apis::v1::SubscriptionRequest;
use agent_ng::protos::github::com::michaelhenkel::config_controller::pkg::apis::v1;
use std::collections::HashMap;
use std::error::Error;
use std::env;
use std::vec::Vec;
use serde::Deserialize;
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

#[derive(Deserialize, Debug)]
pub struct Config {
    pub enabled: Option<bool>,
    server: Option<String>,
}

pub struct CN2ConfigController {
    config: Config,
    name: String,
}

impl CN2ConfigController{
    pub fn new(name: String, config: Config) -> Self {
        Self{
            config: config,
            name: name,
        }
    }    
}

#[async_trait]
impl ConfigController for CN2ConfigController{
    fn name(&self) -> String{
        "CN2ConfigController".to_string()
    }
    async fn run(self) -> Result<(), Box<dyn std::error::Error + Send>> {
        println!("running cn2 plugin");
        let server = string_to_static_str(self.config.server.unwrap());
        let channel = Endpoint::from_static(server)
            .connect()
            .await.unwrap();
        let sender_map: Arc<Mutex<HashMap<String,crossbeam_channel::Sender<v1::Resource>>>> = Arc::new(Mutex::new(HashMap::new()));
        let mut join_handles = Vec::new();
        for r in res_list(){
            let (sender, receiver): (crossbeam_channel::Sender<v1::Resource>, crossbeam_channel::Receiver<v1::Resource>) = unbounded();
            let mut sender_map = sender_map.lock().await;
            let sender_clone = sender.clone();
            sender_map.insert(r.to_string(), sender);
            let rc = ResourceController::new();
            let res = get_res(r.clone());
            let run_res = rc.run(channel.clone(), receiver, sender_clone, res, r.to_string()).map_err(|_| "Unable to get book".to_string());
            let join_handle = tokio::task::spawn(run_res);
            join_handles.push(join_handle);
        }
        let subscribe_thread = subscribe(channel.clone(), sender_map, self.name).map_err(|_| "Unable to get book".to_string());
        let join_handle = tokio::task::spawn(subscribe_thread);
    
        join_handles.push(join_handle);
        futures::future::join_all(join_handles).await;
        Ok(())
    }
    
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

async fn subscribe(channel: tonic::transport::Channel, sender_map: Arc<Mutex<HashMap<String,crossbeam_channel::Sender<v1::Resource>>>>, name: String) -> Result<(), Box<dyn Error>> {
    println!("started subscriber_controller");
    let mut client = ConfigControllerClient::new(channel.clone());
    let request = tonic::Request::new(SubscriptionRequest {
        name: name,
    });

    let mut stream = client
        .subscribe_list_watch(request)
        .await?
        .into_inner();

    while let Some(resource) = stream.message().await? {
        //println!("got resource");
        let sender_map = sender_map.lock().await;
        if let Some(sender) = sender_map.get(resource.kind.as_str()) {
            if let Err(err) = sender.send(resource){
                println!("{:?}", err);
            }
        }
    }
    Ok(())
}

fn get_node() -> String {
    if env::args().len() > 0 {
        let args: Vec<String> = env::args().collect();
        args[1].to_string()
    } else {
        "5b3s30".to_string()
    }
}