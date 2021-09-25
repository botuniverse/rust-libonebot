use crate::{Group, User};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Message {
    pub id: String,

    pub source: MessageSource,
    pub sender: User,

    pub content: Vec<MessageSegment>,

    extended: HashMap<String, String>,
}

impl Message {
    pub fn new(id: String, source: MessageSource, sender: User) -> Self {
        Self {
            id,
            source,
            sender,
            content: Vec::new(),
            extended: HashMap::new(),
        }
    }

    pub fn append(mut self, seg: MessageSegment) -> Self {
        self.content.push(seg);
        self
    }
}

#[derive(Debug, Clone)]
pub enum MessageSource {
    Private(User),
    Group(Group),
}

#[derive(Debug, Clone, Serialize)]
pub enum MessageSegment {
    Text(String),
    Emoji(String),
    Image(Media),
    Record(Media),
    Video(Media),
    At(User),
    Location(f64, f64), // lat, lon
    Reply(i64),
    Foward(i64),
    Custom(HashMap<String, String>),
}

#[derive(Debug, Clone, Serialize)]
pub enum Media {
    File(String),
    URL(String, bool, bool, bool), // url, cache, proxy, timeout
    Base64(String),
}
