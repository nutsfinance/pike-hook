use std::panic;
use std::process;
use std::sync::Arc;

use ethers::prelude::k256::ecdsa::SigningKey;
use ethers::prelude::{Provider, Signer, SignerMiddleware, Wallet, WalletError};
use ethers::providers::Http;

mod bindings;
mod config;
mod pike_service;

use crate::config::Config;

async fn create_client(
    config: &Config,
) -> (
    Arc<SignerMiddleware<Provider<Ws>, Wallet<SigningKey>>>,
    Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
) {
    let provider_ws = loop {
        let provider_result = Provider::<Ws>::connect(config.rpc_provider.clone()).await;
        if let Ok(provider) = provider_result {
            break provider;
        }
    };
    let provider_https = loop {
        let provider_result = Provider::<Http>::try_from(config.rpc_provider_relayer.clone());
        if let Ok(provider) = provider_result {
            break provider;
        }
    };
    let wallet = config.wallet.clone().with_chain_id(config.chain_id);
    (
        Arc::new(SignerMiddleware::new(provider_ws, wallet.clone())),
        Arc::new(SignerMiddleware::new(provider_https, wallet)),
    )
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    // TODO add more metrics

    let config = Config::default();
    let provider = Provider::<Http>::try_from(config.eth_provider_rpc.clone())?;

    let wallet = config
        .private_key
        .parse::<LocalWallet>()
        .context("Failed to parse private key")?;
    let wallet = wallet.with_chain_id(config.chain_id);

    let client: SignerMiddleware<Provider<Http>, Wallet<SigningKey>> =
        SignerMiddleware::new(provider.clone(), wallet);

    let client = Arc::new(client);

    let mut pike_bot = PikeService::new(client, provider, config);

    pike_bot
        .launch()
        .await
        .context("Failed to launch Pike service")?;

    Ok(())
}
