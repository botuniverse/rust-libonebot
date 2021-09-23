use serde::Deserialize;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Action {
    pub action: fn(_: serde_json::Value),
}

impl From<fn(_: serde_json::Value)> for Action {
    fn from(action: fn(_: serde_json::Value)) -> Self {
        Self { action }
    }
}

#[derive(Deserialize)]
pub(crate) struct ActionJson {
    pub action: String,
    pub params: serde_json::Value,
}
