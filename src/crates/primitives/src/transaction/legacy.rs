use std::io::Bytes;
use super::transaction::TxKind;

pub struct TxLegacy {
    /// Added as EIP-155: Simple replay attack protection
    pub chain_id: Option<u64>,
    /// A scalar value equal to the number of transactions sent by the sender
    pub nonce: u64,
    /// A scalar value equal to the number of wei paid per unit of gas
    pub gas_price: u128,
    /// A scalar value equal to the amx
    /// amount of gas that should be used in executing
    /// this transaction. This is paid up-front, before any
    /// computation is done and may not be increased later
    pub gas_limit: u64,
    /// The 160-bit address of the message
    pub to: TxKind,
    /// A scalar value equal to the number of wei to
    /// be transferred to the message call's recipient or
    /// in the case of contract creation, as an endowment
    /// to the newly create account
    pub value: u64,
    /// Input has two uses depending on TxKind
    /// EVM-code for the account init `CREATE`
    /// data: an unlimited size byte array specifying the input data
    /// of the message call
    pub input: Bytes<u32>
}