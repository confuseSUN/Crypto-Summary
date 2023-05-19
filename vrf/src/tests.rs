use std::{assert_eq, ops::Mul};

use ark_ec::{AffineRepr, Group};
use ark_ff::MontFp;
use ark_secp256k1::Fq;
use ark_std::test_rng;
use elliptic_curve::hash_to_curve::sw_map::HashToCurve;

use crate::KeyPair;

pub struct Secp256K1SWMap;

impl HashToCurve<ark_secp256k1::Config> for Secp256K1SWMap {
    const Z: Fq = MontFp!("1");

    const C1 : Option<Fq> = None;

    const C2 : Option<Fq> = None;

    const C3 : Option<Fq> = None;
    
    const C4 : Option<Fq> = None;


    const DST: &'static [u8] = b"secp256k1";
}

#[test]
fn tset_generate_key_pair() {
    let mut prng = test_rng();
    let key_pair = KeyPair::<ark_secp256k1::Config>::new(&mut prng);

    assert_eq!(
        ark_secp256k1::Projective::generator().mul(&key_pair.private_key),
        key_pair.public_key
    )
}

#[test]
fn test_vrf() {
    let mut prng = test_rng();
    let key_pair = KeyPair::<ark_secp256k1::Config>::new(&mut prng);

    let seed = b"I am a seed";

    let proof = key_pair.vrf::<Secp256K1SWMap, _>(seed, &mut prng);

    assert!(proof.verify::<Secp256K1SWMap>(&key_pair.public_key))
}
