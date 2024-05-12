use swayipc_async::Connection;

pub struct Runner;
use tracing::{debug, error, info};

impl Runner {
    pub async fn execute(commands: Vec<&str>){
        info!("execute {:?}", commands);
        let mut ipc = Connection::new().await.unwrap();
        for command in commands.as_slice(){
            match ipc.run_command(command).await{
                Ok(result) => debug!("{:?}", result),
                Err(e) => error!("{:?}", e),
            }
        }
    }
    pub async fn execute_one(command: String){
        info!("execute {:?}", command);
        let mut ipc = Connection::new().await.unwrap();
        match ipc.run_command(command).await{
            Ok(result) => debug!("{:?}", result),
            Err(e) => error!("{:?}", e),
        }
    }
}
