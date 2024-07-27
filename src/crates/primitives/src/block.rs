use primitives_traits::header::Header;
use crate::transaction::transaction::TransactionSigned;

// Ethereum Full Block
pub struct Block {
    //Block header
    pub header: Header,
    // Transactions in this block
    pub body: Vec<TransactionSigned>,
    // Ommers/uncles header;
    //obsidian://open?vault=Obsidian%20Vault&file=Ommers%20%26%20uncles%20header
    pub ommers: Vec<Header>,
    // pub withdrawals:
    // pub requests:
}

