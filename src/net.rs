use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::{BufRead, BufReader, Error, ErrorKind},
    net::Ipv4Addr,
    process::exit,
    process::{Command, Stdio},
    str::FromStr,
};

use regex::Regex;

use pnet::util::MacAddr;

use tempfile::NamedTempFile;

use crate::run::ExitCodes;

use serde_xml_rs::from_reader;

#[derive(Debug, Clone)]
pub struct Gateway {
    pub ip: Ipv4Addr,
    pub mac: MacAddr,
}

// ip  : IP to scan (192.168.1.0)
// msak: Net Mask to scan (/24)
#[derive(Debug, Clone)]
pub struct ScanInfo {
    pub addr: Ipv4Addr,
    pub mask: u8,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Times {
    pub srtt: String,
    pub rttvar: String,
    pub to: String,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Address {
    pub addr: String,
    pub addrtype: String,

    #[serde(rename = "vendor", default = "get_unknown")]
    pub vendor: String,
}

#[derive(PartialEq, Debug, Deserialize)]
pub struct Status {
    pub state: String,
    pub reason: String,
    pub reason_ttl: String,
}

#[derive(Debug, Deserialize)]
pub struct Host {
    #[serde(rename = "status")]
    pub status: Status,

    #[serde(rename = "address")]
    pub address: Vec<Address>,
}

#[derive(Debug, Deserialize)]
pub struct NmapRun {
    pub scanner: String,
    pub args: String,
    pub start: String,
    pub startstr: String,
    pub version: String,
    pub xmloutputversion: String,

    #[serde(rename = "host", default)]
    pub hosts: Vec<Host>,
}

impl PartialEq for Host {
    fn eq(&self, other: &Host) -> bool {
        self.address[0].addr == other.address[0].addr
    }
}

fn get_unknown() -> String { "Unknown".to_string() }

//ipmask: IP/Mask in String format like '192.168.1.0/24'
//round: Total round of scan can increase accuracy of result
pub fn do_scan_nmap(ipmask: &String, round: u8) -> Vec<Host> {
    let mut res: Vec<Host> = vec![];

    for _ in 0..round {
        let file_temp = NamedTempFile::new().expect("Could not create tempfile");
        let file_temp_path = file_temp
            .path()
            .to_str()
            .expect("Could not find tempfile")
            .to_string();

        match Command::new("nmap")
            .arg("-sn")
            .arg("-PS")
            .arg(format!("{}", &ipmask))
            .arg("-oX")
            .arg(file_temp_path.clone())
            .stdout(Stdio::null())
            .output()
        {
            Ok(r) => r,
            Err(ref e) if e.kind() == ErrorKind::NotFound => {
                exit(ExitCodes::NmapNotInstalled as i32);
            }
            Err(_) => {
                exit(ExitCodes::NmapRunError as i32);
            }
        };

        let reader = BufReader::new(file_temp.as_file());

        let result: NmapRun = from_reader(reader).unwrap();

        for g in result.hosts {
            if !res.contains(&g) {
                res.push(g);
            }
        }
    }

    res
}

pub fn do_scan_arp() -> HashMap<Ipv4Addr, MacAddr> {
    let path = "/proc/net/arp";

    let mut map: HashMap<Ipv4Addr, MacAddr> = HashMap::new();

    let _f = match OpenOptions::new().read(true).write(false).open(path) {
        Ok(v) => {
            let file = BufReader::new(&v);

            for (num, line) in file.lines().enumerate() {
                if num == 0 {
                    continue;
                }

                let l = line.unwrap();
                let mut x = l.split_whitespace();

                let str_ip: &str = x.nth(0).unwrap();
                let str_mac: &str = x.nth(2).unwrap();

                if str_mac == "00:00:00:00:00:00" {
                    continue;
                }

                let ip: Ipv4Addr = str_ip.parse().unwrap();
                let mac: MacAddr = MacAddr::from_str(str_mac).unwrap();

                map.insert(ip, mac);
            }

            return map;
        }
        Err(e) => panic!("Unable to read mac address from {}, Err: {}", path, e),
    };
}

pub fn get_hostname_addr() -> Result<Ipv4Addr, Error> {
    let output = match Command::new("hostname").arg("-i").output() {
        Ok(r) => r,
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            error!("Hostname command not found!");
            exit(ExitCodes::HostnameNotFound as i32);
        }
        Err(e) => {
            error!("Hostname command run error: {}", e);
            exit(ExitCodes::HostnameRunError as i32);
        }
    };

    let ip: Ipv4Addr = match String::from_utf8_lossy(&output.stdout).trim().parse() {
        Ok(r) => r,
        Err(e) => {
            error!("Hostname output parse error: {}", e);
            exit(ExitCodes::HostnameParseError as i32);
        }
    };
    Ok(ip)
}

pub fn get_gateway() -> Gateway {
    lazy_static! {
        static ref RGX_MAC: Regex = Regex::new(r"([0-9a-fA-F]{1,2}[\.:-]){5}([0-9a-fA-F]{1,2})").unwrap();
        static ref RGX_IP: Regex = Regex::new(r"((?:[0-9]{1,3}\.){3}[0-9]{1,3})").unwrap();
    }

    let output = Command::new("arp")
        .arg("-a")
        .arg("_gateway")
        .output()
        .expect("failed to execute process");

    let f = String::from_utf8(output.stdout).unwrap();

    if output.status.success() {
        let ip = RGX_IP.find(&f).unwrap().as_str().parse().unwrap();
        let mac = RGX_MAC.find(&f).unwrap().as_str().parse().unwrap();

        let gw: Gateway = Gateway { ip: ip, mac: mac };

        return gw;
    } else {
        let err = String::from_utf8_lossy(&output.stderr);
        panic!("Unable to get gw! Err: {}", err);
    }
}
