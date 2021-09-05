use crate::{Group, User};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Message {
    id: i64,
    source: MessageSource,
    sender: User,

    content: Vec<MessageSegment>,

    extended: HashMap<String, String>,
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
