pub mod scanner;
pub mod manager;
pub mod icon;
pub mod settings;
pub mod monitor;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartupItem {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub path: String,
    pub command: String,
    pub icon: Option<String>,
    pub source: String,
    pub source_type: String,
    pub source_location: String,
    pub enabled: bool,
    pub valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SourceType {
    Registry,
    Folder,
}

impl ToString for SourceType {
    fn to_string(&self) -> String {
        match self {
            SourceType::Registry => "registry".to_string(),
            SourceType::Folder => "folder".to_string(),
        }
    }
}
