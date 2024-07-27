use alloy_primitives::aliases::B256;

pub struct Signature {
    /// The R field of the signature; the point on the curve.
    pub r: B256,
    /// The S field of the signature; the point on the curve.
    pub s: B256,
    /// yParity: Signature Y parity; formally Ty
    /// also knows as `v`
    pub odd_y_parity: bool,
}
