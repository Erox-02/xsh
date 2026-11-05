use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Default, Deserialize)]

pub struct Conf {
    pub aliases : Option<HashMap<String, String>>,
}

impl Conf {
    pub fn load() -> Self {
        let path = Path::new("xsh.toml");
        if let Ok(content) = fs::read_to_string(path) {
            toml::from_str(&content).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn resolv_alias(&self, command: &str, args: &[&str]) -> Option<String> {
        if let Some(aliases) = &self.aliases {
            if let Some(alias_cmd) = aliases.get(command) {
                if args.is_empty() {
                    return Some(alias_cmd.clone());  
                } else {
                    return Some(format!("{} {}", alias_cmd, args.join(" ")));
                }
            }
        }
        None
    }
}