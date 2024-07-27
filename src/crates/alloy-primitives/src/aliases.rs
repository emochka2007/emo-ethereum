/// A byte array of fixed length (`[u8; N]`).
///
/// This type allows us to more tightly control serialization, deserialization.
/// rlp encoding, decoding, and other type-level attributes for fixed-length
/// byte arrays.
///
pub type B256 = [u8; 32];
pub type B160 = [u8;20];
pub type B64 = [u8; 8];
pub type Address = B160;

pub type BlockNumber = u64;
