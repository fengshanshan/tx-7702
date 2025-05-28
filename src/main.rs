use eyre::Result;
use tx_7702::{Config, examples};

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration from environment
    let config = Config::from_env()?;

    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    
    match args.get(1).map(|s| s.as_str()) {
        Some("init") => {
            println!("Initializing wallet...");
            examples::initialize_wallet(config.clone(), config.bob_signer).await?;
        }
        Some("self") => {
            println!("Running self-authorization pattern of 7702 erc20 transaction...");
            examples::self_authorization_transaction(config).await?;
        }
        Some("relayer") => {
            println!("Running relayer pattern of 7702 erc20 transaction...");
            examples::relayer_transaction(config).await?;
        }
        Some("simple") => {
            println!("Running simplest 7702 transaction");
            examples::simple_transaction().await?;
        }
        Some("normal") => {
            println!("Running normal ERC20 transfer without 7702");
            examples::transfer_erc20(config).await?;
        }
        _ => {
            println!("EIP-7702 Transaction Demo");
        }
    }

    Ok(())
}
