use crate::{Comm, Event, Result};
use async_trait::async_trait;
use tokio::sync::broadcast::{Receiver, Sender};

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
        _action_sender: Sender<String>,
        event_sender: Sender<Event>,
    ) -> Result<()> {
        let mut event_receiver = event_sender.subscribe();
        let client = reqwest::Client::new();
        loop {
            let event = event_receiver.recv().await;
            if let Ok(event) = event {
                let resp = client
                    .post(&self.post_url)
                    .body(event.to_json().unwrap())
                    .send()
                    .await?;
            }
        }
    }
}
