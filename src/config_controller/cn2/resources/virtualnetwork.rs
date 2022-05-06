use agent_ng::protos::github::com::michaelhenkel::config_controller::pkg::apis::v1::config_controller_client::ConfigControllerClient;
use agent_ng::protos::github::com::michaelhenkel::config_controller::pkg::apis::v1;
use crate::config_controller::cn2::resources::resource::{ResourceInterface};
use agent_ng::protos::ssd_git::juniper::net::contrail::cn2::contrail::pkg::apis::core::v1alpha1;
use async_trait::async_trait;

#[derive(Copy,Clone)]
pub struct VirtualNetworkController {}

impl VirtualNetworkController {
    pub fn new() -> Self {
        Self{}
    }
}

#[async_trait]
impl ResourceInterface for VirtualNetworkController{
    async fn process(&self, client: &mut ConfigControllerClient<tonic::transport::Channel>, sender: crossbeam_channel::Sender<v1::Resource>, resource: v1::Resource){
        let mut client = client.clone();
        tokio::spawn(async move {
            let resource_key = format!("{}/{}/{}", resource.clone().kind, resource.clone().namespace, resource.clone().name);
            println!("starting VirtualNetwork process for {}", resource_key);
            let res_result: Result<tonic::Response<v1alpha1::VirtualNetwork>, tonic::Status> = client.get_virtual_network(resource.clone()).await;
            match res_result {
                Ok(mut res) => {
                    let res: &mut v1alpha1::VirtualNetwork = res.get_mut();
                    println!("##########Start: VirtualNetwork##########");
                    println!("{}/{}", res.metadata.as_ref().unwrap().namespace(), res.metadata.as_ref().unwrap().name());
                    println!("labels {:?}", res.metadata.as_ref().unwrap().labels);
                    println!("##########Done: VirtualNetwork##########");
                    let mut ref_list: Vec<v1alpha1::ResourceReference> = Vec::new();
                    let v4_subnet_ref = res.spec.as_ref().unwrap().v4_subnet_reference.to_owned().unwrap();
                    ref_list.push(v4_subnet_ref);
                    
                    match res.spec.as_ref().unwrap().v6_subnet_reference.to_owned(){
                        Some(v6_subnet_ref) => {
                            ref_list.push(v6_subnet_ref);
                        },
                        _ => {}, 
                    }
                    let mut resource = v1::Resource{
                        name: resource.name,
                        namespace: resource.namespace,
                        kind: resource.kind,
                        action: i32::from(v1::resource::Action::Del),
                        references: ref_list,
                    };
                    //cache_channel.send(resource.clone()).unwrap();
                    sender.send(resource.clone()).unwrap();
                    resource.action = i32::from(v1::resource::Action::Add);
                    //cache_channel.send(resource.clone()).unwrap();
                },
                Err(err) => {
                    if err.code() == tonic::Code::NotFound {
                        let resource = v1::Resource{
                            name: resource.name,
                            namespace: resource.namespace,
                            kind: resource.kind,
                            action: i32::from(v1::resource::Action::Del),
                            references: resource.references,
                        };
                        sender.send(resource.clone()).unwrap();
                        //cache_channel.send(resource.clone()).unwrap();
                    } else {
                        println!("err {:?}", err);
                        let resource = v1::Resource{
                            name: resource.name,
                            namespace: resource.namespace,
                            kind: resource.kind,
                            action: i32::from(v1::resource::Action::Retry),
                            references: resource.references,
                        };
                        sender.send(resource).unwrap();
                    }
                },
            }
        });
        tokio::task::yield_now().await;
    }
}
