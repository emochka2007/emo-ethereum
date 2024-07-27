use alloy_primitives::aliases::Address;

pub struct Withdrawal {
    pub index: u64,
    pub validator_index: u64,
    /// Target address for withdrawn ether
    pub address: Address,
    /// Value of the withdrawal in gwei
    pub amount: u64
}
pub struct Withdrawals(Vec<Withdrawal>);