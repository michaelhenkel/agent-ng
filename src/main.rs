mod config_controller;
mod config;
mod cache_controller;
use crossbeam_channel::unbounded;
use agent_ng::protos::github::com::michaelhenkel::config_controller::pkg::apis::v1;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::config::load().unwrap();

    let (sender, receiver): (crossbeam_channel::Sender<v1::Resource>, crossbeam_channel::Receiver<v1::Resource>) = unbounded();
    
    let cache = cache_controller::cache::cache::new(receiver);
    let cache_runner = cache.run();

    let cc = config_controller::config_controller::start(config.name.unwrap(), config.config_controller.unwrap(), sender);
    
    let (config_res, cache_res) = tokio::join!(cc, cache_runner);
    //futures::future::join_all(cc);
    
    //let join_handle = tokio::task::spawn(cc);
    //join_handle.await?;
    Ok(())
}



