use std::ops::{Add, Mul};

use ark_ec::short_weierstrass::{Projective, SWCurveConfig};
use ark_serialize::CanonicalSerialize;
use elliptic_curve::hash_to_curve::sw_map::SWMap;
use sha2::Sha256;

use crate::KeyPair;

pub struct VRFProof<P: SWCurveConfig> {
    // The gamma can be output as a random number by converting it into bytes.
    pub gamma: Projective<P>,
    pub c: P::ScalarField,
    pub s: P::ScalarField,
    pub seed: Vec<u8>,
}

impl<P: SWCurveConfig> VRFProof<P> {
    pub fn verify<H: SWMap<P>>(&self, pk: &Projective<P>) -> bool {
        let mut buf = Vec::new();
        pk.serialize_uncompressed(&mut buf).unwrap();
        buf.extend_from_slice(&self.seed);

        let h = H::hash::<Sha256>(&buf);

        // u = c * PK + s * G
        let u: Projective<P> = pk.mul(&self.c).add(P::GENERATOR.mul(&self.s));

        // v=  c * gamma + s * h
        let v = self.gamma.mul(&self.c).add(h.mul(&self.s));

        // c = hash(PK || seed || h || gamma || u || v)
        let mut h_buf = Vec::new();
        h.serialize_uncompressed(&mut h_buf).unwrap();

        let mut gamma_buf: Vec<_> = Vec::new();
        self.gamma.serialize_uncompressed(&mut gamma_buf).unwrap();

        let mut u_buf: Vec<_> = Vec::new();
        u.serialize_uncompressed(&mut u_buf).unwrap();

        let mut v_buf: Vec<_> = Vec::new();
        v.serialize_uncompressed(&mut v_buf).unwrap();

        buf.extend_from_slice(&h_buf);
        buf.extend_from_slice(&gamma_buf);
        buf.extend_from_slice(&u_buf);
        buf.extend_from_slice(&v_buf);

        let c = KeyPair::<P>::hash_to_field(&buf);

        if c == self.c {
            true
        } else {
            false
        }
    }
}
