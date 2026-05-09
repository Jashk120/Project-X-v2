use crate::modules::did::dto::CreateDidResponse;
use anyhow::{Context, Result, anyhow};
use hiero_did_core::{KeysUtility, did::Network};
use hiero_did_method::parse_did;
use hiero_did_registrar::create::create_did;
use hiero_did_resolver::{DidDocumentBuilder, MirrorNodeClient};
use hiero_sdk::{AccountId, Client, PrivateKey};
use serde_json::Value;
use std::env;
use std::str::FromStr;

fn setup_client() -> Result<Client> {
    let account_id = env::var("HEDERA_ACCOUNT_ID").context("HEDERA_ACCOUNT_ID not set")?;
    let private_key = env::var("HEDERA_PRIVATE_KEY").context("HEDERA_PRIVATE_KEY not set")?;

    let client = Client::for_testnet();
    client.set_operator(
        AccountId::from_str(&account_id).context("Invalid HEDERA_ACCOUNT_ID")?,
        PrivateKey::from_str_der(&private_key).context("Invalid HEDERA_PRIVATE_KEY (expected DER)")?,
    );

    Ok(client)
}

fn parse_network(s: Option<&str>) -> Result<Network> {
    match s.unwrap_or("testnet") {
        "testnet" => Ok(Network::Testnet),
        "mainnet" => Ok(Network::Mainnet),
        other => Err(anyhow!("unsupported network: {other}; use testnet or mainnet")),
    }
}

pub async fn create(network: Option<String>, controller: Option<String>) -> Result<CreateDidResponse> {
    let network = parse_network(network.as_deref())?;
    let client = setup_client()?;

    let created = create_did(&client, network, controller).await
        .map_err(|e| anyhow!("create_did failed: {e}"))?;

    Ok(CreateDidResponse {
        did: created.did.to_string(),
        topic_id: created.did.topic_id.clone(),
        public_key_base58: KeysUtility::from_bytes(created.public_key_bytes).to_base58(),
        private_key_base58: KeysUtility::from_bytes(created.private_key_bytes).to_base58(),
    })
}

pub async fn resolve(did: String) -> Result<Value> {
    let parsed = parse_did(&did).map_err(|e| anyhow!("invalid did: {e}"))?;

    let mirror = match parsed.network {
        Network::Testnet => MirrorNodeClient::for_testnet(),
        Network::Mainnet => MirrorNodeClient::for_mainnet(),
    };

    let messages = mirror
        .get_topic_messages(&parsed.topic_id)
        .await
        .map_err(|e| anyhow!("failed fetching topic messages: {e}"))?;

    let resolution = DidDocumentBuilder::from(messages)
        .resolve(&parsed)
        .await
        .map_err(|e| anyhow!("resolve failed: {e}"))?;

    serde_json::to_value(resolution).context("failed to serialize resolution")
}
