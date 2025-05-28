use crate::core::{Config, Eip7702Builder};
use eyre::Result;

pub async fn initialize_wallet(config: Config, sender: alloy::signers::local::PrivateKeySigner) -> Result<()> {
    let builder = Eip7702Builder::new(config.clone());
   
    // help alice create a wallet_core and initialize it(create storage)
    let signed_authorization = builder.create_authorization(&config.alice_signer).await?;
    let tx = builder.build_initialize_transaction(&sender, config.alice_signer.address(), signed_authorization);

    let receipt = builder.send_transaction(tx, &sender).await?;
    println!("Transaction included in block: {}", receipt.block_number.expect("Failed to get block number"));

    let storage_address = builder.get_storage_address(config.alice_signer.address()).await?;
    println!("Storage address: {}", storage_address);

    Ok(())
}
