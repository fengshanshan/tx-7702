use alloy::primitives::{Address, address};

/// Common addresses used throughout the application
pub struct Addresses;

impl Addresses {
    /// ECDSA validator address (address(1) for built-in ECDSA validation)
    pub const ECDSA_VALIDATOR: Address = address!("0x0000000000000000000000000000000000000001");
}

/// Transaction pattern types
#[derive(Debug, Clone, Copy)]
pub enum TransactionPattern {
    /// Self-authorization: User authorizes and sends their own transaction
    SelfAuthorization,
    /// Relayer pattern: User signs off-chain, relayer submits transaction
    RelayerPattern,
} 