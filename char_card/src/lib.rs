pub mod error;
pub mod ffi;
pub mod wasm;

use std::collections::HashMap;

use crate::error::{CharCardError, Result};
use base64::Engine;
use inject_manager::InjectionManager;
use serde::{Deserialize, Serialize};
use serde_json as json;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct CharacterV2 {
    pub spec: String,
    pub spec_version: String,
    pub data: Character,
}

impl Default for CharacterV2 {
    fn default() -> Self {
        CharacterV2 {
            spec: "chara_card_v2".to_string(),
            spec_version: "2.0".to_string(),
            data: Character::default(),
        }
    }
}

impl CharacterV2 {
    fn from_v1(value: CharacterV1) -> Self {
        Self {
            data: Character {
                name: value.name,
                description: value.description,
                personality: value.personality,
                scenario: value.scenario,
                first_mes: value.first_mes,
                mes_example: value.mes_example,
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

impl From<CharacterV1> for CharacterV2 {
    fn from(value: CharacterV1) -> Self {
        Self::from_v1(value)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct CharacterV1 {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub personality: String,
    #[serde(default)]
    pub scenario: String,
    #[serde(default)]
    pub first_mes: String,
    #[serde(default)]
    pub mes_example: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Default)]
pub struct Character {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub personality: String,
    #[serde(default)]
    pub scenario: String,
    #[serde(default)]
    pub first_mes: String,
    #[serde(default)]
    pub mes_example: String,
    #[serde(default)]
    pub creator_notes: String,
    #[serde(default)]
    pub system_prompt: String,
    #[serde(default)]
    pub post_history_instructions: String,
    #[serde(default)]
    pub alternate_greetings: Vec<String>,
    #[serde(default)]
    pub character_book: InjectionManager,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub creator: String,
    #[serde(default)]
    pub character_version: String,
    #[serde(default)]
    pub extensions: HashMap<String, json::Value>,
}

impl Character {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn personality(&self) -> &str {
        &self.personality
    }

    pub fn scenario(&self) -> &str {
        &self.scenario
    }

    pub fn first_mes(&self) -> &str {
        &self.first_mes
    }

    pub fn mes_example(&self) -> &str {
        &self.mes_example
    }

    pub fn creator(&self) -> &str {
        &self.creator
    }

    pub fn character_version(&self) -> &str {
        &self.character_version
    }

    pub fn extensions(&self) -> HashMap<String, json::Value> {
        self.extensions.clone()
    }

    pub fn tags(&self) -> Vec<String> {
        self.tags.clone()
    }

    pub fn character_book(&self) -> InjectionManager {
        self.character_book.clone()
    }

    pub fn alternate_greetings(&self) -> Vec<String> {
        self.alternate_greetings.clone()
    }

    pub fn post_history_instructions(&self) -> &str {
        &self.post_history_instructions
    }

    pub fn system_prompt(&self) -> &str {
        &self.system_prompt
    }

    pub fn creator_notes(&self) -> &str {
        &self.creator_notes
    }

    pub fn from_json<S: AsRef<str>>(char_json: S) -> Result<Self> {
        if let Ok(char) = json::from_str::<CharacterV1>(char_json.as_ref()) {
            return Ok(CharacterV2::from_v1(char).data);
        }

        if let Ok(char) = json::from_str::<CharacterV2>(char_json.as_ref()) {
            return Ok(char.data);
        }
        Err(CharCardError::NoCharacterFound)
    }

    pub fn from_png<R: std::io::Read>(r: R) -> Result<Character> {
        let decoder = png::Decoder::new(r);
        let mut reader = decoder.read_info().unwrap();
        reader.finish().unwrap();
        for text_chunk in &reader.info().uncompressed_latin1_text {
            if text_chunk.keyword == "chara" {
                let char_data =
                    base64::prelude::BASE64_STANDARD.decode(text_chunk.text.as_bytes())?;
                let char_json = std::str::from_utf8(&char_data)?;

                if let Ok(char) = json::from_str::<CharacterV2>(char_json) {
                    return Ok(char.data)
                }

                match json::from_str::<CharacterV1>(char_json) {
                    Ok(char) => return Ok(CharacterV2::from_v1(char).data),
                    Err(e) => return Err(CharCardError::InvalidJsonData(e))
                }
            }
        }
        Err(CharCardError::NoCharacterFound)
    }
}