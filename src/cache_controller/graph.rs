use super::cache::Key;
use std::collections::{HashMap, VecDeque};
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
    pub fn traverse(&mut self, from: Key, to: Key, filter: Vec<String>) -> Vec<Key>{
        let mut result: Vec<Key> = Vec::new();
        let mut queue = VecDeque::new();
        if self.nodes.contains_key(&from) {
            queue.push_back(from);
            let mut visited: HashMap<Key, bool> = HashMap::new();
            loop {
                if queue.len() == 0 {
                    break;
                }
                let key = queue.pop_front().unwrap();
                visited.insert(key.clone(), true);
                if key.kind != to.kind {
                    let near = self.edges.get(&key).unwrap();
                    for j in near{
                        let mut ignore = false;
                        if !filter.contains(&j.kind) {
                            ignore = true;
                        }
                        let j_visited = visited.get(j);
                        match j_visited{
                            Some(true) => {
                                if !ignore{
                                    queue.push_back(j.clone());
                                    visited.insert(j.clone(), true);
                                }
                            },
                            _ => {},
                        }
                    }
                }
                if key.kind == to.kind {
                    println!("found node {:?}", key);
                    result.push(key.clone());
                }
            }
        }
        result
    }
}
