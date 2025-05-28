use alloy::primitives::U256;
use eyre::Result;

use crate::core::{Config, Eip7702Builder};

/// Demonstrates self-authorization pattern where Bob authorizes his own EOA
pub async fn self_authorization_transaction(config: Config) -> Result<()> {
    let builder = Eip7702Builder::new(config.clone());
    let sender = config.bob_signer.clone();

    // Print initial balances
    builder.print_balances("Initial Balances").await?;

    let authorization = builder
        .create_authorization(&config.bob_signer)
        .await?;
    let nonce = builder
        .get_wallet_nonce(config.bob_signer.address())
        .await?;

    let transfer_amount = U256::from(1e17); // 0.1 token
    let calls = vec![builder.create_token_transfer_call(
        &sender,
        config.receiver_address,
        transfer_amount,
    )];

    let validation_hash = builder
        .get_validation_hash(config.bob_signer.address(), nonce, &calls)
        .await?;
    let validation_data = builder.sign_validation_data(&config.bob_signer, &validation_hash);

    let tx = builder.build_execute_with_validator_transaction(
        &sender,
        config.bob_signer.address(),
        authorization,
        &calls,
        validation_data,
    );

    let receipt = builder.send_transaction(tx, &config.bob_signer).await?;
    println!("✅ Transaction successful! Gas used: {}", receipt.gas_used);

    // Print final balances
    builder.print_balances("Final Balances").await?;

    println!("\n🎉 Self-authorization transaction completed successfully!");
    Ok(())
}

/// Demonstrates relayer pattern where Alice signs off-chain and Bob submits the transaction
pub async fn relayer_transaction(config: Config) -> Result<()> {
    println!("🚀 Starting Relayer Transaction Pattern");
    println!("Alice will sign off-chain, Bob will submit the transaction and pay gas");

    let builder = Eip7702Builder::new(config.clone());
    let sender = config.bob_signer.clone();

    // Print initial balances
    builder.print_balances("Initial Balances").await?;

    let authorization = builder
        .create_authorization(&config.alice_signer)
        .await?;
    
    let nonce = builder
        .get_wallet_nonce(config.alice_signer.address())
        .await?;

    let transfer_amount = U256::from(500000); // 0.1 token
    let calls = vec![builder.create_token_transfer_call(
        &config.alice_signer,
        config.receiver_address,
        transfer_amount,
    )];

    let validation_hash = builder
        .get_validation_hash(config.alice_signer.address(), nonce, &calls)
        .await?;
    let validation_data = builder.sign_validation_data(&config.alice_signer, &validation_hash);

    let tx = builder.build_execute_with_validator_transaction(
        &sender,
        config.alice_signer.address(), // Transaction goes to Alice's address
        authorization,
        &calls,
        validation_data,
    );

    // Bob sends the transaction and pays the gas
    let receipt = builder.send_transaction(tx, &config.bob_signer).await?;
    println!("Transaction included in block: {}", receipt.block_number.expect("Failed to get block number"));

    // Print final balances
    builder.print_balances("Final Balances").await?;

    println!("\n🎉 Relayer transaction completed successfully!");
    println!("Alice's tokens were transferred while Bob paid the gas fees!");
    Ok(())
}

/// Demonstrates both transaction patterns sequentially
pub async fn demonstrate_patterns(config: Config) -> Result<()> {
    println!("🌟 EIP-7702 Transaction Patterns Demo");
    println!("This demo will show both self-authorization and relayer patterns\n");

    // Run self-authorization pattern
    self_authorization_transaction(config.clone()).await?;

    println!("\n{}\n", "=".repeat(80));

    // Run relayer pattern  
    relayer_transaction(config).await?;

    println!("\n🏁 All transaction patterns demonstrated successfully!");
    Ok(())
}

