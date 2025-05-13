mod simplest;
mod okx;

use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    
    match args.get(1).map(|s| s.as_str()) {
        Some("simplest") => simplest::make_transaction().await?,
        Some("okx") => okx::make_transaction().await?,
        _ => println!("Please specify which example to run: simplest or okx"),
    }
    
    Ok(())
}
