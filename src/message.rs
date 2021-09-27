use crate::{Group, User};
use serde::Serialize;
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone)]
pub struct Message {
    pub id: String,

    pub source: MessageSource,
    pub sender: User,

    pub content: Vec<MessageSegment>,
}

impl Message {
    pub fn new<S: Display>(id: S) -> MessageBuilder {
        MessageBuilder {
            id: id.to_string(),
            content: Vec::new(),
        }
    }
    pub fn from_private(self, user: User) -> Message {
        Message {
            id: self.id,
            source: MessageSource::Private(user.clone()),
            sender: user,
            content: self.content,
        }
    }

    pub fn from_group(self, group: Group, sender: User) -> Message {
        Message {
            id: self.id,
            source: MessageSource::Group(group),
            sender,
            content: self.content,
        }
    }

    pub fn text<S: Display>(mut self, text: S) -> Self {
        self.content.push(MessageSegment::Text(text.to_string()));
        self
    }
}

pub struct MessageBuilder {
    id: String,

    content: Vec<MessageSegment>,
}

impl MessageBuilder {
    pub fn from_private(self, user: User) -> Message {
        Message {
            id: self.id,
            source: MessageSource::Private(user.clone()),
            sender: user,
            content: self.content,
        }
    }

    pub fn from_group(self, group: Group, sender: User) -> Message {
        Message {
            id: self.id,
            source: MessageSource::Group(group),
            sender,
            content: self.content,
        }
    }

    pub fn text<S: Display>(mut self, text: S) -> Self {
        self.content.push(MessageSegment::Text(text.to_string()));
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
