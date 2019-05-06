// ====================================================
// Netlyser Copyright(C) 2019 Furkan Türkal
// This program comes with ABSOLUTELY NO WARRANTY; This is free software,
// and you are welcome to redistribute it under certain conditions; See
// file LICENSE, which is part of this source code package, for details.
// ====================================================

use std::{
    collections::HashMap,
    net::{Ipv4Addr},
};

use pnet::util::{MacAddr};

use crate::net;
use crate::config;

use rusqlite::types::ToSql;
use rusqlite::{Connection, Result, NO_PARAMS};

use notify_rust::Notification;

use chrono::prelude::*;

#[derive(Debug, Clone)]
pub struct NotifyInfo {
    pub host: Host,
    pub connected: bool,
    pub disconnected: bool,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Host {
    pub ip: Ipv4Addr,
    pub mac: MacAddr,
    pub name: String,
    pub device_name: String,
}

impl Host {
    pub fn new() -> Host {
        Host {
            ip: Ipv4Addr::UNSPECIFIED,
            mac: MacAddr::zero(),
            name: String::new(),
            device_name: String::new(),
        }
    }

    pub fn set_ip(&mut self, ip: Ipv4Addr) {
        self.ip = ip
    }

    pub fn set_mac(&mut self, mac: MacAddr) {
        self.mac = mac;
    }

    pub fn set_info(&mut self, info: &config::HostInfo) {
        if info.name.to_string().trim().is_empty() {
            self.name = "Unknown".to_string();
        } else {
            self.name = info.name.to_string();
        }
        if info.device_name.to_string().trim().is_empty() {
            self.device_name = "Unknown".to_string();
        } else {
            self.device_name = info.device_name.to_string();
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_device_name(&mut self, name: String) {
        self.device_name = name;
    }
}

pub fn get_notifies(olds: &Vec<Host>, news: &Vec<Host>, conf: &config::Config, db: &String) {
    let mut rmvs: Vec<Host> = vec![];
    let mut adds: Vec<Host> = vec![];

    let mut change: bool = false;

    for old in olds.clone() {
        if !news.contains(&old) {
            rmvs.push(old);
            change = true;
        }
    }

    for nev in news.clone() {
        if !olds.contains(&nev) {
            adds.push(nev);
            change = true;
        }
    }

    if change && (olds.len() != news.len()) {
        if rmvs.len() > 0 {
            on_hosts_disconnected(&db, rmvs, &conf);
        }
        if adds.len() > 0 {
            on_hosts_connected(&db, adds, &conf);
        }
    }

}

pub fn on_hosts_connected(db: &String, hosts: Vec<Host>, conf: &config::Config) {
    for h in hosts {
        if !conf.is_root && conf.general.notify_on_connect {
            notify(&h, true);
        }
        match add_to_db(&db, &h, true) {
            Ok(v) => {
                info!("[db::on_hosts_connected()]: 'add_to_db()' success: {:?}", v);
            }
            Err(e) => {
                warn!("[db::on_hosts_connected()]: error throwed when running 'add_to_db()' function. Err: {}, ", e);
            }
        }
    }
}

pub fn on_hosts_disconnected(db: &String, hosts: Vec<Host>, conf: &config::Config) {
    for h in hosts {
        if !conf.is_root && conf.general.notify_on_disconnect {
            notify(&h, false);
        }
        match add_to_db(&db, &h, false) {
            Ok(v) => {
                info!("[db::on_hosts_disconnected()]: 'add_to_db()' success: {:?}", v);
            }
            Err(e) => {
                warn!("[db::on_hosts_disconnected()]: error throwed when running 'add_to_db()' function. Err: {}, ", e);
            }
        }
    }
}

pub fn notify(host: &Host, con_or_dis: bool){
    let not: String = format!("Name: {}\nDevice: {}", host.name, host.device_name);
    if con_or_dis {
        Notification::new()
            .appname("Netlyser")
            .summary("CONNECT!")
            .body(&not)
            .timeout(5000)
            .show().unwrap();
    } else {
        Notification::new()
            .appname("Netlyser")
            .summary("DISCONNECT!")
            .body(&not)
            .timeout(5000)
            .show().unwrap();
    }
}

pub fn add_to_db(db: &String, host: &Host, con_or_dis: bool) -> Result<()>{
    let conn = Connection::open(db.to_string())?;
    let log_type = if con_or_dis { "connect".to_string() } else { "disconnect".to_string() };

    let res = conn.execute(
        "CREATE TABLE IF NOT EXISTS logs (
                   log_id           INTEGER PRIMARY KEY AUTOINCREMENT
                  ,log_name         TEXT NOT NULL
                  ,log_device       TEXT NOT NULL
                  ,log_ip           TEXT NOT NULL
                  ,log_mac          TEXT NOT NULL
                  ,log_type         TEXT NOT NULL
                  ,log_time         INTEGER NOT NULL
                  )",
        NO_PARAMS,
    )?;

    if res != 0 {
        warn!("[db::add_to_db()]: Unable to add to db. Code: {}, ", res);
    }

    info!("[db::add_to_db()]: create function exited with: {:?}", res);

    let exec = conn.execute(
        "INSERT INTO logs (log_name, log_device, log_ip, log_mac, log_type, log_time) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        &[&host.name as &ToSql, &host.device_name as &ToSql, &host.ip.to_string() as &ToSql, &host.mac.to_string() as &ToSql, &log_type, &Local::now().timestamp() as &ToSql],
    )?;

    if exec != 1 {
        warn!("[db::add_to_db()]: Unable to execute command. Code: {}, ", exec);
    }

    info!("[db::add_to_db()]: execute function exited with: {:?}", exec);

    Ok(())
}

pub fn migrate_to_host_list(macmap: &HashMap<MacAddr, config::HostInfo>, gw: &net::Gateway, result: Vec<net::Host>, arps: HashMap<Ipv4Addr, MacAddr>, pc: Ipv4Addr) -> Vec<Host> {
    let mut hosts: Vec<Host> = vec![];

    info!("[db::migrate_to_host_list()]: migrate len: {:?}", result.len());

    for host in result {
        let mut current: Option<Host> = Some(Host::new());

        let h = match &mut current {
            Some(x) => x,
            None => continue,
        };

        h.set_ip(host.address[0].addr.parse().unwrap());

        if h.ip.eq(&pc) {
            continue;
        }

        if h.ip.eq(&gw.ip){
            h.set_name("GATEWAY".to_string());
            h.set_device_name("GATEWAY".to_string());
            hosts.push(h.clone());
            continue;
        }

        match arps.get(&h.ip) {
            Some(&mac) => {
                h.set_mac(mac);

                match macmap.get(&mac) {
                    Some(n) => h.set_info(n),
                    None => {
                        h.set_name("Unknown".to_string());
                        h.set_device_name("Unknown".to_string())
                    }
                }
            }
            _ => println!("Can't find MAC for IP: {}", h.ip),
        }

        hosts.push(h.clone());
    }

    hosts
}
