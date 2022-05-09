extern crate lru;

use lru::LruCache;
use super::graph::Graph;
use std::hash::Hash;
use agent_ng::protos::github::com::michaelhenkel::config_controller::pkg::apis::v1;
use agent_ng::protos::ssd_git::juniper::net::contrail::cn2::contrail::pkg::apis::core::v1alpha1;

#[derive(Clone, Debug)]
pub struct Cache {
    receiver: crossbeam_channel::Receiver<Action>,
    sender: crossbeam_channel::Sender<Action>,
    s1: crossbeam_channel::Sender<v1::Resource>,
    r1: crossbeam_channel::Receiver<v1::Resource>,
}

pub enum Action{
    Add(ResourceKeyReferences),
    Get(v1::Key),
}

pub enum ResourceKeyReferences{
    VirtualNetwork(v1alpha1::VirtualNetwork, v1::Key, Vec<v1alpha1::ResourceReference>),
    VirtualMachine(v1alpha1::VirtualMachine, v1::Key, Vec<v1alpha1::ResourceReference>),
    VirtualMachineInterface(v1alpha1::VirtualMachineInterface, v1::Key, Vec<v1alpha1::ResourceReference>),
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct CacheKey {
    pub kind: String,
    pub namespace: String,
    pub name: String,
}

pub fn to_cache_key(key: v1::Key) -> CacheKey {
    CacheKey { 
        kind: key.kind, 
        namespace: key.namespace,
        name: key.name,
    }
}

impl Cache {
    pub fn new() -> Self {
        let (sender, receiver): (crossbeam_channel::Sender<Action>, crossbeam_channel::Receiver<Action>) = crossbeam_channel::unbounded();
        let (s1, r1): (crossbeam_channel::Sender<v1::Resource>, crossbeam_channel::Receiver<v1::Resource>) = crossbeam_channel::unbounded();
        Self{
            receiver: receiver,
            sender: sender,
            s1: s1,
            r1: r1,
        }
    }

    pub fn get(&self, key: v1::Key) -> v1::Resource{
        self.sender.send(Action::Get(key));
        let reply = self.r1.recv().unwrap();
        reply
    }

    pub fn add(&self, mut resource: ResourceKeyReferences) {
        self.sender.send(Action::Add(resource));
    }

    fn update_graph(&self, key: v1::Key, references: Vec<v1alpha1::ResourceReference>, mut g: Graph){
        let result = g.add_node(to_cache_key(key.clone()), to_cache_key(key.clone()));
        if references.len() > 0 {
            for reference in &references {
                g.add_edge(to_cache_key(key.clone()), CacheKey{
                    name: reference.object_reference.as_ref().unwrap().name.as_ref().unwrap().to_string(),
                    namespace: reference.object_reference.as_ref().unwrap().namespace.as_ref().unwrap().to_string(),
                    kind: reference.object_reference.as_ref().unwrap().kind.as_ref().unwrap().to_string(),
                });
            }
        }
        match result{
            None => {
                println!("add");
            },
            _ => {
                println!("update");
            },
        }
        g.print();  
    }    

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error + Send>> {
        let mut virtual_network_cache: LruCache<CacheKey, v1alpha1::VirtualNetwork> = LruCache::unbounded();
        let mut virtual_machine_cache: LruCache<CacheKey, v1alpha1::VirtualMachine> = LruCache::unbounded();
        let mut virtual_machine_interface_cache: LruCache<CacheKey, v1alpha1::VirtualMachineInterface> = LruCache::unbounded();
        println!("starting cache");
        let g = Graph::new();
        loop{
            let action = self.receiver.recv().unwrap();
            match action {
                Action::Add(resource_key) => {
                    println!("adding resource to cache");
                    match resource_key {
                        ResourceKeyReferences::VirtualNetwork(resource, key, references ) => {
                            virtual_network_cache.push(to_cache_key(key.clone()), resource.clone());
                            self.update_graph(key.clone(), references, g.clone());
                        },
                        ResourceKeyReferences::VirtualMachine(resource, key, references) => {
                            virtual_machine_cache.push(to_cache_key(key.clone()), resource.clone());
                            self.update_graph(key.clone(), references, g.clone());
                        },
                        ResourceKeyReferences::VirtualMachineInterface(resource, key, references ) => {
                            virtual_machine_interface_cache.push(to_cache_key(key.clone()), resource.clone());
                            self.update_graph(key.clone(), references, g.clone());
                        },
                    }

                },
                Action::Get(key)=> {
                    println!("getting resource from cache");
                    println!("{:?}", key);
                    match key.kind.as_str(){
                        "VirtualNetwork" => {
                            let resource = virtual_network_cache.get(&to_cache_key(key)).unwrap();
                            let res = v1::Resource{
                                resource: Some(v1::resource::Resource::VirtualNetwork(resource.clone())),
                            };
                            self.s1.send(res);
                        },
                        "VirtualMachine" => {
                            let resource = virtual_machine_cache.get(&to_cache_key(key)).unwrap();
                            let res = v1::Resource{
                                resource: Some(v1::resource::Resource::VirtualMachine(resource.clone())),
                            };
                            self.s1.send(res);
                        },
                        "VirtualMachineInterface" => {
                            let resource = virtual_machine_interface_cache.get(&to_cache_key(key)).unwrap();
                            let res = v1::Resource{
                                resource: Some(v1::resource::Resource::VirtualMachineInterface(resource.clone())),
                            };
                            self.s1.send(res);
                        },
                        _ => {},
                    }
                },
                _ => { break; },
            }
        }
        
        Ok(())
    }
}