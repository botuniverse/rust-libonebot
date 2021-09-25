use serde::Deserialize;

#[derive(Clone)]
pub struct Action {
    pub action: fn(_: serde_json::Value) -> String,
}

impl From<fn(_: serde_json::Value) -> String> for Action {
    fn from(action: fn(_: serde_json::Value) -> String) -> Self {
        Self { action }
    }
}

#[derive(Deserialize)]
pub(crate) struct ActionJson {
    pub action: String,
    pub params: serde_json::Value,
}
