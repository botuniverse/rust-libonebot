use crate::{Message, MessageSegment, Result, User};
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone)]
pub struct Event {
    pub id: String,
    pub platform: String,

    pub time: DateTime<Utc>,
    pub content: EventContent,

    pub bot_user: User,
}

impl Event {
    pub fn new<S: Display>(id: S) -> EventBuilder {
        EventBuilder {
            id: id.to_string(),
            platform: String::new(),
            time: Utc::now(),
            bot_user: User::new("-1"),
        }
    }

    pub fn platform<S: Display>(mut self, platform: S) -> Self {
        self.platform = platform.to_string();
        self
    }

    pub fn time(mut self, time: DateTime<Utc>) -> Self {
        self.time = time;
        self
    }

    pub fn bot_user<S: Display>(mut self, user: User) -> Self {
        self.bot_user = user;
        self
    }

    pub(crate) fn to_json(&self) -> Result<String> {
        let ret = serde_json::to_string(&EventJson::from(self.clone()))?;
        return Ok(ret);
    }
}

pub struct EventBuilder {
    id: String,
    platform: String,

    time: DateTime<Utc>,

    bot_user: User,
}

impl EventBuilder {
    pub fn platform<S: Display>(mut self, platform: S) -> Self {
        self.platform = platform.to_string();
        self
    }

    pub fn time(mut self, time: DateTime<Utc>) -> Self {
        self.time = time;
        self
    }

    pub fn bot_user<S: Display>(mut self, user: User) -> Self {
        self.bot_user = user;
        self
    }

    pub fn message(self, message: Message) -> Event {
        Event {
            id: self.id,
            platform: self.platform,
            time: self.time,
            content: EventContent::Message(message),
            bot_user: self.bot_user,
        }
    }

    pub fn notice(self, notice: Notice) -> Event {
        Event {
            id: self.id,
            platform: self.platform,
            time: self.time,
            content: EventContent::Notice(notice),
            bot_user: self.bot_user,
        }
    }

    pub fn request(self, request: Request) -> Event {
        Event {
            id: self.id,
            platform: self.platform,
            time: self.time,
            content: EventContent::Request(request),
            bot_user: self.bot_user,
        }
    }

    pub fn meta(self, meta: Meta) -> Event {
        Event {
            id: self.id,
            platform: self.platform,
            time: self.time,
            content: EventContent::Meta(meta),
            bot_user: self.bot_user,
        }
    }
}

#[derive(Serialize)]
struct EventJson {
    id: String,
    platform: String,

    time: i64,
    self_id: String,
    r#type: String,
    detail_type: Option<String>,
    sub_type: String,

    message: Option<Vec<MessageSegment>>,
    message_id: Option<String>,
    user_id: Option<String>,
    alt_message: Option<String>,

    group_id: Option<String>,

    flag: Option<String>,
}

impl From<Event> for EventJson {
    fn from(event: Event) -> Self {
        Self {
            id: event.id,
            platform: event.platform,
            time: event.time.timestamp(),
            self_id: event.bot_user.id.clone(),
            r#type: event.content.r#type(),
            detail_type: if let EventContent::Message(message) = &event.content {
                use crate::message::MessageSource;
                Some(
                    match message.source {
                        MessageSource::Private(_) => "private",
                        MessageSource::Group(_) => "group",
                    }
                    .to_string(),
                )
            } else {
                None
            },
            sub_type: String::new(),
            message: if let EventContent::Message(message) = &event.content {
                Some(message.content.clone())
            } else {
                None
            },
            message_id: if let EventContent::Message(message) = &event.content {
                Some(message.id.clone())
            } else {
                None
            },
            user_id: if let EventContent::Message(message) = &event.content {
                Some(message.sender.id.clone())
            } else {
                None
            },
            alt_message: None,
            group_id: if let EventContent::Message(message) = &event.content {
                use crate::message::MessageSource;
                if let MessageSource::Group(group) = &message.source {
                    Some(group.id.clone())
                } else {
                    None
                }
            } else {
                None
            },
            flag: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum EventContent {
    Message(Message),
    Notice(Notice),
    Request(Request),
    Meta(Meta),
}

impl EventContent {
    fn r#type(&self) -> String {
        match self {
            Self::Message(_) => "message",
            Self::Notice(_) => "notice",
            Self::Request(_) => "request",
            Self::Meta(_) => "meta",
        }
        .to_string()
    }
}

#[derive(Debug, Clone)]
pub struct Notice {}

#[derive(Debug, Clone)]
pub struct Request {
    flag: String,
}

#[derive(Debug, Clone)]
pub struct Meta {
    extended: HashMap<String, String>,
}
