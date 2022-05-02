use config_client::protos::github::com::michaelhenkel::config_controller::pkg::apis::v1::config_controller_client::ConfigControllerClient;
use config_client::protos::github::com::michaelhenkel::config_controller::pkg::apis::v1;
use crate::resources::resource::{ResourceInterface};
use config_client::protos::ssd_git::juniper::net::contrail::cn2::contrail::pkg::apis::core::v1alpha1;
use async_trait::async_trait;

#[derive(Copy,Clone)]
pub struct VirtualMachineController {}

impl VirtualMachineController {
    pub fn new() -> Self {
        Self{}
    }
}

#[async_trait]
impl ResourceInterface for VirtualMachineController{
    async fn process(&self, client: &mut ConfigControllerClient<tonic::transport::Channel>, sender: crossbeam_channel::Sender<v1::Resource>, resource: v1::Resource){
        let mut client = client.clone();
        tokio::spawn(async move {
            let res_result: Result<tonic::Response<v1alpha1::VirtualMachine>, tonic::Status> = client.get_virtual_machine(resource.clone()).await;
            match res_result {
                Ok(mut res) => {
                    let res: &mut v1alpha1::VirtualMachine = res.get_mut();
                    println!("##########Start: VirtualMachine##########");
                    println!("{}/{}", res.metadata.as_ref().unwrap().namespace(), res.metadata.as_ref().unwrap().name());
                    println!("labels {:?}", res.metadata.as_ref().unwrap().labels);
                    println!("##########Done: VirtualMachine##########");
                    let resource = v1::Resource{
                        name: resource.name,
                        namespace: resource.namespace,
                        kind: resource.kind,
                        action: i32::from(v1::resource::Action::Del),
                    };
                    sender.send(resource).unwrap();
                },
                Err(err) => {
                    if err.code() == tonic::Code::NotFound {
                        let resource = v1::Resource{
                            name: resource.name,
                            namespace: resource.namespace,
                            kind: resource.kind,
                            action: i32::from(v1::resource::Action::Del),
                        };
                        sender.send(resource).unwrap();
                    } else {
                        println!("err {:?}", err);
                        let resource = v1::Resource{
                            name: resource.name,
                            namespace: resource.namespace,
                            kind: resource.kind,
                            action: i32::from(v1::resource::Action::Retry),
                        };
                        sender.send(resource).unwrap();
                    }
                },
            }
        });
        tokio::task::yield_now().await;
    }
}