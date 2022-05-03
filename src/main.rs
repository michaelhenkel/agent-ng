mod config_controller;
mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::config::load().unwrap();
    let cc = config_controller::config_controller::start(config.name.unwrap(), config.config_controller.unwrap());
    let join_handle = tokio::task::spawn(cc);
    join_handle.await?;
    Ok(())
}



