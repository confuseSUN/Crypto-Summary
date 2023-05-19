use std::{assert_eq, ops::Mul};

use ark_ec::{AffineRepr, Group};
use ark_ff::MontFp;
use ark_secp256k1::Fq;
use ark_std::test_rng;
use elliptic_curve::hash_to_curve::{sw_map::HashToCurve, secp256K1_sw::Secp256K1SWMap};

use crate::KeyPair;

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
