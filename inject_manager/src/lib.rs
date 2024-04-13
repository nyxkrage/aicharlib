pub mod ffi;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Default)]
pub struct InjectionEntry {
    #[serde(default)]
    pub keys: Vec<String>,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub extensions: extensions::Extensions,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub insertion_order: i32,
    #[serde(default)]
    pub case_sensitive: bool,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub priority: i32,
    #[serde(default)]
    pub id: i32,
    #[serde(default)]
    pub comment: String,
    #[serde(default)]
    pub selective: bool,
    #[serde(default)]
    pub secondary_keys: Vec<String>,
    #[serde(default)]
    pub constant: bool,
    #[serde(default)]
    pub position: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Default)]
pub struct InjectionManager {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub scan_depth: i32,
    #[serde(default)]
    pub token_budget: i32,
    #[serde(default)]
    pub recursive_scanning: bool,
    #[serde(default)]
    pub extensions: extensions::Extensions,
    #[serde(default)]
    pub entries: Vec<InjectionEntry>,
}