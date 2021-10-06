use crate::{Error, Result};
use serde::Deserialize;
use std::{collections::HashMap, default::Default, fmt::Debug};

#[derive(Debug, Clone)]
pub struct Config {
    pub heartbeat: Option<u32>,
    pub auth: Auth,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        Self {
            heartbeat: None,
            auth: Auth { access_token: None },
        }
    }

    pub fn from_config_file<F: ConfigFile>(config_file: &F) -> Result<Self> {
        let mut config = Self::new();

        if let Some(heartbeat) = config_file.heartbeat() {
            if !heartbeat.enable {
                config.heartbeat = None;
            } else if let Some(interval) = heartbeat.interval {
                config.heartbeat = Some(interval);
            } else {
                return Err(Error::msg(
                    "config error: no heartbeat.interval when heartbeat.enabled is true",
                ));
            }
        }

        if let Some(auth) = config_file.auth() {
            if let Some(access_token) = &auth.access_token {
                config.auth.access_token = Some(access_token.clone());
            }
        }

        Ok(config)
    }
}

#[derive(Debug, Clone)]
pub struct Auth {
    pub access_token: Option<String>,
}

pub trait ConfigFile: Debug {
    fn heartbeat(&self) -> Option<&ConfigFileHeartBeat>;
    fn auth(&self) -> Option<&ConfigFileAuth>;
    fn comm_methods(&self) -> Option<&HashMap<String, ConfigFileCommMethod>>;
}

#[derive(Debug, Deserialize)]
pub struct ConfigFileHeartBeat {
    pub enable: bool,
    pub interval: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct ConfigFileAuth {
    pub access_token: Option<String>,
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
    comm_method: Option<HashMap<String, ConfigFileCommMethod>>,
}

impl DefaultConfigFile {
    pub fn new() -> Self {
        Self {
            heartbeat: None,
            auth: None,
            comm_method: None,
        }
    }
}

impl Default for DefaultConfigFile {
    fn default() -> Self {
        Self::new()
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
    fn comm_methods(&self) -> Option<&HashMap<String, ConfigFileCommMethod>> {
        match &self.comm_method {
            Some(comm_methods) => Some(&comm_methods),
            None => None,
        }
    }
}
