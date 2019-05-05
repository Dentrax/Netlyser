use clap::*;
use regex::Regex;

use crate::error;

use std::{
    ffi::OsString,
};

#[derive(Clone, Debug)]
pub struct Args {
    pub quiet: bool,
    pub verbose: u64,
    pub network: String,
    pub path_config: String,
    pub path_output: String
}

pub fn get_args() -> error::Result<Args> {
    get_args_impl(None::<&[&str]>)
}

//Ref: https://github.com/watchexec/watchexec/blob/master/src/cli.rs
fn get_args_impl<I, T>(from: Option<I>) -> error::Result<Args>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let app = App::new("netlyser")
        .version(crate_version!())
        .about("Network observing tool for your sweet home")
        .arg(Arg::with_name("quiet")
             .help("Quiet mode (Overrides verbose mode)")
             .short("q")
             .long("quiet")
             .takes_value(false)
             .multiple(false)
             .required(false))

        .arg(Arg::with_name("verbose")
             .help("Verbose mode (Warn: -v, Info: -vv, Debug: -vvv, Trace: -vvvv)")
             .short("v")
             .long("verbose")
             .takes_value(false)
             .multiple(true)
             .required(false))

        .arg(Arg::with_name("network")
             .help("CIDR notation of the network you want to scan, e.g.'192.168.1.0/24'")
             .short("n")
             .long("network")
             .takes_value(true)
             .multiple(false)
             .required(true)
             .validator(is_ipmask))

        .arg(Arg::with_name("config-file")
             .help("Input filepath for the config file, e.g '~/.config/netlyser.conf")
             .short("c")
             .long("config-file")
             .takes_value(true)
             .multiple(false)
             .required(true)
             .validator(is_file_yaml))

        .arg(Arg::with_name("output-path")
             .help("Output filepath for the SQLite database file, e.g. '/var/log/sweet-home.db'")
             .short("o")
             .long("output-path")
             .takes_value(true)
             .multiple(false)
             .required(true));

    let args = match from {
        None => app.get_matches(),
        Some(i) => app.get_matches_from(i),
    };

    let network: String = value_t!(args.value_of("network"), String)?;
    let path_config: String = value_t!(args.value_of("config-file"), String)?;
    let path_output: String = value_t!(args.value_of("output-path"), String)?;

    match args.occurrences_of("verbose") {
        1 => println!("Verbose: Warn"),
        2 => println!("Verbose: Info"),
        3 => println!("Verbose: Debug"),
        4 => println!("Verbose: Trace"),
        5 | _ => {},
    }

    Ok(Args {
        quiet: args.is_present("quiet"),
        verbose: args.occurrences_of("verbose"),
        network: network,
        path_config: path_config,
        path_output: path_output
    })

}

fn is_ipmask(val: String) -> std::result::Result<(), String> {
    lazy_static! {
        static ref RGX_IPMASK: Regex = Regex::new(r"^((?:[0-9]{1,3}\.){3}[0-9]{1,3}/[1-3]?[0-9])$").unwrap();
    }

    if RGX_IPMASK.is_match(&val) {
        Ok(())
    } else {
        Err(String::from("the network format must be like '196.168.1.0/24'"))
    }
}

fn is_file_yaml(val: String) -> std::result::Result<(), String> {
    lazy_static! {
        static ref RGX_FILE_YAML: Regex = Regex::new(r"^((.+?)/)?([\w]+\.yaml)$").unwrap();
    }

    if RGX_FILE_YAML.is_match(&val) {
        Ok(())
    } else {
        Err(String::from("the config file format must be like '.../path/to/config.yaml'"))
    }
}
