use std::collections::HashMap;
use serde::Deserialize;

use super::generator::Generator;

fn get_preset() -> Vec<(&'static str, Generator)> {
    vec![
        ("path", Generator::Template("${DEPOT_LOCAL_REL_PATH}".to_owned())),
        ("full-path", Generator::Template("${DEPOT_LOCAL_PATH}".to_owned())),
    ]
}

pub struct FieldsDefinition {
    fields: HashMap<String, Generator>,
}
impl FieldsDefinition {
    pub fn get(&self, name: &str) -> Option<&Generator> {
        self.fields.get(name)
    }
}
impl Default for FieldsDefinition {
    fn default() -> Self {
        let mut fields: HashMap<String, Generator> = HashMap::new();
        for def in get_preset() {
            fields.insert(def.0.to_owned(), def.1);
        }
        Self { fields: fields }
    }
}
impl<'de> Deserialize<'de> for FieldsDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        let mut fields: HashMap<String, Generator> = Deserialize::deserialize(deserializer)?;

        for def in get_preset() {
            if ! fields.contains_key(def.0) {
                fields.insert(def.0.to_owned(), def.1);
            }
        }

        Ok(Self {
            fields: fields,
        })
    }
}
