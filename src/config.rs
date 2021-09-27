use crate::{Error, Result};
use serde::Deserialize;
use std::{collections::HashMap, fmt::Debug};

#[derive(Debug)]
pub struct Config {
    pub heartbeat: Option<u32>,
    pub auth: Auth,
    pub channel_capacity: usize,
}

impl Config {
    pub fn new() -> Self {
        Self {
            heartbeat: None,
            auth: Auth { access_token: None },
            channel_capacity: crate::DEFAULT_CHANNEL_CAPACITY,
        }
    }

    pub fn from_config_file<F: ConfigFile>(config_file: &F) -> Result<Self> {
        let mut config = Self::new();

        if let Some(heartbeat) = config_file.heartbeat() {
            if !heartbeat.enabled {
                config.heartbeat = None;
            } else {
                if let Some(interval) = heartbeat.interval {
                    config.heartbeat = Some(interval);
                } else {
                    return Err(Error::msg(
                        "config error: no heartbeat.interval when heartbeat.enabled is true",
                    ));
                }
            }
        }

        if let Some(auth) = config_file.auth() {
            if let Some(access_token) = &auth.access_token {
                config.auth.access_token = Some(access_token.clone());
            }
        }

        if let Some(channel_capacity) = config_file.channel_capacity() {
            if let Some(capacity) = channel_capacity.event {
                config.channel_capacity = capacity;
            }
        }

        Ok(config)
    }
}

#[derive(Debug)]
pub struct Auth {
    pub access_token: Option<String>,
}

pub trait ConfigFile: Debug {
    fn heartbeat(&self) -> Option<&ConfigFileHeartBeat>;
    fn auth(&self) -> Option<&ConfigFileAuth>;
    fn channel_capacity(&self) -> Option<&ConfigFileChannelCapacity>;
    fn comm_methods(&self) -> Option<&HashMap<String, ConfigFileCommMethod>>;
}

#[derive(Debug, Deserialize)]
pub struct ConfigFileHeartBeat {
    pub enabled: bool,
    pub interval: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct ConfigFileAuth {
    pub access_token: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ConfigFileChannelCapacity {
    pub event: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct ConfigFileCommMethod {
    pub r#type: String,

    pub host: Option<String>,
    pub port: Option<u16>,

    pub url: Option<String>,

    pub timeout: Option<u32>,
    pub secret: Option<String>,

    pub reconnect_interval: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct DefaultConfigFile {
    heartbeat: Option<ConfigFileHeartBeat>,
    auth: Option<ConfigFileAuth>,
    channel_capacity: Option<ConfigFileChannelCapacity>,
    comm_method: Option<HashMap<String, ConfigFileCommMethod>>,
}

impl DefaultConfigFile {
    pub fn new() -> Self {
        Self {
            heartbeat: None,
            auth: None,
            channel_capacity: None,
            comm_method: None,
        }
    }
}

impl ConfigFile for DefaultConfigFile {
    fn heartbeat(&self) -> Option<&ConfigFileHeartBeat> {
        match &self.heartbeat {
            Some(heartbeat) => Some(&heartbeat),
            None => None,
        }
    }
    fn auth(&self) -> Option<&ConfigFileAuth> {
        match &self.auth {
            Some(auth) => Some(&auth),
            None => None,
        }
    }
    fn channel_capacity(&self) -> Option<&ConfigFileChannelCapacity> {
        match &self.channel_capacity {
            Some(channelcapacity) => Some(&channelcapacity),
            None => None,
        }
    }
    fn comm_methods(&self) -> Option<&HashMap<String, ConfigFileCommMethod>> {
        match &self.comm_method {
            Some(comm_methods) => Some(&comm_methods),
            None => None,
        }
    }
}
