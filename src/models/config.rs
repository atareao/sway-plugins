use tokio::fs::read_to_string;
use std::process;
use serde::Deserialize;
use super::super::plugins::{
    Autonaming,
    Autotiling,
    Autotransparency,
};


#[derive(Debug, Deserialize, Clone)]
pub struct Config{
    pub autonaming: Autonaming,
    pub autotiling: Autotiling,
    pub autotransparency: Autotransparency,
}

impl Config{
    pub async fn read_configuration(config_file: &str) -> Config{
        let content = match read_to_string(config_file)
            .await {
                Ok(value) => value,
                Err(e) => {
                    println!("Error with config file `config.yml`: {e}");
                    process::exit(0);
                }
            };
        match serde_yaml::from_str(&content){
            Ok(configuration) => configuration,
            Err(e) => {
                println!("Error with config file `config.yml`: {e}");
                process::exit(0);
            }
        }
    }
}
