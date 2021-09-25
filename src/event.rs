use crate::{Message, MessageSegment, Result, User};
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Event {
    id: String,
    pub platform: String,

    time: DateTime<Utc>,
    content: EventContent,

    pub bot_user: User,

    extended: HashMap<&'static str, String>,
}

impl Event {
    pub fn new(id: String, content: EventContent) -> Self {
        Self {
            id,
            platform: String::new(),

            time: Utc::now(),
            content,

            bot_user: User::default(),

            extended: HashMap::new(),
        }
    }

    pub(crate) fn to_json(&self) -> Result<String> {
        let ret = serde_json::to_string(&EventJson::from(self.clone()))?;
        return Ok(ret);
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
pub struct Notice {
    extended: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Request {
    extended: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Meta {
    extended: HashMap<String, String>,
}
