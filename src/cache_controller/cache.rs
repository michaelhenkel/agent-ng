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
    get_sender: crossbeam_channel::Sender<v1::Resource>,
    get_receiver: crossbeam_channel::Receiver<v1::Resource>,
    list_sender: crossbeam_channel::Sender<v1::ResourceList>,
    list_receiver: crossbeam_channel::Receiver<v1::ResourceList>,
    result_sender: crossbeam_channel::Sender<Result<(), NodeNotExist>>,
    result_receiver: crossbeam_channel::Receiver<Result<(), NodeNotExist>>,
}

pub enum Action{
    Add(ResourceKeyReferences),
    Get(v1::Key),
    List(v1::FromToFilter),
}

pub enum ResourceKeyReferences{
    VirtualNetwork(v1alpha1::VirtualNetwork, v1::Key, Vec<v1alpha1::ResourceReference>),
    VirtualMachine(v1alpha1::VirtualMachine, v1::Key, Vec<v1alpha1::ResourceReference>),
    VirtualMachineInterface(v1alpha1::VirtualMachineInterface, v1::Key, Vec<v1alpha1::ResourceReference>),
}

#[derive(PartialEq, Eq, Hash, Clone, Debug, Default)]
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

pub fn to_v1_key(cache_key: CacheKey) -> v1::Key {
    v1::Key { 
        kind: cache_key.kind, 
        namespace: cache_key.namespace,
        name: cache_key.name,
    }
}

pub fn reference_to_cache_key(reference: v1alpha1::ResourceReference) -> CacheKey{
    CacheKey { 
        kind: reference.object_reference.as_ref().unwrap().kind.as_ref().unwrap().to_string(),
        namespace: reference.object_reference.as_ref().unwrap().namespace.as_ref().unwrap().to_string(),
        name: reference.object_reference.as_ref().unwrap().name.as_ref().unwrap().to_string(),
    }
}

struct NodeNotExist;
pub struct AddError;

fn in_resource_list(reference: v1alpha1::ResourceReference) -> bool {
    let reference_list = vec!["VirtualNetwork".to_string(),"VirtualMachine".to_string(),"VirtualMachineInterface".to_string()];
    let ref_kind = reference.object_reference.as_ref().unwrap().kind.as_ref().unwrap();
    reference_list.contains(ref_kind)
}

impl Cache {
    pub fn new() -> Self {
        let (sender, receiver): (crossbeam_channel::Sender<Action>, crossbeam_channel::Receiver<Action>) = crossbeam_channel::unbounded();
        let (get_sender, get_receiver): (crossbeam_channel::Sender<v1::Resource>, crossbeam_channel::Receiver<v1::Resource>) = crossbeam_channel::unbounded();
        let (list_sender, list_receiver): (crossbeam_channel::Sender<v1::ResourceList>, crossbeam_channel::Receiver<v1::ResourceList>) = crossbeam_channel::unbounded();
        let (result_sender, result_receiver): (crossbeam_channel::Sender<Result<(), NodeNotExist>>, crossbeam_channel::Receiver<Result<(), NodeNotExist>>) = crossbeam_channel::unbounded();
        Self{
            receiver,
            sender,
            get_sender,
            get_receiver,
            list_sender,
            list_receiver,
            result_sender,
            result_receiver,
        }
    }

    pub fn get(&self, key: v1::Key) -> v1::Resource{
        self.sender.send(Action::Get(key));
        let reply = self.get_receiver.recv().unwrap();
        reply
    }

    pub fn find(&self, from_to_filter: v1::FromToFilter) -> v1::ResourceList{
        self.sender.send(Action::List(from_to_filter));
        let reply = self.list_receiver.recv().unwrap();
        reply
    }

    pub fn add(&self, mut resource: ResourceKeyReferences) -> Result<(), AddError>{
        self.sender.send(Action::Add(resource));
        let result = self.result_receiver.recv().unwrap();
        match result{
            Ok(()) => {
                return Ok(());
            },
            Err(e) => { 
                return Err(AddError);
            },
        };
    }

    fn update_graph(&self, key: v1::Key, references: Vec<v1alpha1::ResourceReference>, g: &mut Graph) -> Result<(), NodeNotExist> {
        let result = g.add_node(to_cache_key(key.clone()), to_cache_key(key.clone()));
        if references.len() > 0 {
/*
            for reference in &references {
                if reference.object_reference.as_ref().unwrap().kind.as_ref().unwrap().to_string() == "VirtualMachine".to_string(){
                    println!("VM");
                }
                if in_resource_list(reference.clone()) && !g.contains_node(reference_to_cache_key(reference.clone())){
                    return Err(NodeNotExist)
                }
            };
*/
            for reference in &references {
                g.add_edge(CacheKey{
                    name: reference.object_reference.as_ref().unwrap().name.as_ref().unwrap().to_string(),
                    namespace: reference.object_reference.as_ref().unwrap().namespace.as_ref().unwrap().to_string(),
                    kind: reference.object_reference.as_ref().unwrap().kind.as_ref().unwrap().to_string(),
                }, to_cache_key(key.clone()));
            }

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
        Ok(())
    }    

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error + Send>> {
        let mut virtual_network_cache: LruCache<CacheKey, v1alpha1::VirtualNetwork> = LruCache::unbounded();
        let mut virtual_machine_cache: LruCache<CacheKey, v1alpha1::VirtualMachine> = LruCache::unbounded();
        let mut virtual_machine_interface_cache: LruCache<CacheKey, v1alpha1::VirtualMachineInterface> = LruCache::unbounded();
        println!("starting cache");
        let mut graph = Graph::new();
        loop{
            let action = self.receiver.recv().unwrap();
            match action {
                Action::Add(resource_key) => {
                    println!("adding resource to cache");
                    match resource_key {
                        ResourceKeyReferences::VirtualNetwork(resource, key, references ) => {
                            virtual_network_cache.push(to_cache_key(key.clone()), resource.clone());
                            let res = self.update_graph(key.clone(), references, &mut graph);
                            self.result_sender.send(res);
                        },
                        ResourceKeyReferences::VirtualMachine(resource, key, references) => {
                            virtual_machine_cache.push(to_cache_key(key.clone()), resource.clone());
                            let res = self.update_graph(key.clone(), references, &mut graph);
                            self.result_sender.send(res);
                        },
                        ResourceKeyReferences::VirtualMachineInterface(resource, key, references ) => {
                            virtual_machine_interface_cache.push(to_cache_key(key.clone()), resource.clone());
                            let res = self.update_graph(key.clone(), references, &mut graph);
                            self.result_sender.send(res);
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
                            self.get_sender.send(res);
                        },
                        "VirtualMachine" => {
                            let resource = virtual_machine_cache.get(&to_cache_key(key)).unwrap();
                            let res = v1::Resource{
                                resource: Some(v1::resource::Resource::VirtualMachine(resource.clone())),
                            };
                            self.get_sender.send(res);
                        },
                        "VirtualMachineInterface" => {
                            let resource = virtual_machine_interface_cache.get(&to_cache_key(key)).unwrap();
                            let res = v1::Resource{
                                resource: Some(v1::resource::Resource::VirtualMachineInterface(resource.clone())),
                            };
                            self.get_sender.send(res);
                        },
                        _ => {},
                    }
                },
                Action::List(from_to_filter)=> {
                    println!("finding resource from graph");
                    let from_key = to_cache_key(from_to_filter.clone().from.unwrap());
                    let to_key = to_cache_key(from_to_filter.clone().to.unwrap());
                    let filter = from_to_filter.clone().filter.clone();
                    let result = graph.clone().traverse(from_key, to_key, filter);
                    let mut resource_list: Vec<v1::Resource> = Vec::new();
                    for key in result{
                        match key.kind.as_str(){
                            "VirtualNetwork" => {
                                let resource = virtual_network_cache.get(&key.clone()).unwrap();
                                let res = v1::Resource{
                                    resource: Some(v1::resource::Resource::VirtualNetwork(resource.clone())),
                                };
                                resource_list.push(res.clone());
                            },
                            "VirtualMachine" => {
                                let resource = virtual_machine_cache.get(&key.clone()).unwrap();
                                let res = v1::Resource{
                                    resource: Some(v1::resource::Resource::VirtualMachine(resource.clone())),
                                };
                                resource_list.push(res.clone());
                            },
                            "VirtualMachineInterface" => {
                                let resource = virtual_machine_interface_cache.get(&key.clone()).unwrap();
                                let res = v1::Resource{
                                    resource: Some(v1::resource::Resource::VirtualMachineInterface(resource.clone())),
                                };
                                resource_list.push(res.clone());
                            },
                            _ => {},
                        }
                    }
                    let resources = v1::ResourceList{
                        resources: resource_list,
                    };
                    self.list_sender.send(resources.clone());
                },
                _ => { break; },
            }
            println!("current graph: ");
            graph.print();
        }
        
        Ok(())
    }
}