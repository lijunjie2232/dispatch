use serde::Deserialize;
use std::fs;
use std::net::IpAddr;

use crate::dispatcher::RawWeightedAddress;
use eyre::Result;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub addresses: Vec<String>,
    pub ip: Option<String>,
    pub port: Option<u16>,
}

/// 从配置文件读取完整配置
pub fn load_config(config_path: &str) -> Result<Config> {
    let config_content = fs::read_to_string(config_path)
        .map_err(|e| eyre::eyre!("Failed to read config file {}: {}", config_path, e))?;

    let config: Config = serde_yaml::from_str(&config_content)
        .map_err(|e| eyre::eyre!("Failed to parse config file {}: {}", config_path, e))?;

    Ok(config)
}

/// 从配置文件读取地址
pub fn load_addresses_from_config(config_path: &str) -> Result<Vec<RawWeightedAddress>> {
    let config = load_config(config_path)?;

    // 将配置文件中的字符串地址转换为RawWeightedAddress
    let mut addresses = Vec::new();
    for addr_str in config.addresses {
        addresses.push(
            <RawWeightedAddress as std::str::FromStr>::from_str(&addr_str)
                .map_err(|e| eyre::eyre!("Invalid address '{}' in config file: {}", addr_str, e))?,
        );
    }

    Ok(addresses)
}

/// 从配置文件读取IP地址
pub fn load_ip_from_config(config_path: &str) -> Result<Option<IpAddr>> {
    let config = load_config(config_path)?;

    match config.ip {
        Some(ip_str) => {
            let ip = ip_str.parse().map_err(|e| {
                eyre::eyre!("Invalid IP address '{}' in config file: {}", ip_str, e)
            })?;
            Ok(Some(ip))
        }
        None => Ok(None),
    }
}

/// 从配置文件读取端口
pub fn load_port_from_config(config_path: &str) -> Result<Option<u16>> {
    let config = load_config(config_path)?;
    Ok(config.port)
}
