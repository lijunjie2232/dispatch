use std::{net::IpAddr, str::FromStr};

use clap::Parser;
use debug::LogStrategy;
use dispatcher::{RawWeightedAddress, WeightedAddress};
use eyre::Result;
use tracing::{info, warn};

mod config;
mod debug;
mod dispatcher;
mod list;
mod net;
mod server;
mod socks;

/// A proxy that balances traffic between multiple internet connections
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Opt {
    /// Write debug logs to stdout instead of a file
    #[arg(short, long)]
    debug: bool,
    #[command(subcommand)]
    command: Command,
}

#[derive(Parser, Debug)]
enum Command {
    /// Lists all available network interfaces
    List,
    /// Starts the SOCKS proxy server
    Balance {
        /// Which IP to accept connections from
        #[arg(default_value = "127.0.0.1", long, short = 'i')]
        ip: IpAddr,
        /// Which port to listen to for connections
        #[arg(default_value = "1080", long, short = 'p')]
        port: u16,
        #[arg(long, short = 'c')]
        config: String,
        /// The network interface IP addresses to dispatch to, in the form of <address>[/priority]
        #[arg(value_parser = RawWeightedAddress::from_str)]
        addresses: Vec<RawWeightedAddress>,
    },
}

fn main() -> Result<()> {
    let opt = Opt::parse();

    let _guard = debug::install(if opt.debug {
        LogStrategy::Stdout
    } else {
        LogStrategy::File
    })?;

    match opt.command {
        Command::List => list::list(),
        Command::Balance {
            mut ip,
            mut port,
            config,
            mut addresses,
        } => {
            // 如果没有通过命令行提供addresses，则尝试从配置文件读取
            if !config.is_empty() {
                info!("Using config file: {}", config);

                addresses = config::load_addresses_from_config(&config)?;

                if let Some(config_ip) = config::load_ip_from_config(&config)? {
                    ip = config_ip;
                    warn!("Using IP from config file: {}", ip);
                }

                if let Some(config_port) = config::load_port_from_config(&config)? {
                    port = config_port;
                    warn!("Using port from config file: {}", port);
                }
            } else {
                info!("No config file provided, parsing sys args..");
            }
            let addresses = WeightedAddress::resolve(addresses)?;

            server::server(ip, port, addresses)?
        }
    }

    Ok(())
}
