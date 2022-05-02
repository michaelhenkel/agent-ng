mod resources;
mod config_controller;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("starting agent");
    let cc = config_controller::config_controller::start();
    let join_handle = tokio::task::spawn(cc);
    join_handle.await?;
    //futures::future::join_all(join_handle).await;
    //let res = join_handle.await?;
    Ok(())
}