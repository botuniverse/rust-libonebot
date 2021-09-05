use crate::{Message, MessageSegment, Result, User};
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Event {
    time: DateTime<Utc>,
    content: EventContent,

    bot_user: User,

    extended: HashMap<&'static str, String>,
}

impl Event {
    pub(crate) fn new(content: EventContent, bot_user: User) -> Self {
        Self {
            time: Utc::now(),
            content,

            bot_user,

            extended: HashMap::new(),
        }
    }

    pub(crate) fn to_json(&self) -> Result<String> {
        let ret = serde_json::to_string(&EventJson::from(self.clone()))?;
        return Ok(ret);
    }

    fn detail_type(&self) -> &'static str {
        ""
    }

    fn sub_type(&self) -> &'static str {
        ""
    }
}

#[derive(Serialize)]
struct EventJson {
    time: i64,
    self_id: String,
    r#type: &'static str,
    detail_type: &'static str,
    sub_type: &'static str,

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
            time: event.time.timestamp(),
            self_id: event.bot_user.id.clone(),
            r#type: event.content.r#type(),
            detail_type: event.detail_type(),
            sub_type: event.sub_type(),
            message: None,
            message_id: None,
            user_id: None,
            alt_message: None,
            group_id: None,
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
    Stop,
}

impl EventContent {
    fn r#type(&self) -> &'static str {
        match self {
            Self::Message(_) => "message",
            Self::Notice(_) => "notice",
            Self::Request(_) => "request",
            Self::Meta(_) => "meta",
            Self::Stop => "stop",
        }
    }
    fn is_stop(&self) -> bool {
        if let Self::Stop = self {
            true
        } else {
            false
        }
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
