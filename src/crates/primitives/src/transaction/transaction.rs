use alloy_primitives::aliases::{Address, B256};
use crate::transaction::eip1559::TxEip1559;
use crate::transaction::eip2930::TxEip2930;
use crate::transaction::eip4844::TxEip4844;
use crate::transaction::signature::Signature;
use super::legacy::TxLegacy;
pub enum Transaction {
    /// Legacy Transaction (type `0x0`)
    /// Traditional Ethereum transactions containing parameteres
    /// `nonce`, `gasPrice`, `gasLimit`, `to`, `value`, `data`, `v`, `r`, and `s`
    Legacy(TxLegacy),
    /// Transaction with an [`AccessList`], type `0x1`
    /// accessList specifies an array of addresses and storage keys that the transaction
    /// plans to access, enabling gas savings on cross-contract calls by pre-declaring the
    /// accessed contract and storage slots
    Eip2930(TxEip2930),
    /// A tx with priority fee (EIP-1559), type `0x2`
    /// `maxPriorityFeePerGas` - specifying the maximum fee above the base fee the sender
    /// is willing to pay
    /// `maxFeePerGas` - setting the maximum total fee the sender is willing to pay
    Eip1559(TxEip1559),
    /// Shard block Transactions (EIP-4844), type `0x3`
    Eip4844(TxEip4844)
}
pub enum TxKind {
    /// A transaction that creates a contract.
    #[default]
    Create,
    /// A transaction that calls a contract or transfer.
    Call(Address),
}
pub struct TransactionSigned {
    /// Transaction hash
    pub hash: B256,
    /// The transaction signature values
    pub signature: Signature,
    /// Raw transaction info
    pub transaction: Transaction,
}

