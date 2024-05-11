use std::collections::HashMap;
use tokio::fs::read_to_string;
use std::process;
use serde::{
    Serialize,
    Deserialize,
};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config{
    pub icons: HashMap<String, String>,
    pub duplicates: bool,
    pub autotiling: bool,
    pub autonaming: bool,
    pub autotransparency: bool,
    pub transparency: f32,
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
