use alloy::sol;

// Contract interfaces generated from ABI
sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    struct Call {
        address target;
        uint256 value;
        bytes data;
    }

    #[allow(missing_docs)]
    #[sol(rpc)]
    contract IWalletCore {
        function initialize() external; 
        function getMainStorage() external view returns (address);
        function executeWithValidator(Call[] calldata calls, address validator, bytes calldata validateData) external;
        function getNonce(address sender) external view returns (uint256);
        function getValidationTypedHash(uint256 nonce, Call[] calldata calls) external view returns (bytes32);
        function addValidator(
            address validatorImpl,
            bytes calldata immutableArgs
        ) external;
    }

    #[allow(missing_docs)]
    #[sol(rpc)]
    contract IStorage {
        function getNonce() external view returns (uint256);
    }

    #[allow(missing_docs)]
    #[sol(rpc)]
    contract ERC20 {
        function approve(address spender, uint256 amount) public virtual override returns (bool);
        function transfer(address recipient, uint256 amount) public virtual override returns (bool);
        function balanceOf(address account) public view virtual override returns (uint256);
        function transferFrom(address sender, address recipient, uint256 amount) public virtual override returns (bool);
    }
} 