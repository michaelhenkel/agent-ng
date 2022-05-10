use agent_ng::protos::github::com::michaelhenkel::config_controller::pkg::apis::v1::config_controller_client::ConfigControllerClient;
use agent_ng::protos::github::com::michaelhenkel::config_controller::pkg::apis::v1;
use crate::config_controller::cn2::resources::resource::{ResourceInterface};
use agent_ng::protos::ssd_git::juniper::net::contrail::cn2::contrail::pkg::apis::core::v1alpha1;
use async_trait::async_trait;
use crate::cache_controller::cache::{Cache, Action, ResourceKeyReferences};

#[derive(Copy,Clone)]
pub struct VirtualMachineController {}

impl VirtualMachineController {
    pub fn new() -> Self {
        Self{}
    }
}

#[async_trait]
impl ResourceInterface for VirtualMachineController{
    async fn process(&self, client: &mut ConfigControllerClient<tonic::transport::Channel>, sender: crossbeam_channel::Sender<v1::KeyAction>, key: v1::Key, cache_client: Cache){
        let mut client = client.clone();
        tokio::spawn(async move {
            let res_result: Result<tonic::Response<v1alpha1::VirtualMachine>, tonic::Status> = client.get_virtual_machine(key.clone()).await;
            match res_result {
                Ok(mut res) => {
                    let res: &mut v1alpha1::VirtualMachine = res.get_mut();
                    println!("##########Start: VirtualMachine##########");
                    println!("{}/{}", res.metadata.as_ref().unwrap().namespace(), res.metadata.as_ref().unwrap().name());
                    println!("labels {:?}", res.metadata.as_ref().unwrap().labels);
                    println!("##########Done: VirtualMachine##########");
                    let key_action = v1::KeyAction{
                        key: Some(key.clone()),
                        action: i32::from(v1::key_action::Action::Del),
                    };
                    let references: Vec<v1alpha1::ResourceReference> = Vec::new();

                    let cache_add_result = cache_client.add(ResourceKeyReferences::VirtualMachine(res.clone(), key.clone(), references));
                    match cache_add_result{
                        Ok(()) => {
                            sender.send(key_action).unwrap();
                        },
                        Err(e) => {
                            let key_action = v1::KeyAction{
                                key: Some(key),
                                action: i32::from(v1::key_action::Action::Retry),
                            };
                            sender.send(key_action).unwrap();
                        },
                    };
                    //cache_client.add(resource.clone());
                    //sender.send(key_action).unwrap();
                },
                Err(err) => {
                    if err.code() == tonic::Code::NotFound {
                        let key_action = v1::KeyAction{
                            key: Some(key),
                            action: i32::from(v1::key_action::Action::Del),
                        };
                        sender.send(key_action).unwrap();
                    } else {
                        println!("err {:?}", err);
                        let key_action = v1::KeyAction{
                            key: Some(key),
                            action: i32::from(v1::key_action::Action::Retry),
                        };
                        sender.send(key_action).unwrap();
                    }
                },
            }
        });
        tokio::task::yield_now().await;
    }
}