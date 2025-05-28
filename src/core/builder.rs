use alloy::{
    eips::eip7702::{Authorization, SignedAuthorization},
    network::{TransactionBuilder, TransactionBuilder7702},
    primitives::{Address, Bytes, FixedBytes, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::{local::PrivateKeySigner, SignerSync},
};
use eyre::Result;

use crate::core::{
    config::Config,
    contracts::{Call, IWalletCore, IStorage, ERC20},
    types::Addresses,
};

/// EIP-7702 transaction builder and utilities
pub struct Eip7702Builder {
    config: Config,
}

impl Eip7702Builder {
    /// Create a new EIP-7702 builder with the given configuration
    pub fn new(config: Config) -> Eip7702Builder {
        Eip7702Builder { config }
    }

    /// Create an authorization for an EOA to use WalletCore code
    pub async fn create_authorization(
        &self,
        authorizer: &PrivateKeySigner,
    ) -> Result<alloy::eips::eip7702::SignedAuthorization> {
        let provider = ProviderBuilder::new()
            .wallet(authorizer.clone())
            .on_http(self.config.rpc_url.clone());
        let nonce = provider.get_transaction_count(authorizer.address()).await?;
        let chain_id = U256::from(provider.get_chain_id().await?);

        let authorization = Authorization {
            chain_id,
            address: self.config.wallet_core_address,
            nonce,
        };

        let signature = authorizer.sign_hash_sync(&authorization.signature_hash())?;
        Ok(authorization.into_signed(signature))
    }

    /// Get the storage address for a wallet (read-only, no signer needed)
    pub async fn get_storage_address(&self, wallet_address: Address) -> Result<Address> {
        let provider = ProviderBuilder::new()
            .on_http(self.config.rpc_url.clone());
        let wallet = IWalletCore::new(wallet_address, provider);
        Ok(wallet.getMainStorage().call().await?._0)
    }

    /// Get the current nonce from wallet storage
    pub async fn get_wallet_nonce(&self, wallet_address: Address) -> Result<U256> {
        let storage_address = self.get_storage_address(wallet_address).await?;
        let provider = ProviderBuilder::new()   
            .on_http(self.config.rpc_url.clone());
        let storage = IStorage::new(storage_address, provider);
        Ok(storage.getNonce().call().await?._0)
    }

    /// Create a token transfer call
    pub fn create_token_transfer_call(&self, caller: &PrivateKeySigner, to: Address, amount: U256) -> Call {
        let provider = ProviderBuilder::new().wallet(caller.clone())
            .on_http(self.config.rpc_url.clone());
        let erc20 = ERC20::new(self.config.token_address, provider);
        let transfer_call = erc20.transfer(to, amount);
        let calldata = transfer_call.calldata().to_owned();

        Call {
            target: self.config.token_address,
            value: U256::from(0),
            data: calldata,
        }
    }

    /// Generate validation hash for a set of calls
    pub async fn get_validation_hash(
        &self,
        wallet_address: Address,
        nonce: U256,
        calls: &[Call],
    ) -> Result<FixedBytes<32>> {
        let provider = ProviderBuilder::new()
            .on_http(self.config.rpc_url.clone());
        let wallet = IWalletCore::new(wallet_address, provider);
        Ok(wallet
            .getValidationTypedHash(nonce, calls.to_vec())
            .call()
            .await?
            ._0)
    }

    /// Sign validation data
    pub fn sign_validation_data(
        &self,
        signer: &PrivateKeySigner,
        validation_hash: &FixedBytes<32>,
    ) -> Bytes {
        let signature = signer.sign_hash_sync(validation_hash).unwrap();
        Bytes::from(signature.as_bytes())
    }

    /// Build initialize transaction
    pub fn build_initialize_transaction(
        &self,
        caller: &PrivateKeySigner,
        wallet_address: Address,
        authorization: SignedAuthorization,
    ) -> TransactionRequest {
        let provider = ProviderBuilder::new().wallet(caller.clone()).on_http(self.config.rpc_url.clone());
        let wallet = IWalletCore::new(wallet_address, provider);
        
        let init_call = wallet.initialize();
        let calldata = init_call.calldata().to_owned();

        TransactionRequest::default()
            .with_to(wallet_address)
            .with_authorization_list(vec![authorization])
            .with_input(calldata)
    }

    /// Build an EIP-7702 transaction
    pub fn build_execute_with_validator_transaction(
        &self,
        caller: &PrivateKeySigner,
        to: Address,
        authorization: SignedAuthorization,
        calls: &[Call],
        validation_data: Bytes,
    ) -> TransactionRequest {
        let provider = ProviderBuilder::new()
            .on_http(self.config.rpc_url.clone());
        let wallet = IWalletCore::new(caller.address(), provider);
        
        let execute_call = wallet.executeWithValidator(
            calls.to_vec(),
            Addresses::ECDSA_VALIDATOR,
            validation_data,
        );
        let calldata = execute_call.calldata().to_owned();

        TransactionRequest::default()
            .with_to(to)
            .with_authorization_list(vec![authorization])
            .with_input(calldata)
    }

    /// Send a transaction and wait for receipt
    pub async fn send_transaction(
        &self,
        tx: TransactionRequest,
        signer: &PrivateKeySigner,
    ) -> Result<alloy::rpc::types::TransactionReceipt> {
        let provider = ProviderBuilder::new()
            .wallet(signer.clone())
            .on_http(self.config.rpc_url.clone());
        
        let pending_tx = provider.send_transaction(tx).await?;
        println!("Transaction submitted: {}", pending_tx.tx_hash());
        
        let receipt = pending_tx.get_receipt().await?;
      
        Ok(receipt)
    }

    /// Get token balance for an address
    pub async fn get_token_balance(&self, address: Address) -> Result<U256> {
        let provider = ProviderBuilder::new()
            .on_http(self.config.rpc_url.clone());
        let erc20 = ERC20::new(self.config.token_address, provider);
        Ok(erc20.balanceOf(address).call().await?._0)
    }

    /// Get ETH balance for an address
    pub async fn get_eth_balance(&self, address: Address) -> Result<U256> {
        let provider = ProviderBuilder::new()
            .on_http(self.config.rpc_url.clone());
        Ok(provider.get_balance(address).await?)
    }

    /// Print balances for debugging
    pub async fn print_balances(&self, label: &str) -> Result<()> {
        println!("\n=== {} ===", label);
        
        let alice_eth = self.get_eth_balance(self.config.alice_signer.address()).await?;
        let bob_eth = self.get_eth_balance(self.config.bob_signer.address()).await?;
        let receiver_eth = self.get_eth_balance(self.config.receiver_address).await?;
        
        let alice_tokens = self.get_token_balance(self.config.alice_signer.address()).await?;
        let bob_tokens = self.get_token_balance(self.config.bob_signer.address()).await?;
        let receiver_tokens = self.get_token_balance(self.config.receiver_address).await?;
        
        println!("Alice - ETH: {}, Tokens: {}", alice_eth, alice_tokens);
        println!("Bob - ETH: {}, Tokens: {}", bob_eth, bob_tokens);
        println!("Receiver - ETH: {}, Tokens: {}", receiver_eth, receiver_tokens);
        
        Ok(())
    }
} 