use config_client::protos::github::com::michaelhenkel::config_controller::pkg::apis::v1::config_controller_client::ConfigControllerClient;
use config_client::protos::github::com::michaelhenkel::config_controller::pkg::apis::v1;
use std::collections::HashMap;
use std::vec::Vec;
use std::error::Error;
use crate::resources;
use async_trait::async_trait;

pub struct ResourceController {

}

impl ResourceController {
    pub fn new() -> Self {
        Self{}
    }
    pub async fn run(self, channel: tonic::transport::Channel, receiver: crossbeam_channel::Receiver<v1::Resource>, sender: crossbeam_channel::Sender<v1::Resource>, resource_interface: Box<dyn ResourceInterface + Send>, name: String) -> Result<(), Box<dyn Error + Send >> {
        let mut w_map: HashMap<String,v1::Resource> = HashMap::new();
        let mut r_map: HashMap<String,v1::Resource> = HashMap::new();
        let mut client = ConfigControllerClient::new(channel.clone());
        println!("Starting ResourceController for {}", name);
        loop{
            let resource = receiver.recv().unwrap();
            match v1::resource::Action::from_i32(resource.action){
               Some(v1::resource::Action::Add) => {
                   let resource_key = format!("{}/{}/{}", resource.clone().kind, resource.clone().namespace, resource.clone().name);
                   //println!("Add {}", resource_key.clone());
                    if w_map.contains_key(&resource_key) {
                        //println!("in WQ");
                        if !r_map.contains_key(&resource_key){
                            //println!("not in RQ, pushing it");
                            r_map.insert(resource_key, resource.clone());
                        }
                    } else {
                        w_map.insert(resource_key, resource.clone());
                        //println!("not in WQ, pushing it and starting process");
                        resource_interface.process(&mut client, sender.clone(), resource.clone()).await;
                    }
                },
                Some(v1::resource::Action::Del) => {
                    let resource_key = format!("{}/{}/{}", resource.kind, resource.namespace, resource.name);
                    //println!("Del");
                    if w_map.contains_key(&resource_key) {
                        //println!("in WQ, removing it");
                        w_map.remove(&resource_key);
                    }// else {
                    //    println!("not in WQ");
                    //}
                    if r_map.contains_key(&resource_key){
                        //println!("in RQ, removing it and send it");
                        r_map.remove(&resource_key);
                        let resource = v1::Resource{
                            name: resource.name,
                            namespace: resource.namespace,
                            kind: resource.kind,
                            action: i32::from(v1::resource::Action::Add),
                        };
                        sender.send(resource).unwrap();
                    }
                },
                Some(v1::resource::Action::Retry) => {
                    let resource_key = format!("{}/{}/{}", resource.kind, resource.namespace, resource.name);
                    if r_map.contains_key(&resource_key) {
                        r_map.remove(&resource_key);
                    }
                    resource_interface.process(&mut client, sender.clone(), resource.clone()).await;
                },
                _ => { break; },
            }
            
        }
        Ok(())
    }
}

#[async_trait]
pub trait ResourceInterface: Send + Sync{
    async fn process(&self, client: &mut ConfigControllerClient<tonic::transport::Channel>, sender: crossbeam_channel::Sender<v1::Resource>, resource: v1::Resource);

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