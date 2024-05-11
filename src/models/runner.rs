use tokio_i3ipc::I3;

pub struct Runner;
use tracing::{debug, error, info};

impl Runner {
    pub async fn execute(commands: Box<[&str]>){
        info!("execute {:?}", commands);
        let mut ipc = I3::connect().await.unwrap();
        for command in commands.into_iter(){
            match ipc.run_command(command).await{
                Ok(result) => debug!("{:?}", result),
                Err(e) => error!("{:?}", e),
            }
        }
    }
}
