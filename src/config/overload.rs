use std::collections::HashMap;

use serde::Deserialize;

use crate::utils::{GenericResult, Compilable};

use super::pattern::Pattern;

#[derive(Deserialize)]
#[serde(transparent)]
pub struct OverloadsContainer {
    overloads: Vec<Overload>,
}
impl OverloadsContainer {
    pub fn find_overload_name(&self, param: &str) -> GenericResult<Option<&str>> {
        for overload in &self.overloads {
            for pattern in &overload.patterns {
                let matcher = pattern.compile()?;
                if matcher.is_match(param) {
                    return Ok(Some(&overload.name));
                }
            }
        }
        return Ok(None);
    }
}
impl Default for OverloadsContainer {
    fn default() -> Self {
        Self { overloads: Default::default() }
    }
}

#[derive(Deserialize)]
pub struct Overload {
    pub name: String,
    pub patterns: Vec<Pattern>,
}

#[derive(Deserialize)]
pub struct Overloadable<T> {
    #[serde(flatten)]
    params: T,
    #[serde(default)]
    overloads: HashMap<String, T>,
}
impl<T> Overloadable<T> {
    pub fn get_params(&self, name: Option<&str>) -> &T {
        if let Some(name) = name {
            self.overloads.get(name).unwrap_or(&self.params)
        } else {
            &self.params
        }
    }
}
