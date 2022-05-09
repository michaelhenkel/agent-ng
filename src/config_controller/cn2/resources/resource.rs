use agent_ng::protos::github::com::michaelhenkel::config_controller::pkg::apis::v1::config_controller_client::ConfigControllerClient;
use agent_ng::protos::github::com::michaelhenkel::config_controller::pkg::apis::v1;
use std::collections::HashMap;
use std::vec::Vec;
use std::error::Error;
use crate::cache_controller::cache;
use crate::config_controller::cn2::resources;
use async_trait::async_trait;

pub struct ResourceController {
    cache_client: cache::Cache,
    channel: tonic::transport::Channel,
    receiver: crossbeam_channel::Receiver<v1::KeyAction>,
    sender: crossbeam_channel::Sender<v1::KeyAction>,
    resource_interface: Box<dyn ResourceInterface + Send>,
    name: String,
}

impl ResourceController {
    pub fn new(cache_client: cache::Cache, channel: tonic::transport::Channel, receiver: crossbeam_channel::Receiver<v1::KeyAction>, sender: crossbeam_channel::Sender<v1::KeyAction>,resource_interface: Box<dyn ResourceInterface + Send>, name: String) -> Self {
        Self{
            cache_client: cache_client,
            channel: channel,
            receiver: receiver,
            sender: sender,
            name: name,
            resource_interface: resource_interface,
        }
    }
    pub async fn run(self) -> Result<(), Box<dyn Error + Send >> {
        let mut w_map: HashMap<String,v1::Key> = HashMap::new();
        let mut r_map: HashMap<String,v1::Key> = HashMap::new();
        let mut client = ConfigControllerClient::new(self.channel.clone());
        println!("Starting ResourceController for {}", self.name);
        loop{
            let mut key_action = self.receiver.recv().unwrap();
            match v1::key_action::Action::from_i32(key_action.clone().action){
               Some(v1::key_action::Action::Add) => {
                   let resource_key = format!("{}/{}/{}", key_action.clone().key.unwrap().kind, key_action.clone().key.unwrap().namespace, key_action.clone().key.unwrap().name);
                    if w_map.contains_key(&resource_key) {
                        if !r_map.contains_key(&resource_key){
                            r_map.insert(resource_key, key_action.clone().key.unwrap());
                        }
                    } else {
                        w_map.insert(resource_key, key_action.clone().key.unwrap());
                        self.resource_interface.process(&mut client, self.sender.clone(), key_action.clone().key.unwrap(), self.cache_client.clone()).await;
                    }
                },
                Some(v1::key_action::Action::Del) => {
                    let resource_key = format!("{}/{}/{}", key_action.clone().key.unwrap().kind, key_action.clone().key.unwrap().namespace, key_action.clone().key.unwrap().name);
                    if w_map.contains_key(&resource_key) {
                        w_map.remove(&resource_key);
                    }
                    if r_map.contains_key(&resource_key){
                        r_map.remove(&resource_key);
                        key_action.action = i32::from(v1::key_action::Action::Add);
                        self.sender.send(key_action.clone()).unwrap();
                    }
                },
                Some(v1::key_action::Action::Retry) => {
                    let resource_key = format!("{}/{}/{}", key_action.clone().key.unwrap().kind, key_action.clone().key.unwrap().namespace, key_action.clone().key.unwrap().name);
                    if r_map.contains_key(&resource_key) {
                        r_map.remove(&resource_key);
                    }
                    self.resource_interface.process(&mut client, self.sender.clone(), key_action.clone().key.unwrap(), self.cache_client.clone()).await;
                },
                _ => { break; },
            }
            
        }
        Ok(())
    }
}

#[async_trait]
pub trait ResourceInterface: Send + Sync{
    async fn process(&self, client: &mut ConfigControllerClient<tonic::transport::Channel>, sender: crossbeam_channel::Sender<v1::KeyAction>, key: v1::Key, cache_client: cache::Cache);

}

pub fn res_list() -> Vec<String> {
    vec![
        "VirtualNetwork".to_string(),
        "VirtualMachineInterface".to_string(),
        "VirtualMachine".to_string(),
    ]
}

pub fn get_res(name: String) -> Box<dyn ResourceInterface + Send> {
    match name.as_str() {
        "VirtualNetwork" => Box::new(resources::virtualnetwork::VirtualNetworkController::new()),
        "VirtualMachineInterface" => Box::new(resources::virtualmachineinterface::VirtualMachineInterfaceController::new()),
        "VirtualMachine" => Box::new(resources::virtualmachine::VirtualMachineController::new()),
        _ => Box::new(resources::virtualmachineinterface::VirtualMachineInterfaceController::new()),
    }
}