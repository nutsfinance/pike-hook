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
use ethers::prelude::{Http, Provider, SignerMiddleware, Wallet};
use serde_json::Value;

use crate::bindings::PikeEvents;
use crate::config::Config;
use crate::pike_service::Account;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
enum ContractKeyKind {
    Market,
    PriceFeed,
    InterestRateModel,
    Oracle,
    Auditor,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct ContractKey {
    address: Address,
    kind: ContractKeyKind,
}

pub struct PikeService {
    provider: Provider<Http>,
    client: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    contracts_to_listen: HashMap<ContractKey, Address>,
    last_block_synced: U64,
    oracle: PikeOracle<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    markets: HashMap<Address, Abigen>,
    borrowers: HashMap<Address, Account>,
}

impl PikeService {
    pub fn new(
        client: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
        provider: Provider<Http>,
        config: Config,
    ) -> PikeService {
        PikeService {
            provider,
            client: Arc::clone(&client),
            contracts_to_listen: HashMap::new(),
            last_block_synced: U64::from(block_number) - 1u64,
            oracle: PikeOracle::new(Address::zero(), Arc::clone(&client)),
            markets: HashMap::<Address, FixedLender>::new(),
            borrowers: HashMap::new(),
        }
    }
}
