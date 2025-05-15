use std::str::FromStr;

use alloy::{
    eips::eip7702::Authorization,
    primitives::{address, U256, Bytes, Address},
    providers::{Provider, ProviderBuilder},
    network::{TransactionBuilder, TransactionBuilder7702},
    rpc::types::TransactionRequest,
    signers::{local::PrivateKeySigner, SignerSync}, 
    transports::http::reqwest::Url,
    sol,
};
use eyre::Result;

// Codegen from ABI file to interact with the contract.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IWalletCore,
    "contracts/abi/IWalletCore.json"
);

pub async fn make_transaction() -> Result<()> {
        // Load environment variables from .env file
        dotenv::dotenv().ok();

        // Get environment variables
        let rpc_url_str = std::env::var("SEPOLIA_RPC_URL")
            .expect("SEPOLIA_RPC_URL must be set in .env file");
        let rpc_url = Url::parse(&rpc_url_str).expect("Invalid RPC URL");
        let alice_private_key = std::env::var("ALICE_PRIVATE_KEY")
            .expect("ALICE_PRIVATE_KEY must be set in .env file");
        let bob_private_key = std::env::var("BOB_PRIVATE_KEY")
            .expect("BOB_PRIVATE_KEY must be set in .env file");
     
        // Create two users, Alice and Bob
        let alice: PrivateKeySigner = alice_private_key.parse()?;
        println!("Alice address and balance: {}", alice.address());
        let bob: PrivateKeySigner = bob_private_key.parse()?;
        println!("Bob address and balance: {}", bob.address());
    
        let provider = ProviderBuilder::new().wallet(alice.clone()).on_http(rpc_url.clone());
        println!("Alice's balance: {:?}", provider.get_balance(alice.address()).await?);
        println!("Bob's balance: {:?}", provider.get_balance(bob.address()).await?);

        // Create a contract instance.
        let wallet_call_contract = IWalletCore::new(address!("0x80296FF8D1ED46f8e3C7992664D13B833504c2Bb"), provider.clone());
    
       
        // Create an authorization object for Alice to sign
        let authorization = Authorization {
            chain_id: U256::from(provider.get_chain_id().await?), // Sepolia chain ID
            address: *wallet_call_contract.address(),
            nonce: provider.get_transaction_count(alice.address()).await?,
        };
    
        // Alice signs the authorization
        let signature = alice.sign_hash_sync(&authorization.signature_hash())?;
        let signed_authorization = authorization.clone().into_signed(signature);
    
        let receiver_address = std::env::var("RECEIVER_ADDRESS").expect("RECEIVER_ADDRESS must be set in .env file");
        let receiver = Address::from_str(&receiver_address).unwrap();
        println!("random account's balance: {:?}", provider.get_balance(receiver).await?);
    
        // Prepare the initialize call
        let initialize_call = wallet_call_contract.initialize();
        let initialize_data = initialize_call.calldata().to_vec();
    
        let tx_init = TransactionRequest::default()
            .with_to(alice.address())
            .with_authorization_list(vec![signed_authorization.clone()])
            .with_input(initialize_data);
    
        // Send the initialize transaction
        let pending_tx_init = provider.send_transaction(tx_init).await?;
        println!("Pending initialize transaction... {}", pending_tx_init.tx_hash());
        let receipt_init = pending_tx_init.get_receipt().await?;
        println!("Initialize included in block {}", receipt_init.block_number.expect("Failed to get block number"));


        //  // Create an authorization object for Alice to sign
        // let authorization_for_transfer = Authorization {
        //     chain_id: U256::from(provider.get_chain_id().await?), // Sepolia chain ID
        //     address: *wallet_call_contract.address(),
        //     nonce: provider.get_transaction_count(alice.address()).await?,
        // };
    
        // // Alice signs the authorization
        // let signature_for_transfer = alice.sign_hash_sync(&authorization_for_transfer.signature_hash())?;
        // let signed_authorization_for_transfer = authorization_for_transfer.clone().into_signed(signature_for_transfer);   
        // let calls_following = vec![
        //     IWalletCore::Call {
        //         target: receiver,
        //         value: U256::from(1000000000000u64),
        //         data: Bytes::new(),
        //     },
        //     IWalletCore::Call {
        //         target: receiver,
        //         value: U256::from(2000000000000u64),
        //         data: Bytes::new(),
        //     }
        // ];
    
        // // Build the transaction for the following calls
        // let call_following = wallet_call_contract.executeFromSelf(calls_following);
        // let calldata_following = call_following.calldata().to_owned();
    
        // let tx_following = TransactionRequest::default()
        //     .with_to(alice.address())
        //     .with_authorization_list(vec![signed_authorization_for_transfer])
        //     .with_input(calldata_following);
       
        // // Send the following transaction
        // let pending_tx_following = provider.send_transaction(tx_following).await?;
        // println!("Pending following transaction... {}", pending_tx_following.tx_hash());
        // let receipt_following = pending_tx_following.get_receipt().await?;
        // println!("Following calls included in block {}", receipt_following.block_number.expect("Failed to get block number"));
    
        // println!("random account's balance: {:?}", provider.get_balance(receiver).await?);
    
        Ok(())
}