use tracing::debug;
use swayipc_async::{
    Event,
    WindowChange,
};
use serde::Deserialize;
use super::{
    super::models::{
        Config,
        Runner,
    },
    plugin::PluginTrait
};


#[derive(Debug, Deserialize, Clone)]
pub struct Autotransparency{
    pub enable: bool,
    pub transparency: f32,
}

impl Autotransparency {
    pub fn new(config: &Config) -> Self{
        debug!("new: {:?}", config);
        config.autotransparency.clone()

    }

    async fn do_work(&self) {
        debug!("do_work");
        let command = format!(r#"[con_mark="current"] opacity {}"#,
            self.transparency);
        let commands = vec![
            &command,
            r#"[con_mark="current"] mark transparency"#,
            r#"[con_mark="current"] unmark current"#,
            r#"opacity 1"#,
            r#"mark current"#,

        ];
        Runner::execute(commands).await;

    }
}

impl PluginTrait for Autotransparency {

    async fn start(&self) {
        debug!("start");
    }

    async fn process(&self, event: &Event) {
        debug!("process: {:?}", event);
        if self.enable {
            if let Event::Window(window_event) = event{
                if window_event.change == WindowChange::Focus {
                    self.do_work().await;
                }
            }
        }
    }

    async fn end(&self) {
        debug!("undo");
        if self.enable {
            let commands = vec![
                r#"[con_mark="transparency"] opacity 1"#,
                r#"[con_mark="current"] unmark transparency"#,
            ];
            Runner::execute(commands).await;
        }
    }
}
