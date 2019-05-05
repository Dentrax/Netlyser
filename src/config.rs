use crate::serde_yaml;

use std::{
    fmt,
    fmt::{Display},
    collections::HashMap,
    io::{Read},
    fs::{OpenOptions},
    str::{FromStr},
};

use pnet::util::{MacAddr};

// 0: Notify nothing
// 1: Notify when connected
// 2: Notify when disconnected
// 3: Notify when connected or disconnected
#[derive(Debug)]
pub enum NotifyType {
    Nothing      = 0,
    Connected    = 1,
    Disconnected = 2,
    All          = 3,
}

#[derive(Debug)]
pub enum ConnectType {
    Connected    = 0,
    Disconnected = 1,
}

#[derive(Debug)]
pub enum AwarnessType {
    Known   = 0,
    Unknown = 1,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ErrorType {
    ReadError,
    SerdeError,
    ParseError,
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<std::io::Error> for ErrorType {
    fn from(_: std::io::Error) -> ErrorType {
        ErrorType::ReadError
    }
}

impl From<serde_yaml::Error> for ErrorType {
    fn from(_: serde_yaml::Error) -> ErrorType {
        ErrorType::SerdeError
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub general: General,
    pub hosts: Vec<Host>,

    #[serde(skip)]
    pub is_root: bool,
}

#[derive(Clone, Deserialize, Debug)]
pub struct General {
    pub interval: u64,
    pub round: u8,
    pub notify_on_connect: bool,
    pub notify_on_disconnect: bool,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Host {
    mac: String,
    name: String,
    device: String,
}

#[derive(Debug, Clone)]
pub struct HostInfo {
    pub name: String,
    pub device_name: String,
}

impl Config {
    fn is_valid(&self) -> bool {
        for host in &self.hosts {
            match MacAddr::from_str(host.mac.as_str()) {
                Ok(r) => r,
                Err(_) => return false,
            };
        }
        true
    }
}

pub fn get_config(filename: &str) -> Result<Config, ErrorType> {
    let mut file = OpenOptions::new().read(true).write(false).open(&filename).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let config: Config = serde_yaml::from_str(&content).unwrap();
    if config.is_valid() {
        Ok(config)
    } else {
        Err(ErrorType::ParseError)
    }
}

pub fn get_mac_info_map(config: Config) -> HashMap<MacAddr, HostInfo> {
    let mut map: HashMap<MacAddr, HostInfo> = HashMap::new();

    for host in config.hosts {
        let mac: MacAddr = MacAddr::from_str(&host.mac).unwrap();

        map.insert(mac, HostInfo{name: host.name, device_name: host.device});
    }

    map
}

