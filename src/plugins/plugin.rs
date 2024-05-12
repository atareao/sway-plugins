use tracing::debug;
use tokio_i3ipc::event::Event;

use super::{Autonaming, Autotiling, Autotransparency};
use super::super::models::Config;

pub trait PluginTrait {

    async fn start(&self);

    async fn process(&self, event: &Event);

    async fn end(&self);
}

pub enum Plugin {
    Autonaming(Autonaming),
    Autotiling(Autotiling),
    Autotransparency(Autotransparency),
}

impl Plugin {
    pub fn get_plugins(config: &Config) -> Vec<Plugin>{
        debug!("get_plugins: {:?}", config);
        vec![
            Plugin::Autotiling(Autotiling::new(config)),
            Plugin::Autonaming(Autonaming::new(config)),
            Plugin::Autotransparency(Autotransparency::new(config)),
        ]
    }
}

impl PluginTrait for Plugin {

    async fn start(&self) {
        match self{
            Plugin::Autonaming(plugin) => plugin.start().await,
            Plugin::Autotiling(plugin) => plugin.start().await,
            Plugin::Autotransparency(plugin) => plugin.start().await,
        }
    }

    async fn process(&self, event: &tokio_i3ipc::event::Event) {
        match self{
            Plugin::Autonaming(plugin) => plugin.process(event).await,
            Plugin::Autotiling(plugin) => plugin.process(event).await,
            Plugin::Autotransparency(plugin) => plugin.process(event).await,
        }
    }

    async fn end(&self) {
        match self{
            Plugin::Autonaming(plugin) => plugin.end().await,
            Plugin::Autotiling(plugin) => plugin.end().await,
            Plugin::Autotransparency(plugin) => plugin.end().await,
        }
    }
}


