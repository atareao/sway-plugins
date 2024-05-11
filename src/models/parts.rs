use regex::{
    Captures,
    Regex,
};
use tracing::{debug, info};


#[derive(Debug)]
pub struct Parts{
    pub num: String,
    pub name: Option<String>,
    pub icons: Option<String>,
}


impl From<String> for Parts {
    fn from(name: String) -> Self{
        info!("from");
        let re = Regex::new(r"(?P<num>[0-9]+):?(?P<name>\w+)? ?(?P<icons>.+)?").unwrap();
        Parts::from(re.captures(&name).unwrap())
    }
}

impl From<Parts> for String {
    fn from(parts: Parts) -> Self {
        info!("from");
        debug!("from: {:?}", parts);
        if parts.name.is_some() || parts.icons.is_some() {
            let name = parts.name.clone().unwrap_or("".to_string());
            if parts.icons.is_some() {
                let icons = parts.icons.clone().unwrap_or("".to_string());
                format!("{}:{} {}", parts.num, name, icons)
            }else{
                format!("{}:{}", parts.num, name)

            }
        }else{
            parts.num.clone()
        }
    }
}

impl From<Captures<'_>> for Parts {
    fn from(caps: Captures) -> Self{
        info!("from");
        debug!("from: {:?}", caps);
        Self {
            num: caps.name("num").map_or("".to_string(), |v| v.as_str().to_string()),
            name: caps.name("name").map(|v| v.as_str().to_string()),
            icons: caps.name("icons").map(|v| v.as_str().to_string()),
        }
    }
}
