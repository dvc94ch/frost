//! Schnorr signature signing keys

use rand_core::{CryptoRng, RngCore};

use crate::{random_nonzero, Ciphersuite, Error, Field, Group, Scalar, Signature, VerifyingKey};

/// A signing key for a Schnorr signature on a FROST [`Ciphersuite::Group`].
#[derive(Copy, Clone)]
pub struct SigningKey<C>
where
    C: Ciphersuite,
{
    pub(crate) scalar: Scalar<C>,
}

impl<C> SigningKey<C>
where
    C: Ciphersuite,
{
    /// Generate a new signing key.
    pub fn new<R: RngCore + CryptoRng>(mut rng: R) -> SigningKey<C> {
        let scalar = random_nonzero::<C, R>(&mut rng);

        SigningKey { scalar }
    }

    /// Deserialize from bytes
    pub fn from_bytes(
        bytes: <<C::Group as Group>::Field as Field>::Serialization,
    ) -> Result<SigningKey<C>, Error<C>> {
        <<C::Group as Group>::Field as Field>::deserialize(&bytes)
            .map(|scalar| SigningKey { scalar })
            .map_err(|e| e.into())
    }

    /// Serialize `SigningKey` to bytes
    pub fn to_bytes(&self) -> <<C::Group as Group>::Field as Field>::Serialization {
        <<C::Group as Group>::Field as Field>::serialize(&self.scalar)
    }

    /// Create a signature `msg` using this `SigningKey`.
    pub fn sign<R: RngCore + CryptoRng>(&self, mut rng: R, msg: &[u8]) -> Signature<C> {
        let k = random_nonzero::<C, R>(&mut rng);

        let R = <C::Group>::generator() * k;

        // Generate Schnorr challenge
        let c = crate::challenge::<C>(&R, &VerifyingKey::<C>::from(*self).element, msg);

        let z = k + (c.0 * self.scalar);

        Signature { R, z }
    }
}

impl<C> From<&SigningKey<C>> for VerifyingKey<C>
where
    C: Ciphersuite,
{
    fn from(signing_key: &SigningKey<C>) -> Self {
        VerifyingKey {
            element: C::Group::generator() * signing_key.scalar,
        }
    }
}

impl<C> From<SigningKey<C>> for VerifyingKey<C>
where
    C: Ciphersuite,
{
    fn from(signing_key: SigningKey<C>) -> Self {
        VerifyingKey::<C>::from(&signing_key)
    }
}
