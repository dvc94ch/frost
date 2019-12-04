use blake2b_simd::{Params, State};

use crate::Scalar;

/// Provides H^star, the hash-to-scalar function used by RedJubjub.
pub struct HStar {
    state: State,
}

impl Default for HStar {
    fn default() -> Self {
        let state = Params::new()
            .hash_length(64)
            .personal(b"Zcash_RedJubjubH")
            .to_state();
        Self { state }
    }
}

impl HStar {
    /// Add `data` to the hash, and return `Self` for chaining.
    pub fn update(mut self, data: &[u8]) -> Self {
        self.state.update(data);
        self
    }

    /// Consume `self` to compute the hash output.
    pub fn finalize(self) -> Scalar {
        Scalar::from_bytes_wide(self.state.finalize().as_array())
    }
}
