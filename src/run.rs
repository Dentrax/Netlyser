use crate::cli::Args;
use crate::error::Result;

use crate::config;
use crate::db;
use crate::net;

use crate::log;

use std::{thread, time};

use std::io::Write;

use std::process::exit;
use std::process::Command;

pub enum ExitCodes {
    RootRequired = 1,
    ConfigFileDoesNotExist = 2,
    ConfigInvalid = 3,
    NmapNotInstalled = 4,
    NmapRunError = 5,
    ResultWriteError = 6,
    RootCheckError = 7,
    RootCommandError = 8,
    HostnameNotFound = 9,
    HostnameRunError = 10,
    HostnameParseError = 11,
    DBCreateError = 12,
}

//Ref: https://github.com/max-wittig/bernard/blob/master/src/main.rs#L120
fn is_root() -> bool {
    let output = match Command::new("id").arg("-u").output() {
        Ok(r) => r,
        Err(_) => {
            exit(ExitCodes::RootCommandError as i32);
        }
    };
    let id: i32 = match String::from_utf8_lossy(&output.stdout)
        .trim()
        .parse::<i32>()
    {
        Ok(r) => r,
        Err(_) => {
            exit(ExitCodes::RootCheckError as i32);
        }
    };
    id == 0
}

fn init_logger(verbose: u64, quiet: bool) {
    let mut log_builder = env_logger::Builder::new();

    let mut level = match verbose {
        1 => log::LevelFilter::Warn,
        2 => log::LevelFilter::Info,
        3 => log::LevelFilter::Debug,
        4 => log::LevelFilter::Trace,
        r if r > 4 as u64 => log::LevelFilter::Trace,
        _ => log::LevelFilter::Error,
    };

    if quiet {
        level = log::LevelFilter::Off
    }

    log_builder
        .format(|buf, r| writeln!(buf, "*** {}", r.args()))
        .filter(None, level)
        .init();
}

pub fn run(args: Args) -> Result<()> {
    init_logger(args.verbose, args.quiet);

    let is_root = is_root();

    if !is_root {
        println!("Info: You can escalate privileges via 'sudo' to get more accurate results!")
    }

    let mut config = match config::get_config(&args.path_config) {
        Ok(r) => r,
        Err(config::ErrorType::ParseError) => {
            error!("Config is invalid. Please make sure that only valid MAC addresses are used!");
            exit(ExitCodes::ConfigInvalid as i32);
        }
        Err(config::ErrorType::SerdeError) => {
            error!("Config file does not in YAML format!");
            exit(ExitCodes::ConfigInvalid as i32);
        }
        Err(config::ErrorType::ReadError) => {
            error!("Config file does not exist at the given location!");
            exit(ExitCodes::ConfigFileDoesNotExist as i32);
        }
    };

    info!("Config file loaded successfully!");

    config.is_root = is_root;

    let conf = config.clone();
    let duration = time::Duration::from_millis(config.general.interval);

    let macmap = config::get_mac_info_map(config);
    let gateway = net::get_gateway();

    let ipmask = args.network.clone();
    let hostname = net::get_hostname_addr().unwrap();

    let mut olds: Vec<db::Host> = vec![];

    let scanner: std::thread::JoinHandle<()> = std::thread::spawn(move || loop {
        let res_nmap = net::do_scan_nmap(&ipmask, 5);
        let res_arp = net::do_scan_arp();

        let news = db::migrate_to_host_list(&macmap, &gateway, res_nmap, res_arp, hostname);

        db::get_notifies(&olds, &news, &conf, &args.path_output);

        olds = news.clone();

        thread::sleep(duration);
    });

    let _s = scanner.join();

    Ok(())
}
