use tokio_i3ipc::I3;

pub struct Runner;
use tracing::{debug, error, info};

impl Runner {
    pub async fn execute(command: &str){
        info!("execute {command}");
        match I3::connect()
            .await
            .unwrap()
            .run_command(command)
            .await{
            Ok(result) => debug!("{:?}", result),
            Err(e) => error!("{:?}", e),
        }
    }
}
