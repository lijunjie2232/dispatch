# dispatch

Forked from [https://github.com/alexkirsz/dispatch)](https://github.com/alexkirsz/dispatch)

A SOCKS proxy that balances traffic between network interfaces.

_Works on macOS, Windows, and Linux._

This is a Rust rewrite of [dispatch-proxy](https://github.com/alexkirsz/dispatch-proxy), originally written in CoffeeScript and targeting Node.js.

## Quick links

- [dispatch](#dispatch)
  - [Quick links](#quick-links)
  - [Installation](#installation)
    - [From pre-built binaries](#from-pre-built-binaries)
  - [Rationale](#rationale)
  - [Use cases](#use-cases)
  - [Usage](#usage)
    - [Configuration File](#configuration-file)
  - [Examples](#examples)
  - [How It Works](#how-it-works)
      - [License](#license)

## Installation

### From pre-built binaries

You can download pre-built binaries for macOS, Windows, and Linux from the [releases page](https://github.com/lijiunjie2232/dispatch/releases).

## Rationale

You often find yourself with multiple unused internet connections—be it 5G mobile hotspot or a free Wi-Fi network—that your system won't let you use alongside your primary one.

For instance, my first student residence used to provide me with cabled and wireless internet accesses. Both were separately capped at a bandwidth 1,200kB/s. My 3G mobile internet access provided me with an additional 400kB/s. Combining all of these with dispatch and a download manager resulted in a 2,800kB/s effective bandwidth!

## Use cases

The possibilities are endless:

- Use it with a download manager or a BitTorrent client, combining multiple connections' bandwidth when downloading single files;
- Combine as many interfaces as you have access to into a single load-balanced interface;
- Run different apps on separate interfaces with multiple proxies (e.g. for balancing download/upload);
- Create a hotspot proxy at home that connects through Ethernet and your 5G card for all your mobile devices;
- etc.

## Usage

```
❯ ./target/release/dispatch -h
[Fork] A SOCKS proxy that balances traffic between network interfaces.

Usage: dispatch [OPTIONS] <COMMAND>

Commands:
  list     Lists all available network interfaces
  balance  Starts the SOCKS proxy server
  help     Print this message or the help of the given subcommand(s)

Options:
  -d, --debug    Write debug logs to stdout instead of a file
  -h, --help     Print help
  -V, --version  Print version
```

```
❯ ./target/release/dispatch balance -h
Starts the SOCKS proxy server

Usage: dispatch balance [OPTIONS] --config <CONFIG> [ADDRESSES]...

Arguments:
  [ADDRESSES]...  The network interface IP addresses to dispatch to, in the form of <address>[/priority]

Options:
  -i, --ip <IP>          Which IP to accept connections from [default: 127.0.0.1]
  -p, --port <PORT>      Which port to listen to for connections [default: 1080]
  -c, --config <CONFIG>  
  -h, --help             Print help
```

### Configuration File

A configuration file could be used to specify network interface addresses instead of providing them directly on the command line. The configuration file is in YAML format, with the default path being [./dispatch.yaml](file:///home/li/route/dispatch/dispatch.yaml).

Configuration file example:

```yaml
ip: 127.0.0.1
port: 1080
# Example configuration file
addresses:
  - "192.168.1.100/3"  # High priority
  - "192.168.1.101"    # Default priority (1)
  - "10.0.0.1/2"       # Medium priority
```

Creating your own configuration file by copying the [dispatch.yaml.example](file:///home/li/route/dispatch/dispatch.yaml.example) file and modifying it:

```bash
cp dispatch.yaml.example dispatch.yaml
# Edit the dispatch.yaml file to add your network interfaces
```

Start the proxy server using the configuration file:

```bash
dispatch balance --config ./dispatch.yaml
```

If both command-line addresses and a configuration file are provided, the command-line addresses will take precedence.

## Examples

```
$ dispatch list
```

Lists all available network interfaces.

```
$ dispatch balance 10.0.0.0 fdaa:bbcc:ddee:0:1:2:3:4
```

Dispatch incoming connections to local addresses `10.0.0.0` and `fdaa:bbcc:ddee:0:1:2:3:4`.

```
$ dispatch balance 10.0.0.0/7 10.0.0.1/3
```

Dispatch incoming connections to `10.0.0.0` 7 times out of 10 and to `10.0.0.1` 3 times out of 10.

```
❯ ./target/release/dispatch balance -c ./dispatch.yaml
SOCKS proxy started on 127.0.0.1:8081
...
```

Dispatch incoming connections to the network interfaces specified in the configuration file.


## How It Works

Whenever the SOCKS proxy server receives an connection request to an address or domain, it selects one of the provided local addresses using the [Weighted Round Robin](https://en.wikipedia.org/wiki/Weighted_round_robin) algorithm. All further connection traffic will then go through the interface corresponding to the selected local address.

**Beware:** If the requested address or domain resolves to an IPv4 (resp. IPv6) address, an IPv4 (resp. IPv6) local address must be provided.

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
