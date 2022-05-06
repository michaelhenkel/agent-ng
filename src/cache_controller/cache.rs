extern crate lru;

use lru::LruCache;
use super::graph::Graph;
use std::hash::{Hash, Hasher};
use agent_ng::protos::github::com::michaelhenkel::config_controller::pkg::apis::v1;

#[derive(Clone, Debug)]
pub struct Cache {
    receiver: crossbeam_channel::Receiver<v1::Resource>,
    sender: crossbeam_channel::Sender<v1::Resource>,
    s1: crossbeam_channel::Sender<v1::Resource>,
    r1: crossbeam_channel::Receiver<v1::Resource>,
}

pub enum Action{
    Add(v1::Resource),
    Get(Key),
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Key {
    pub name: String,
    pub namespace: String,
    pub kind: String,
}

impl Key {
    fn create(resource: v1::Resource) -> Self {
        Self{
            name: resource.name,
            namespace: resource.namespace,
            kind: resource.kind,
        }
    }
}

impl Cache {
    pub fn new() -> Self {
        let (sender, receiver): (crossbeam_channel::Sender<v1::Resource>, crossbeam_channel::Receiver<v1::Resource>) = crossbeam_channel::unbounded();
        let (s1, r1): (crossbeam_channel::Sender<v1::Resource>, crossbeam_channel::Receiver<v1::Resource>) = crossbeam_channel::unbounded();
        Self{
            receiver: receiver,
            sender: sender,
            s1: s1,
            r1: r1,
        }
    }

    pub fn get(&self, key: Key) -> v1::Resource{
        let mut res = v1::Resource::default();
        res.action = i32::from(v1::resource::Action::Get);
        self.sender.send(res);
        let reply = self.r1.recv().unwrap();
        reply
    }

    pub fn add(&self, mut resource: v1::Resource) {
        resource.action = i32::from(v1::resource::Action::Add);
        self.sender.send(resource);
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error + Send>> {
        let mut resource_cache: LruCache<Key, v1::Resource> = LruCache::unbounded();
        println!("starting cache");
        let mut g = Graph::new();
        loop{
            let resource = self.receiver.recv().unwrap();
            match v1::resource::Action::from_i32(resource.action){
                Some(v1::resource::Action::Add) => {
                    resource_cache.push(Key::create(resource.clone()), resource.clone());
                    let result = g.add_node(Key::create(resource.clone()), resource.clone());
                    if resource.references.len() > 0 {
                        for reference in &resource.references {
                            g.add_edge(Key::create(resource.clone()), Key{
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
                },
                Some(v1::resource::Action::Del) => {
                    println!("del");
                },
                Some(v1::resource::Action::Retry) => {
                    println!("retry");
                },
                Some(v1::resource::Action::Get) => {
                    println!("get");
                    /*
                    let from = Key { name: "".to_string(), namespace: "".to_string(), kind: "".to_string() };
                    let to = Key { name: "".to_string(), namespace: "".to_string(), kind: "".to_string() };
                    let mut filter = Vec::new();
                    filter.push("bla".to_string());
                    let mut result = g.traverse(from, to, filter);
                    for res in &result {
                        let resource = resource_cache.get(res).unwrap();
                        self.s1.send(resource.clone());
                    }
                    */
                    let resource = resource_cache.get(res).unwrap();
                },
                _ => { break; },
            }
        }
        
        Ok(())
    }
}