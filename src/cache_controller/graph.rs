use std::collections::{HashMap, VecDeque};
use agent_ng::protos::github::com::michaelhenkel::config_controller::pkg::apis::v1;
use super::cache::CacheKey;


#[derive(Clone, Debug)]
pub struct Graph{
    nodes: HashMap<CacheKey, CacheKey>,
    edges: HashMap<CacheKey, Vec<CacheKey>>,
}


impl Graph{
    pub fn new() -> Self{
        let edges: HashMap<CacheKey, Vec<CacheKey>> = HashMap::new();
        Self{
            nodes: HashMap::new(),
            edges: edges,
        }
    }

    pub fn add_node(&mut self, key: CacheKey, node: CacheKey) -> Option<CacheKey> {
        self.nodes.insert(key, node)
    }

    pub fn contains_node(&mut self, key: CacheKey) -> bool {
        self.nodes.contains_key(&key)
    }

    pub fn print(&self) {
        println!("nodes: {:?}", self.nodes);
        println!("edges: {:?}", self.edges);
    }
    pub fn add_edge(&mut self, n1: CacheKey, n2: CacheKey){
        match self.edges.get_mut(&n1){
            None => {
                let mut edge_refs: Vec<CacheKey> = Vec::new();
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
    pub fn traverse(&mut self, from: CacheKey, to: CacheKey, filter: Vec<String>) -> Vec<CacheKey>{
        let mut result: Vec<CacheKey> = Vec::new();
        let mut queue = VecDeque::new();
        println!("traversing graph from {:?} to {:?} with filter {:?}", from, to, filter);
        if self.nodes.contains_key(&from) {
            println!("found root node {:?}", from);
            queue.push_back(from);
            let mut visited: HashMap<CacheKey, bool> = HashMap::new();
            loop {
                if queue.len() == 0 {
                    break;
                }
                let key = queue.pop_front().unwrap();
                visited.insert(key.clone(), true);
                if key.kind != to.kind {
                    let near_option = self.edges.get(&key);
                    match near_option{
                        Some(near) => {
                            for j in near{
                                let mut ignore = false;
                                if !filter.contains(&j.kind) {
                                    ignore = true;
                                }
                                let j_visited = visited.get(j);
                                match j_visited{
                                    Some(true) => {

                                    },
                                    _ => {
                                        if !ignore{
                                            queue.push_back(j.clone());
                                            visited.insert(j.clone(), true);
                                        }
                                    },
                                }
                            }
                        },
                        None => {},
                    };
                    //let near = self.edges.get(&key).unwrap();
                }
                if key.kind == to.kind {
                    println!("found node {:?}", key);
                    result.push(key.clone());
                }
            }
        } else {
            println!("didn't find root node {:?} {:?}", from, self.nodes.len());
        }
        result
    }
}
