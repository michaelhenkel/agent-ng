extern crate lru;

use lru::LruCache;
use agent_ng::protos::github::com::michaelhenkel::config_controller::pkg::apis::v1;

pub struct cache {
    receiver: crossbeam_channel::Receiver<v1::Resource>,
}


impl cache {
    pub fn new(receiver: crossbeam_channel::Receiver<v1::Resource>) -> Self {
        Self{
            receiver: receiver,
        }
    }
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error + Send>> {
        println!("starting cache");
        loop{
            let resource = self.receiver.recv().unwrap();
            match v1::resource::Action::from_i32(resource.action){
                Some(v1::resource::Action::Add) => {
                    println!("add");
                },
                Some(v1::resource::Action::Del) => {
                    println!("del");
                },
                Some(v1::resource::Action::Retry) => {
                    println!("retry");
                },
                _ => { break; },
            }
        }
        Ok(())
    }
}