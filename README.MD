# [Netlyser](https://github.com/Dentrax/Netlyser)


[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Open Source Love](https://badges.frapsoft.com/os/v1/open-source.png?v=103)](https://github.com/ellerbrock/open-source-badges/)
[![Build Status](https://travis-ci.org/Dentrax/Netlyser.svg?branch=master)](https://travis-ci.org/Dentrax/Netlyser)
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2FDentrax%2FNetlyser.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2FDentrax%2FNetlyser?ref=badge_shield)
[![Sourcegraph](https://img.shields.io/badge/view%20on-Sourcegraph-brightgreen.svg)](https://sourcegraph.com/github.com/Dentrax/Netlyser)

**Lightweight, a simple yet, Presence Detection Tool written in Rust, based on `nmap`, built for UNIX, made with <3**

[What It Is](#what-it-is)

[Features](#features)

[How To Use](#how-to-use)

[Installation](#installation)

[Requirements](#requirements)

[Dependencies](#dependencies)

[About](#about)

[Collaborators](#collaborators)

[Branches](#branches) 

[Copyright & Licensing](#copyright--licensing)

[Contributing](#contributing)

[Contact](#contact)

## What It Is

**Netlyser is a Rust tool for Presence Detection in your private network.**

### Screenshots

Notification
--------------------------

![Notification](https://raw.githubusercontent.com/Dentrax/Netlyser/master/screenshots/ss_0.png)
![Notification](https://raw.githubusercontent.com/Dentrax/Netlyser/master/screenshots/ss_1.png)

Database
--------------------------

![Database](https://raw.githubusercontent.com/Dentrax/Netlyser/master/screenshots/ss_2.png)

## Features

* Notify the device behaviour (connect / disconnect) using `dbus`

* Store the all of network behaviours (name, type, ip, mac, status) in `sqlite` database log file

* Mapping of the MAC addresses to labels to host devices

## How To Use

### Usage

```txt
netlyser 0.1.0
Network observing tool for your sweet home

USAGE:
    netlyser [FLAGS] [OPTIONS] --config-file <config-file> --network <network> --output-path <output-path>

FLAGS:
    -h, --help       Prints help information
    -q, --quiet      Quiet mode (Overrides verbose mode)
    -V, --version    Prints version information
    -v, --verbose    Verbose mode (Warn: -v, Info: -vv, Debug: -vvv, Trace: -vvvv)

OPTIONS:
    -c, --config-file <config-file>    Input filepath for the config file, e.g '~/.config/netlyser.conf
    -n, --network <network>            CIDR notation of the network you want to scan, e.g.'192.168.1.0/24'
    -o, --output-path <output-path>    Output filepath for the SQLite database file, e.g. '/var/log/sweet-home.db'
```

### Config

```yaml
general:
  interval: 60000
  round: 5
  
  notify_on_connect: true
  notify_on_disconnect: true

hosts:
  - mac: "01:23:45:67:89:AB"
    name: "Furkan"
    device: "iPhone"

  - mac: "BA:98:76:54:32:10"
    name: "Burak"
    device: "iPad"
```

**Interval:** `nmap` check delay in ms
**Round:** `nmap` round count to get more accurate results

## Installation

1. Clone the project to your computer and navigate to folder by executing the following command:
```
$ git clone https://github.com/Dentrax/Netlyser.git && cd ./Netlyser/
```

2. Build the all project using this command
```
$ cargo build --release
```

3. Run the Netlyser with following command
```
$ ./target/release/netlyser
```

## Known Issues

* `dbus` won't send notifications to current user session in privileged mode

* In `unprivileged` mode, it will not work as you expect (see [here](https://superuser.com/questions/887887/different-behavior-sudo-nmap-vs-just-nmap)) in every situation

* `nmap` runs several rounds to get better and more accurate results 

* Not works in Windows (hard-coded binding stuffs)

## TO-DO

- [ ] Add unit tests

- [ ] Support for Windows

- [ ] Customizable notification structure

## Requirements

* `nmap`: is used to discover hosts and services on a computer network by sending packets and analyzing the responses. [Learn More](https://nmap.org/)

* You will need a computer on which you have the rights to compile files

## Dependencies

* Please see the **[Cargo.toml](https://github.com/Dentrax/Netlyser/blob/master/Cargo.toml)** file for dependency informations

## About

Netlyser was created to serve three purposes:

**Netlyser is your painless and lovely network presence observer tool**

1. To act as a painless Presence Detection Tool

2. To provide a simplest and easiest way to observing the home network

3. There is a source for you to develop your own `nmap` based tools

## Collaborators

**Project Manager** - Furkan Türkal (GitHub: **[Dentrax](https://github.com/dentrax)**)

## Branches

We publish source for the **[Netlyser]** in single rolling branch:

The **[master branch](https://github.com/dentrax/Netlyser/tree/master)** is extensively tested and makes a great starting point. Also tracks [live changes](https://github.com/dentrax/Netlyser/commits/master) by commits.

## Copyright & Licensing

The base project code is copyrighted by Furkan 'Dentrax' Türkal and is covered by single licence.

All program code (i.e. .rs) is licensed under MIT License unless otherwise specified. Please see the **[LICENSE.md](https://github.com/Dentrax/Netlyser/blob/master/LICENSE)** file for more information.

[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2FDentrax%2FNetlyser.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2FDentrax%2FNetlyser?ref=badge_large)

**References**

While this repository is being prepared, it may have been quoted from some sources that already mentioned on specific files.

If there is an unspecified source or if you think that I made a copyright infringement, please contact with me.

## Contributing

Please check the [CONTRIBUTING.MD](CONTRIBUTING.MD) file for contribution instructions and naming guidelines.

## Contact

Netlyser was created by Furkan 'Dentrax' Türkal

 * <https://www.furkanturkal.com>
 
You can contact by URL:
    **[CONTACT](https://github.com/dentrax)**

<kbd>Best Regards</kbd>
