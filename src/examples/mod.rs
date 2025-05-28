pub mod patterns;
pub mod simplest;
pub mod normal;
pub mod initialize;

// Re-export main example functions for convenience
pub use patterns::{self_authorization_transaction, relayer_transaction, demonstrate_patterns};
pub use simplest::make_transaction as simple_transaction;
pub use normal::transfer_erc20;
pub use initialize::initialize_wallet; 