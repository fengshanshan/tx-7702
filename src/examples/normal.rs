use alloy::{
    network::TransactionBuilder,
    primitives::{U256, address},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
};
use eyre::Result;
use crate::core::{contracts::ERC20, Config};


// a demo for normal transfer erc20 case
pub async fn transfer_erc20(config: Config) -> Result<()> {
    let sender = config.bob_signer.clone();
    let provider = ProviderBuilder::new()
        .wallet(sender.clone())
        .on_http(config.rpc_url.clone());

    // TODO：change to the token address
    let token_address = address!("0x779877A7B0D9E8603169DdbD7836e478b4624789");
    let erc20 = ERC20::new(token_address, provider.clone());
    let amount = U256::from(99999999999000000u64);

    // Encode the calldata for the transfer function
    let call = erc20.transfer(config.receiver_address, amount);
    let calldata = call.calldata().to_owned();

    // Build the raw transaction
    let tx = TransactionRequest::default()
        .with_to(token_address)
        .with_input(calldata);

    println!("sender's token balance before transfer: {:?}", erc20.balanceOf(sender.address()).call().await?._0);
    println!("receiver's token balance before transfer: {:?}", erc20.balanceOf(config.receiver_address).call().await?._0);

    // Send the transaction
    let pending_tx = provider.send_transaction(tx).await?;
    println!("transfer transaction sent: {}", pending_tx.tx_hash());

    let receipt = pending_tx.get_receipt().await?;
    println!("Transaction included in block: {}", receipt.block_number.expect("Failed to get block number"));

    println!("sender's token balance after transfer: {:?}", erc20.balanceOf(sender.address()).call().await?._0);
    println!("receiver's token balance after transfer: {:?}", erc20.balanceOf(config.receiver_address).call().await?._0);

    Ok(())
}
