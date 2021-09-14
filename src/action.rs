use serde::Deserialize;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Action {
    pub action: fn(_: HashMap<&str, String>),
}

impl From<fn(_: HashMap<&str, String>)> for Action {
    fn from(action: fn(_: HashMap<&str, String>)) -> Self {
        Self { action }
    }
}

#[derive(Deserialize)]
pub(crate) struct ActionJson {
    pub action: String,
}
