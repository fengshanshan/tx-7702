use std::str::FromStr;
use alloy::{
    primitives::{Address, address},
    transports::http::reqwest::Url,
    signers::local::PrivateKeySigner,
};
use eyre::Result;

/// Configuration for the EIP-7702 demo
#[derive(Debug, Clone)]
pub struct Config {
    pub rpc_url: Url,
    pub alice_signer: PrivateKeySigner,
    pub bob_signer: PrivateKeySigner,
    pub receiver_address: Address,
    pub wallet_core_address: Address,
    pub token_address: Address,
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self> {
        dotenv::dotenv().ok();

        let rpc_url_str = std::env::var("SEPOLIA_RPC_URL")
            .expect("SEPOLIA_RPC_URL must be set in .env file");
        let rpc_url = Url::parse(&rpc_url_str).expect("Invalid RPC URL");

        let alice_private_key = std::env::var("ALICE_PRIVATE_KEY")
            .expect("ALICE_PRIVATE_KEY must be set in .env file");
        let alice_signer: PrivateKeySigner = alice_private_key.parse()?;

        let bob_private_key = std::env::var("BOB_PRIVATE_KEY")
            .expect("BOB_PRIVATE_KEY must be set in .env file");
        let bob_signer: PrivateKeySigner = bob_private_key.parse()?;

        let receiver_address_str = std::env::var("RECEIVER_ADDRESS")
            .expect("RECEIVER_ADDRESS must be set in .env file");
        let receiver_address = Address::from_str(&receiver_address_str)?;

        // Default addresses - these could also be environment variables
        let wallet_core_address = address!("0x80296FF8D1ED46f8e3C7992664D13B833504c2Bb");

        // use link token on sepolia as example
        let token_address = address!("0x779877A7B0D9E8603169DdbD7836e478b4624789");

        Ok(Config {
            rpc_url,
            alice_signer,
            bob_signer,
            receiver_address,
            wallet_core_address,
            token_address,
        })
    }
} 