use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;
use std::sync::Arc;
use std::{thread, time};

use anyhow::{Error, Result};
use ethers::abi::{Abi, RawLog};
use ethers::core::types::Filter;
use ethers::core::types::{Address, U256, U64};
use ethers::prelude::k256::ecdsa::SigningKey;
use ethers::prelude::{
    ContractError, EthLogDecode, Http, LogMeta, Middleware, Multicall, Provider, SignerMiddleware,
    StreamExt, Wallet,
};
use serde_json::Value;

use crate::bindings::PikeEvents;
use crate::config::Config;
use crate::credit_service::{Account, Auditor, FixedLender, Market};

use super::{MarketAccount, PikeOracle, Previewer};

pub struct CreditService {
    client: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    last_block_synced: U64,
    oracle: PikeOracle<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    markets: HashMap<Address, FixedLender>,
    borrowers: HashMap<Address, Account>,
}

impl CreditService {
    pub fn new(
        client: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
        provider: Provider<Http>,
        config: Config,
    ) -> CreditService {
        CreditService {
            last_block_synced: U64::from(block_number) - 1u64,
            oracle: PikeOracle::new(Address::zero(), Arc::clone(&client)),
            markets: HashMap::<Address, FixedLender>::new(),
            borrowers: HashMap::new(),
        }
    }

    pub async fn launch(&mut self) -> Result<(), Error> {
        let markets = self.auditor.get_all_markets().call().await?;
        for market in markets {
            self.markets
                .entry(market)
                .or_insert_with_key(|key| FixedLender::new(*key));
        }

        let interest_rate_model_address = Address::from_str("")?;
        self.contracts_to_listen
            .entry(ContractKey {
                address: interest_rate_model_address,
                kind: ContractKeyKind::InterestRateModel,
            })
            .or_insert_with_key(|key| key.address);

        self.update().await?;

        let watcher = self.client.clone();
        let mut on_block = watcher
            .watch_blocks()
            .await
            .map_err(
                ContractError::MiddlewareError::<
                    SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
                >,
            )
            .unwrap()
            .stream();
        while on_block.next().await.is_some() {
            match self.update().await {
                Err(e) => {
                    // TODO
                }
                _ => {}
            }
            let delay = time::Duration::from_secs(20);
            thread::sleep(delay);
        }
        Ok(())
    }

    async fn handle_events(&mut self, logs: Vec<(PikeEvents, LogMeta)>) -> Result<(), Error> {
        let mut block = U64::from(0u64);
        let mut block_timestamp = U256::zero();
        for (event, meta) in logs {
            match event {
                // TODO Set up events here
                _ => {
                    // TODO not handled case
                }
            }
        }
        Ok(())
    }
}
