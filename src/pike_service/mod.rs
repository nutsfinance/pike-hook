pub mod borrow;
pub mod service;

pub use borrow::*;
use ethers::prelude::abigen;

abigen!(
    Hub,
    "lib/pike-universal/deployments/base/HubContract.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

abigen!(
    Spoke,
    "lib/pike-universal/deployments/base/SpokeContract.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

abigen!(
    PythOracle,
    "lib/@pyth-network/abis/PythOracle.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

abigen!(
    ChainlinkOracle,
    "lib/chainlink/contracts/abi/v0.8/AggregatorV2V3Interface.json",
    event_derives(serde::Deserialize, serde::Serialize)
);
