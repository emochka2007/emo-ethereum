use alloy_primitives::aliases::Address;

pub mod header;
pub mod withdrawal;
pub struct Withdrawal {
    /// Monotonically increasing identifier issued by consensus layer.
    #[cfg_attr(feature = "serde", serde(with = "alloy_serde::quantity"))]
    pub index: u64,
    /// Index of validator associated with withdrawal.
    #[cfg_attr(
        feature = "serde",
        serde(with = "alloy_serde::quantity", rename = "validatorIndex")
    )]
    pub validator_index: u64,
    /// Target address for withdrawn ether.
    pub address: Address,
    /// Value of the withdrawal in gwei.
    #[cfg_attr(feature = "serde", serde(with = "alloy_serde::quantity"))]
    pub amount: u64,
}
