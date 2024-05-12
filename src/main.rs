mod models;
mod plugins;

use std::{env, str::FromStr, process};
use clap::{Arg, ArgAction, Command};
use single_instance::SingleInstance;
use tokio_stream::StreamExt;

use tokio::{
    select,
    signal::unix::{
        signal,
        SignalKind,
    },
};
use swayipc_async::{Connection, EventType, Fallible};
use tracing::{debug, error, info};
use tracing_subscriber::{
    EnvFilter,
    layer::SubscriberExt,
    util::SubscriberInitExt,
};
use models::Config;
use plugins::{
    Plugin,
    PluginTrait,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

#[tokio::main]
async fn main() -> Fallible<()> {
    info!("main");
    let log_level = env::var("RUST_LOG").unwrap_or("DEBUG".to_string());
    debug!("log_level: {}", log_level);

    tracing_subscriber::registry()
        .with(EnvFilter::from_str(&log_level).unwrap())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let instance = SingleInstance::new("I3Helper").unwrap();
    if !instance.is_single() {
        error!("Another instance is running");
        process::exit(1);
    }

    let cmd = Command::new(NAME)
        .author(AUTHORS)
        .version(VERSION)
        .about(DESCRIPTION)
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .action(ArgAction::Set)
                .value_name("FILE")
        )
        .get_matches();
    debug!("{:?}", cmd);

    let config_file = if let Some(config_file) = cmd.get_one::<String>("config"){
        config_file
    }else{
        "config.yml"
    };
    let config = Config::read_configuration(config_file).await;
    let config2 = config.clone();

    tokio::spawn(async move {
        let mut sigterm = signal(SignalKind::terminate()).unwrap();
        let mut sigint = signal(SignalKind::interrupt()).unwrap();
            select! {
                _ = sigterm.recv() => {
                    debug!("Recieve SIGTERM");
                    end(&config2).await;
                    process::exit(0);
                },
                _ = sigint.recv() => {
                    debug!("Recieve SIGINT");
                    end(&config2).await;
                    process::exit(0);
                },
            };
    });


    let plugins = Plugin::get_plugins(&config);

    // Start
    for plugin in plugins.iter() {
        plugin.start().await;
    }

    let mut events = Connection::new().await?.subscribe([EventType::Window]).await?;

    while let Some(result_event) = events.next().await {
        if let Ok(event) = result_event { 
            // Process
            for plugin in plugins.iter() {
                plugin.process(&event).await;
            }
        }
    }
    Ok(())
}

async fn end(config: &Config){
    debug!("end: {:?}", config);

    for plugin in Plugin::get_plugins(config).iter() {
        plugin.end().await;
    }
}
