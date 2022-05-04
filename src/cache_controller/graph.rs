use super::cache::Key;
use std::collections::HashMap;
use agent_ng::protos::github::com::michaelhenkel::config_controller::pkg::apis::v1;


pub struct Graph{
    nodes: HashMap<Key, v1::Resource>,
    edges: HashMap<Key, Vec<Key>>,
}

impl Graph{
    pub fn new() -> Self{
        let edges: HashMap<Key, Vec<Key>> = HashMap::new();
        Self{
            nodes: HashMap::new(),
            edges: edges,
        }
    }

    pub fn add_node(&mut self, key: Key, node: v1::Resource) -> Option<v1::Resource> {
        self.nodes.insert(key, node)
    }
    /*
    pub fn get_node(&self, node: Key) -> Key{
        node
    }
    */
    pub fn print(&self) {
        println!("nodes: {:?}", self.nodes);
        println!("edges: {:?}", self.edges);
    }
    pub fn add_edge(&mut self, n1: Key, n2: Key){
        match self.edges.get_mut(&n1){
            None => {
                let mut edge_refs: Vec<Key> = Vec::new();
                edge_refs.push(n2);
                self.edges.insert(n1, edge_refs);
            },
            Some(edge_refs) => {
                if !edge_refs.contains(&n2) {
                    edge_refs.push(n2);
                }
            },
        }
    }
    
}
