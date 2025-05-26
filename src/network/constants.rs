//! Network-specific constants for Bitcoin networks
//!
//! Contains magic bytes, default ports, DNS seeds, and protocol constants
//! for mainnet, testnet, and regtest networks.

use crate::config::Network;

/// Network-specific constants for Bitcoin protocol
#[derive(Debug, Clone)]
pub struct NetworkConstants {
    /// Magic bytes for network message identification
    pub magic_bytes: [u8; 4],
    /// Default P2P port for the network
    pub default_port: u16,
    /// DNS seed nodes for peer discovery
    pub dns_seeds: Vec<&'static str>,
    /// Bitcoin protocol version
    pub protocol_version: u32,
    /// Node services bitfield
    pub services: u64,
    /// Network name for logging/identification
    pub name: &'static str,
}

impl NetworkConstants {
    /// Get network constants for the specified network
    pub fn for_network(network: &Network) -> Self {
        match network {
            Network::Mainnet => Self::mainnet(),
            Network::Testnet => Self::testnet(),
            Network::Regtest => Self::regtest(),
        }
    }

    /// Mainnet network constants
    pub fn mainnet() -> Self {
        Self {
            magic_bytes: [0xf9, 0xbe, 0xb4, 0xd9],
            default_port: 8333,
            dns_seeds: vec![
                "seed.bitcoin.sipa.be",
                "dnsseed.bluematt.me",
                "dnsseed.bitcoin.dashjr.org",
                "seed.bitcoinstats.com",
                "seed.bitcoin.jonasschnelli.ch",
                "seed.btc.petertodd.org",
                "seed.bitcoin.sprovoost.nl",
                "dnsseed.emzy.de",
            ],
            protocol_version: 70016,
            services: 0x01, // NODE_NETWORK
            name: "mainnet",
        }
    }

    /// Testnet network constants
    pub fn testnet() -> Self {
        Self {
            magic_bytes: [0x0b, 0x11, 0x09, 0x07],
            default_port: 18333,
            dns_seeds: vec![
                "testnet-seed.bitcoin.jonasschnelli.ch",
                "seed.tbtc.petertodd.org",
                "seed.testnet.bitcoin.sprovoost.nl",
                "testnet-seed.bluematt.me",
            ],
            protocol_version: 70016,
            services: 0x01, // NODE_NETWORK
            name: "testnet",
        }
    }

    /// Regtest network constants
    pub fn regtest() -> Self {
        Self {
            magic_bytes: [0xfa, 0xbf, 0xb5, 0xda],
            default_port: 18444,
            dns_seeds: vec![], // No DNS seeds for regtest
            protocol_version: 70016,
            services: 0x01, // NODE_NETWORK
            name: "regtest",
        }
    }

    /// Check if this network uses DNS seed discovery
    pub fn uses_dns_seeds(&self) -> bool {
        !self.dns_seeds.is_empty()
    }

    /// Get localhost peers for regtest network
    pub fn localhost_peers(&self) -> Vec<String> {
        match self.name {
            "regtest" => vec![
                format!("127.0.0.1:{}", self.default_port),
                format!("localhost:{}", self.default_port),
            ],
            _ => vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mainnet_constants() {
        let constants = NetworkConstants::mainnet();
        assert_eq!(constants.magic_bytes, [0xf9, 0xbe, 0xb4, 0xd9]);
        assert_eq!(constants.default_port, 8333);
        assert!(!constants.dns_seeds.is_empty());
        assert!(constants.uses_dns_seeds());
        assert_eq!(constants.name, "mainnet");
    }

    #[test]
    fn test_testnet_constants() {
        let constants = NetworkConstants::testnet();
        assert_eq!(constants.magic_bytes, [0x0b, 0x11, 0x09, 0x07]);
        assert_eq!(constants.default_port, 18333);
        assert!(!constants.dns_seeds.is_empty());
        assert!(constants.uses_dns_seeds());
        assert_eq!(constants.name, "testnet");
    }

    #[test]
    fn test_regtest_constants() {
        let constants = NetworkConstants::regtest();
        assert_eq!(constants.magic_bytes, [0xfa, 0xbf, 0xb5, 0xda]);
        assert_eq!(constants.default_port, 18444);
        assert!(constants.dns_seeds.is_empty());
        assert!(!constants.uses_dns_seeds());
        assert_eq!(constants.name, "regtest");
        assert!(!constants.localhost_peers().is_empty());
    }

    #[test]
    fn test_for_network() {
        let mainnet = NetworkConstants::for_network(&Network::Mainnet);
        assert_eq!(mainnet.name, "mainnet");

        let testnet = NetworkConstants::for_network(&Network::Testnet);
        assert_eq!(testnet.name, "testnet");

        let regtest = NetworkConstants::for_network(&Network::Regtest);
        assert_eq!(regtest.name, "regtest");
    }

    #[test]
    fn test_localhost_peers() {
        let regtest = NetworkConstants::regtest();
        let peers = regtest.localhost_peers();
        assert!(peers.contains(&"127.0.0.1:18444".to_string()));
        assert!(peers.contains(&"localhost:18444".to_string()));

        let mainnet = NetworkConstants::mainnet();
        assert!(mainnet.localhost_peers().is_empty());
    }
}