#![allow(unused)]
use ark_ec::short_weierstrass::{Projective, SWCurveConfig};
use ark_ff::{
    field_hashers::{DefaultFieldHasher, HashToField},
    BigInteger,
};
use ark_serialize::CanonicalSerialize;
use ark_std::rand::Rng;
use ark_std::UniformRand;
use elliptic_curve::hash_to_curve::HashToCurve;
use proof::VRFProof;
use sha2::{digest::DynDigest, Sha256};
use std::{ops::*, vec};

#[cfg(test)]
mod tests;

mod proof;

#[derive(Clone, Default)]
pub struct KeyPair<P: SWCurveConfig> {
    pub private_key: P::ScalarField,
    pub public_key: Projective<P>,
}

impl<P: SWCurveConfig> KeyPair<P> {
    // Sample the key pair.
    fn new<R: Rng>(prng: &mut R) -> Self {
        let private_key = P::ScalarField::rand(prng);
        let public_key = P::GENERATOR.mul(&private_key);

        Self {
            private_key,
            public_key,
        }
    }

    // Verifiable Random Function
    fn vrf<H: HashToCurve<P>, R: Rng>(&self, seed: &[u8], prng: &mut R) -> VRFProof<P> {
        let mut buf = Vec::new();
        self.public_key.serialize_uncompressed(&mut buf).unwrap();
        buf.extend_from_slice(seed);

        // 1. h = HTC(PK || seed)
        let h = H::hash::<Sha256>(&buf).unwrap();

        // 2. gamma = h * SK
        let gamma = h.mul(&self.private_key);

        // 3. choise r
        let r = P::ScalarField::rand(prng);

        // 4. u = r * G
        let u = P::GENERATOR.mul(&r);

        // 5. v = r * h
        let v = h.mul(&r);

        // 6. c = hash(PK || seed || h || gamma || u || v)
        let mut h_buf = Vec::new();
        h.serialize_uncompressed(&mut h_buf).unwrap();

        let mut gamma_buf: Vec<_> = Vec::new();
        gamma.serialize_uncompressed(&mut gamma_buf).unwrap();

        let mut u_buf: Vec<_> = Vec::new();
        u.serialize_uncompressed(&mut u_buf).unwrap();

        let mut v_buf: Vec<_> = Vec::new();
        v.serialize_uncompressed(&mut v_buf).unwrap();

        buf.extend_from_slice(&h_buf);
        buf.extend_from_slice(&gamma_buf);
        buf.extend_from_slice(&u_buf);
        buf.extend_from_slice(&v_buf);

        let c = Self::hash_to_field(&buf);

        // 7. s = r - c * SK
        let s = r - c * self.private_key;

        VRFProof {
            gamma,
            c,
            s,
            seed: seed.to_vec(),
        }
    }

    /// Mapping an arbitrary message to a field element,
    fn hash_to_field(msg: &[u8]) -> P::ScalarField {
        let field_hasher =
            <DefaultFieldHasher<Sha256> as HashToField<P::ScalarField>>::new(b"sha256_dst");

        let field_elems: Vec<P::ScalarField> = field_hasher.hash_to_field(msg, 1);

        field_elems[0]
    }
}
