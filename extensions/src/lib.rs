pub mod ffi;

use std::collections::HashMap;
use serde_json as json;

pub type Extensions = HashMap<String, json::Value>;