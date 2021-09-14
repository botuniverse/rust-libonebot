use crate::{Error, Result};
use serde::Deserialize;

pub struct Config {
    pub heartbeat: Option<u32>,
    pub auth: Auth,
    pub channel_capacity: ChannelCapacity,
}

impl Config {
    pub fn new() -> Self {
        Self {
            heartbeat: None,
            auth: Auth { access_token: None },
            channel_capacity: ChannelCapacity {
                action: 0 as usize,
                event: 0 as usize,
            },
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

        Ok(config)
    }
}

pub struct Auth {
    pub access_token: Option<String>,
}

pub struct ChannelCapacity {
    pub action: usize,
    pub event: usize,
}

pub trait ConfigFile {
    fn heartbeat(&self) -> Option<&ConfigFileHeartBeat>;
    fn auth(&self) -> Option<&ConfigFileAuth>;
    fn channel_capacity(&self) -> Option<&ConfigFileChannelCapacity>;
    fn comm_methods(&self) -> Option<&Vec<ConfigFileCommMethod>>;
}

#[derive(Deserialize)]
pub struct ConfigFileHeartBeat {
    pub enabled: bool,
    pub interval: Option<u32>,
}

#[derive(Deserialize)]
pub struct ConfigFileAuth {
    pub access_token: Option<String>,
}

#[derive(Deserialize)]
pub struct ConfigFileChannelCapacity {
    pub action: Option<usize>,
    pub event: Option<usize>,
}

#[derive(Deserialize)]
pub struct ConfigFileCommMethod {
    pub r#type: String,

    pub host: Option<String>,
    pub port: Option<u16>,

    pub url: Option<String>,

    pub timeout: Option<u32>,
    pub secret: Option<String>,

    pub reconnect_interval: Option<u32>,
}

#[derive(Deserialize)]
pub struct DefaultConfigFile {
    heartbeat: Option<ConfigFileHeartBeat>,
    auth: Option<ConfigFileAuth>,
    channel_capacity: Option<ConfigFileChannelCapacity>,
    comm_methods: Option<Vec<ConfigFileCommMethod>>,
}

impl DefaultConfigFile {
    pub fn new() -> Self {
        Self {
            heartbeat: None,
            auth: None,
            channel_capacity: None,
            comm_methods: None,
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
    fn comm_methods(&self) -> Option<&Vec<ConfigFileCommMethod>> {
        match &self.comm_methods {
            Some(comm_methods) => Some(&comm_methods),
            None => None,
        }
    }
}
