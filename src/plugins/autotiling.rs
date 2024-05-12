use tracing::debug;
use tokio_i3ipc::event::{
    Event,
    WindowChange,
};
use serde::Deserialize;
use super::{
    super::models::{
        Config,
        Root,
        Runner,
    },
    plugin::PluginTrait
};

#[derive(Debug, Deserialize, Clone)]
pub struct Autotiling{
    pub enable: bool,
}

impl Autotiling {

    pub fn new(config: &Config) -> Self{
        debug!("new: {:?}", config);
        config.autotiling.clone()
    }
}


impl PluginTrait for Autotiling {

    async fn start(&self) {
        debug!("start");
    }

    async fn process(&self, event: &tokio_i3ipc::event::Event) {
        debug!("process: {:?}", event);
        if self.enable {
            if let Event::Window(window_event) = event{
                if window_event.change == WindowChange::New || 
                        window_event.change == WindowChange::Close || 
                        window_event.change == WindowChange::Move {
                    if let Some(workspace) = Root::default().await.get_workspace(&window_event.container) {
                        if let Some(focused) = workspace.get_focused() {
                            debug!("Focused: {:?}", focused);
                            if focused.rect.height > focused.rect.width{
                                Runner::execute_one("splitv".to_string()).await;
                            }else{
                                Runner::execute_one("splith".to_string()).await;
                            }
                        }
                    }
                }
            }
        }
    }

    async fn end(&self) {
        debug!("undo");
    }
}
