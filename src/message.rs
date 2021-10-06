use crate::{Group, User};
use serde::Serialize;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Message {
    pub id: String,

    pub source: MessageSource,
    pub sender: User,

    pub content: Vec<MessageSegment>,
}

impl Message {
    pub fn build<S: Display>(id: S) -> MessageBuilder {
        MessageBuilder {
            id: id.to_string(),
            content: Vec::new(),
        }
    }
    pub fn private(self, user: User) -> Message {
        Message {
            id: self.id,
            source: MessageSource::Private(user.clone()),
            sender: user,
            content: self.content,
        }
    }

    pub fn group(self, group: Group, sender: User) -> Message {
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

    pub fn emoji<S: Display>(mut self, emoji: S) -> Self {
        self.content.push(MessageSegment::Emoji(emoji.to_string()));
        self
    }

    pub fn image(mut self, image: Media) -> Self {
        self.content.push(MessageSegment::Image(image));
        self
    }

    pub fn record(mut self, record: Media) -> Self {
        self.content.push(MessageSegment::Record(record));
        self
    }

    pub fn video(mut self, video: Media) -> Self {
        self.content.push(MessageSegment::Video(video));
        self
    }

    pub fn at(mut self, user: User) -> Self {
        self.content.push(MessageSegment::At(user));
        self
    }

    pub fn location(mut self, lat: f64, lon: f64) -> Self {
        self.content.push(MessageSegment::Location(lat, lon));
        self
    }

    pub fn reply<S: Display>(mut self, id: S) -> Self {
        self.content.push(MessageSegment::Reply(id.to_string()));
        self
    }

    pub fn forward<S: Display>(mut self, id: S) -> Self {
        self.content.push(MessageSegment::Forward(id.to_string()));
        self
    }

    pub fn custom<S: Display>(mut self, s: S) -> Self {
        self.content.push(MessageSegment::Custom(s.to_string()));
        self
    }
}

pub struct MessageBuilder {
    id: String,

    content: Vec<MessageSegment>,
}

impl MessageBuilder {
    pub fn private(self, user: User) -> Message {
        Message {
            id: self.id,
            source: MessageSource::Private(user.clone()),
            sender: user,
            content: self.content,
        }
    }

    pub fn group(self, group: Group, sender: User) -> Message {
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

    pub fn emoji<S: Display>(mut self, emoji: S) -> Self {
        self.content.push(MessageSegment::Emoji(emoji.to_string()));
        self
    }

    pub fn image(mut self, image: Media) -> Self {
        self.content.push(MessageSegment::Image(image));
        self
    }

    pub fn record(mut self, record: Media) -> Self {
        self.content.push(MessageSegment::Record(record));
        self
    }

    pub fn video(mut self, video: Media) -> Self {
        self.content.push(MessageSegment::Video(video));
        self
    }

    pub fn at(mut self, user: User) -> Self {
        self.content.push(MessageSegment::At(user));
        self
    }

    pub fn location(mut self, lat: f64, lon: f64) -> Self {
        self.content.push(MessageSegment::Location(lat, lon));
        self
    }

    pub fn reply<S: Display>(mut self, id: S) -> Self {
        self.content.push(MessageSegment::Reply(id.to_string()));
        self
    }

    pub fn forward<S: Display>(mut self, id: S) -> Self {
        self.content.push(MessageSegment::Forward(id.to_string()));
        self
    }

    pub fn custom<S: Display>(mut self, s: S) -> Self {
        self.content.push(MessageSegment::Custom(s.to_string()));
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
    Reply(String),
    Forward(String),
    Custom(String),
}

#[derive(Debug, Clone, Serialize)]
pub enum Media {
    File(String),
    URL(String), // url, cache, proxy, timeout
    Base64(String),
}

impl Media {
    pub fn new_file<S: Display>(file: S) -> Self {
        Self::File(file.to_string())
    }

    pub fn new_url<S: Display>(url: S) -> Self {
        Self::URL(url.to_string())
    }

    pub fn new_base64<S: Display>(base64: S) -> Self {
        Self::Base64(base64.to_string())
    }
}
