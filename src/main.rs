mod models;

use std::{io, env, str::FromStr, process};
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
use tokio_i3ipc::{
    event::{
        Event,
        WindowChange,
        Subscribe
    },
    I3,
    reply::Node,
};
use tracing::{debug, error, info};
use tracing_subscriber::{
    EnvFilter,
    layer::SubscriberExt,
    util::SubscriberInitExt,
};
use models::{
    Parts,
    Config, Root, Runner,
};

#[tokio::main]
async fn main() -> io::Result<()> {
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

    let cmd = Command::new("i3helper")
        .author("Lorenzo Carbonell <a.k.a. atareao>")
        .version("1.0.0")
        .about("i3helper")
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


    
    tokio::spawn(async move {
        let mut sigterm = signal(SignalKind::terminate()).unwrap();
        let mut sigint = signal(SignalKind::interrupt()).unwrap();
            select! {
                _ = sigterm.recv() => {
                    debug!("Recieve SIGTERM");
                    undo().await;
                    process::exit(0);
                },
                _ = sigint.recv() => {
                    debug!("Recieve SIGINT");
                    undo().await;
                    process::exit(0);
                },
            };
    });
    autonaming(&config).await;

    let mut i3 = I3::connect().await?;
    let _ = i3.subscribe([Subscribe::Window]).await;
    let mut listener = i3.listen();
    while let Some(event) = listener.next().await {
        if let Ok(Event::Window(window_event)) = event{
            match window_event.change {
                WindowChange::New | WindowChange::Close | WindowChange::Move => {
                    debug!(" New Event {:?}", &window_event);
                    autonaming(&config).await;
                    if window_event.change == WindowChange::New{
                        debug!(" New Event {:?}", &window_event);
                        if config.autotiling {
                            autotiling(&window_event.container).await;
                        }
                    }
                },
                WindowChange::Focus => {
                    debug!("Focus: {:?}", &window_event);
                    if config.autotransparency {
                        autotransparency(&window_event.container,
                            config.transparency).await;
                    }
                },
                _ => debug!("Unknown {:?}", &window_event),
            }
        }else{
            error!("Nada");
        }
    }
    Ok(())
}

async fn undo(){
    undo_autonaming().await;
    undo_autotransparency().await;
}

async fn autonaming(config: &Config){
    if ! config.autonaming {
        return;
    }
    let icons = &config.icons;
    let default = &config.icons.get("default").expect("Default icon not defined").to_string();
    let duplicates = config.duplicates;
    let workspaces = Root::default().await.get_workspaces();
    for workspace in workspaces{
        let name = workspace.get_name();
        let mut parts: Parts = name.into();
        let mut new_icons = Vec::new();
        debug!("Workspace: {}. Num: {}", workspace.get_id(), workspace.get_num());
        let names = workspace.get_apps();
        for name in names{
            let icon = icons.get(&name).unwrap_or(default);
            debug!("name: {} => icon: {}", name, icon);
            if duplicates || !new_icons.contains(icon){
                new_icons.push(icon.to_string());
            }
        }
        parts.icons = Some(new_icons.join(" "));
        let new_name: String = parts.into();
        let command = format!(r#"rename workspace "{}" to "{}""#, 
            workspace.get_name(), new_name);
        Runner::execute(&command).await;
    }
}

async fn undo_autonaming(){
    let workspaces = Root::default().await.get_workspaces();
    for workspace in workspaces{
        let workspace_name = workspace.get_name();
        let mut parts: Parts = workspace_name.clone().into();
        parts.icons = None;
        let new_name: String = parts.into();
        let command = format!(r#"rename workspace "{}" to "{}""#,
            &workspace_name,
            new_name);
        Runner::execute(&command).await;
    }
}

async fn autotransparency(node: &Node, transparency: f32) {
    debug!("autotransparency");
    let command = format!(r#"[con_mark="current"] opacity {}"#, transparency);
    Runner::execute(&command).await;
    Runner::execute(r#"[con_mark="current"] mark transparency"#).await;
    Runner::execute(r#"[con_mark="current"] unmark current"#).await;
    if let Some(workspace) = Root::default().await.get_workspace(node) {
        if let Some(_focused) = workspace.get_focused() {
            Runner::execute(r#"opacity 1"#).await;
            Runner::execute(r#"mark current"#).await;
        }
    }
}

async fn undo_autotransparency(){
    debug!("undo_autotransparency");
    Runner::execute(r#"[con_mark="transparency"] opacity 1"#).await;
    Runner::execute(r#"[con_mark="current"] unmark transparency"#).await;
}

async fn autotiling(node: &Node){
    debug!("autotiling");
    if let Some(workspace) = Root::default().await.get_workspace(node) {
        if let Some(focused) = workspace.get_focused() {
            debug!("Focused: {:?}", focused);
            if focused.rect.height > focused.rect.width{
                Runner::execute("splitv").await;
            }else{
                Runner::execute("splith").await;
            }
        }
    }
}
