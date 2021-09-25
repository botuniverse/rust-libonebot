use crate::{Action, Comm, Event, Result};
use async_trait::async_trait;
use std::collections::HashMap;
use tokio::sync::broadcast::Sender;

#[derive(Debug, Clone)]
pub struct HTTPWebHook {
    post_url: String,
    secret: String,
}

impl HTTPWebHook {
    pub fn new(post_url: String, secret: String) -> Self {
        Self { post_url, secret }
    }
}

#[async_trait]
impl Comm for HTTPWebHook {
    async fn start(
        &self,
        _action_handlers: HashMap<String, Action>,
        event_sender: Sender<Event>,
        platform: String,
    ) -> Result<()> {
        let mut event_receiver = event_sender.subscribe();
        let client = reqwest::Client::new();
        loop {
            let event = event_receiver.recv().await;
            if let Ok(mut event) = event {
                event.platform = platform.clone();
                let _ = client
                    .post(&self.post_url)
                    .body(event.to_json().unwrap())
                    .send()
                    .await?;
            }
        }
    }
}
