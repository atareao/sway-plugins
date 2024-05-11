use tokio_i3ipc::I3;

pub struct Runner;
use tracing::{debug, error, info};

impl Runner {
    pub async fn execute(commands: Vec<&str>){
        info!("execute {:?}", commands);
        let mut ipc = I3::connect().await.unwrap();
        for command in commands.as_slice(){
            match ipc.run_command(command).await{
                Ok(result) => debug!("{:?}", result),
                Err(e) => error!("{:?}", e),
            }
        }
    }
    pub async fn execute_one(command: String){
        info!("execute {:?}", command);
        let mut ipc = I3::connect().await.unwrap();
        match ipc.run_command(command).await{
            Ok(result) => debug!("{:?}", result),
            Err(e) => error!("{:?}", e),
        }
    }
}
