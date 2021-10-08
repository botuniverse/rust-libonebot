use crate::{Error, Result};
use serde::Deserialize;
use std::{collections::HashMap, default::Default, fmt::Debug};

#[derive(Debug, Clone)]
pub struct Config {
    pub auth: Auth,
    pub heartbeat: Option<u32>,
    pub log: Log,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        Self {
            auth: Auth { access_token: None },
            heartbeat: None,
            log: Log {
                output: LogOutput::Stderr,
                path: Some("./onebot.log".to_string()),
                level: log::LevelFilter::Info,
            },
        }
    }

    pub fn from_config_file<F: ConfigFile>(config_file: &F) -> Result<Self> {
        let mut config = Self::new();

        if let Some(auth) = config_file.auth() {
            if let Some(access_token) = &auth.access_token {
                config.auth.access_token = Some(access_token.clone());
            }
        }

        if let Some(log) = config_file.log() {
            if log.mode != "terminal"
                && log.mode != "file"
                && log.mode != "all"
                && log.mode != "off"
            {
                return Err(Error::msg("配置文件错误：未知的日志类型，应为：\"terminal\"、\"file\"、\"all\" 或 \"off\""));
            }
            if log.mode == "terminal" || log.mode == "all" {
                if let Some(output) = &log.output {
                    config.log.output = if output == "stderr" {
                        LogOutput::Stderr
                    } else if output == "stdout" {
                        LogOutput::Stdout
                    } else {
                        return Err(Error::msg(
                            "配置文件错误：未知的终端输出类型，应为：\"stderr\" 或 \"stdout\"",
                        ));
                    }
                }
            } else {
                config.log.output = LogOutput::Nul;
            }
            if log.mode == "file" || log.mode == "all" {
                if let Some(path) = &log.path {
                    config.log.path = Some(path.clone());
                }
            } else {
                config.log.path = None;
            }

            if let Some(level) = &log.level {
                config.log.level = if level == "error" {
                    log::LevelFilter::Error
                } else if level == "warn" {
                    log::LevelFilter::Warn
                } else if level == "info" {
                    log::LevelFilter::Info
                } else if level == "debug" {
                    log::LevelFilter::Debug
                } else if level == "trace" {
                    log::LevelFilter::Trace
                } else {
                    return Err(Error::msg("配置文件错误，未知的日志级别，应为：\"error\"、\"warn\"、\"info\"、\"debug\" 或 \"trace\""));
                }
            }
        }

        if let Some(heartbeat) = config_file.heartbeat() {
            if !heartbeat.enable {
                config.heartbeat = None;
            } else {
                config.heartbeat = Some(heartbeat.interval.unwrap_or(1000));
            }
        }

        Ok(config)
    }
}

#[derive(Debug, Clone)]
pub struct Auth {
    pub access_token: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Log {
    // multiple output and path?
    pub output: LogOutput,
    pub path: Option<String>,
    pub level: log::LevelFilter,
}

#[derive(Debug, Clone)]
pub enum LogOutput {
    Stderr,
    Stdout,
    Nul,
}

pub trait ConfigFile: Debug {
    fn auth(&self) -> Option<&ConfigFileAuth>;
    fn comm_methods(&self) -> Option<&HashMap<String, ConfigFileCommMethod>>;
    fn heartbeat(&self) -> Option<&ConfigFileHeartBeat>;
    fn log(&self) -> Option<&ConfigFileLog>;
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
pub struct ConfigFileHeartBeat {
    pub enable: bool,
    pub interval: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct ConfigFileLog {
    pub mode: String,
    pub output: Option<String>,
    pub path: Option<String>,
    pub level: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DefaultConfigFile {
    auth: Option<ConfigFileAuth>,
    comm_method: Option<HashMap<String, ConfigFileCommMethod>>,
    heartbeat: Option<ConfigFileHeartBeat>,
    log: Option<ConfigFileLog>,
}

impl DefaultConfigFile {
    pub fn new() -> Self {
        Self {
            auth: None,
            comm_method: None,
            heartbeat: None,
            log: None,
        }
    }
}

impl Default for DefaultConfigFile {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigFile for DefaultConfigFile {
    fn auth(&self) -> Option<&ConfigFileAuth> {
        match &self.auth {
            Some(auth) => Some(auth),
            None => None,
        }
    }
    fn comm_methods(&self) -> Option<&HashMap<String, ConfigFileCommMethod>> {
        match &self.comm_method {
            Some(comm_methods) => Some(comm_methods),
            None => None,
        }
    }
    fn heartbeat(&self) -> Option<&ConfigFileHeartBeat> {
        match &self.heartbeat {
            Some(heartbeat) => Some(heartbeat),
            None => None,
        }
    }
    fn log(&self) -> Option<&ConfigFileLog> {
        match &self.log {
            Some(log) => Some(log),
            None => None,
        }
    }
}
