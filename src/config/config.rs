extern crate dotenv;
use ethers::prelude::Address;
use std::env;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Config {
    pub chain_id: u64,
    pub chain_id_name: String,
    pub private_key: String,
    pub eth_provider_rpc: String,
    pub address_provider: Address,
    pub etherscan: String,
    pub liquidator_enabled: bool,
}

impl Default for Config {
    fn default() -> Self {
        dotenv::from_filename(".env").ok();
        dotenv::from_filename(".env.local").ok();
        let chain_id = get_env_or_throw("SPOKE_CHAIN_ID")
            .parse::<u64>()
            .expect("SPOKE_CHAIN_ID not found");
        let address_provider = Address::zero();
        let private_key = get_env_or_throw("PRIV_KEY");

        let (chain_id_name, eth_provider_rpc, etherscan) = match chain_id {
            1 => (
                "base_goerli",
                get_env_or_throw("BASE_GOERLI_PROVIDER"),
                "https://goerli.basescan.org",
            ),

            _ => {
                panic!("Unknown network!")
            }
        };

        let liquidator_enabled = if env::var("PIKE_BOT").unwrap() == "true" {
            true
        } else {
            false
        };

        Config {
            chain_id,
            chain_id_name: chain_id_name.into(),
            address_provider,
            private_key,
            eth_provider_rpc,
            etherscan: etherscan.into(),
            liquidator_enabled,
        }
    }
}

fn get_env_or_throw(env: &str) -> String {
    env::var(env).expect(format!("Not found {}", env).as_str())
}
