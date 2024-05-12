use tracing::debug;
use serde::Deserialize;
use std::collections::HashMap;
use swayipc_async::{
    Event,
    WindowChange,
};

use super::{
    super::models::{
        Config,
        Parts,
        Root,
        Runner,
    },
    plugin::PluginTrait
};

#[derive(Debug, Deserialize, Clone)]
pub struct Autonaming{
    pub enable: bool,
    pub icons: HashMap<String, String>,
    pub duplicates: bool,
}

impl Autonaming {
    async fn do_work(&self) {
        let default = self.icons.get("default").expect("Default icon not defined").to_string();
        let workspaces = Root::default().await.get_workspaces();
        for workspace in workspaces{
            let name = workspace.get_name();
            let mut parts: Parts = name.into();
            let mut new_icons = Vec::new();
            debug!("Workspace: {}. Num: {}", workspace.get_id(), workspace.get_num());
            let names = workspace.get_apps();
            for name in names{
                let icon = self.icons.get(&name).unwrap_or(&default);
                debug!("name: {} => icon: {}", name, icon);
                if self.duplicates || !new_icons.contains(icon){
                    new_icons.push(icon.to_string());
                }
            }
            parts.icons = Some(new_icons.join(" "));
            let new_name: String = parts.into();
            let command = format!(r#"rename workspace "{}" to "{}""#, 
                workspace.get_name(), new_name);
            Runner::execute_one(command).await;
        }
    }
}

impl Autonaming {
    pub fn new(config: &Config) -> Self{
        debug!("new: {:?}", config);
        config.autonaming.clone()
    }

}

impl PluginTrait for Autonaming {

    async fn start(&self) {
        debug!("start");
        if self.enable {
            self.do_work().await;
        }
    }

    async fn process(&self, event: &Event) {
        debug!("process: {:?}", event);
        if self.enable {
            if let Event::Window(window_event) = event{
                if window_event.change == WindowChange::New || 
                        window_event.change == WindowChange::Close || 
                        window_event.change == WindowChange::Move {
                    self.do_work().await;
                }
            }
        }
    }
    async fn end(&self) {
        debug!("undo");
        if self.enable {
            let workspaces = Root::default().await.get_workspaces();
            for workspace in workspaces{
                let workspace_name = workspace.get_name();
                let mut parts: Parts = workspace_name.clone().into();
                parts.icons = None;
                let new_name: String = parts.into();
                let command = format!(r#"rename workspace "{}" to "{}""#,
                    &workspace_name,
                    new_name);
                Runner::execute_one(command).await;
            }
        }
    }
}
