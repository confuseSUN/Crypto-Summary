use ark_ff::MontFp;
use ark_secp256k1::Fq;

use super::sw_map::SWMap;

pub struct Secp256K1SWMap;

impl SWMap<ark_secp256k1::Config> for Secp256K1SWMap {
    const Z: Fq = MontFp!("1");

    const C1: Option<Fq> = None;

    const C2: Option<Fq> = None;

    const C3: Option<Fq> = None;

    const C4: Option<Fq> = None;

    const DST: &'static [u8] = b"secp256k1";
}
