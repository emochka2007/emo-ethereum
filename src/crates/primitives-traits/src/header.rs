use std::io::Bytes;
use alloy_primitives::aliases::{B256, Address, BlockNumber, B64};
pub struct Header {
    /// The Keccak-256-bit hash of the parent
    /// block's header, in its entirety: formally HP
    pub parent_hash: B256,
    pub ommers_hash: B256,
    /// The 160-bit address to which all fees collected from the successful mining of this block
    pub beneficiary: Address,
    /// The Keccak-256-bit hash of the root node of the state trie, after all transactions
    /// are executed and finalisations applied
    pub state_root: B256,
    /// The keccak-256-bit hash of the root node of the trie structure populated with each
    /// transaction in the transactions list portion of the block:
    pub transactions_root: B256,
    /// The keccak 256-bit hash of the root node of the trie structure populated with the receipts
    /// of each tx in the tx list portion of the block
    pub receipts_root: B256,
    /// The Keccak 256-bit hash of the withdrawals list portion of the block
    /// https://eips.ethereum.org/EIPS/eip-4895>
    pub withdrawals_root: Option<B256>,
    /// Logs of the events of the block
    /// obsidian://open?vault=Obsidian%20Vault&file=LogsBloom
    pub logs_bloom: B256,
    /// A scalar value of the difficulty level
    /// this can be calculated from the prev block difficulty
    pub difficulty: u64,
    /// A scalar value equal to the number of ancestor blocks.
    /// Genesis block has a number of zero
    pub number: BlockNumber, //u64
    /// A scalar value equal to the current limit of gas per block
    pub gas_limit: u128,
    /// A scalar value equal to the total gas used in transactions
    pub gas_used: u128,
    /// A scalar value equal to the Unix time
    pub timestamp: u64,
    /// A 256-bit hash which, combined with
    /// the nonce, proves that a sufficient amount of computation has been carried out on this block
    pub mix_hash: B256,
    /// A 64-bit value which, proves that a sufficient amount of computation has been carried
    pub nonce: B64,
    /// A scalar representing EIP1559 base fee
    /// The algorithm results in the base fee per gas increasing when blocks are
    /// above the gas target and decreasing when blocks are below the gas target
    /// The base fee per gas os burned
    pub base_fee_per_gas: Option<u128>,
    /// The total amount of blob gas consumed by the transactions within the block
    /// added in EIP-4844
    /// obsidian://open?vault=Obsidian%20Vault&file=--PROGRAMMING--%2F--BLOCKCHAIN--%2F--ETHEREUM--%2F--EIPs--%2FEIP-4844
    pub blob_gas_used: Option<u128>,
    /// A running total of blob gas consumed in excess of the target
    pub excess_blob_gas: Option<u128>,
    /// The hash of the parent beacon block's root is included in execution blocks, as proposed by EIP-4788
    pub parent_beacon_block_root: Option<B256>,
    /// The Keccak 256-bit hash of the root node of the trie structure
    pub requests_root: Option<B256>,
    /// An arbitrary byte array containing data relevant to this block
    /// This must be 32 bytes or fewer
    pub extra_data: Bytes<u32>
}