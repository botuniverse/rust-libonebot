use crate::{config::ConfigFileCommMethod, Action, Comm, Event, Result};
use async_trait::async_trait;
use std::{collections::HashMap, fmt::Display};
use tokio::sync::broadcast::Sender;

#[derive(Debug, Clone)]
pub struct HTTPWebHook {
    post_url: String,
    secret: Option<String>,
}

impl HTTPWebHook {
    pub fn new<S: Display>(post_url: S) -> Self {
        Self {
            post_url: post_url.to_string(),
            secret: None,
        }
    }

    pub fn secret<S: Display>(mut self, secret: S) -> Self {
        self.secret = Some(secret.to_string());
        self
    }

    pub(crate) fn from_config_file_comm_method(
        comm_method: &ConfigFileCommMethod,
    ) -> Result<Box<dyn Comm>> {
        let mut http_webhook = Self::new(
            comm_method
                .url
                .clone()
                .unwrap_or_else(|| "127.0.0.1:5700".to_string()),
        );
        if let Some(secret) = comm_method.secret.clone() {
            http_webhook = http_webhook.secret(secret);
        }
        Ok(Box::new(http_webhook))
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
                event = event.platform(&platform);
                let _ = client
                    .post(&self.post_url)
                    .body(event.to_json().unwrap())
                    .send()
                    .await?;
            }
        }
    }
}
