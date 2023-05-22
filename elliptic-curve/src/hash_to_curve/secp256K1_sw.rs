use ark_ff::MontFp;
use ark_secp256k1::Fq;

use super::sw_map::SWMap;

pub struct Secp256K1SWMap;

impl SWMap<ark_secp256k1::Config> for Secp256K1SWMap {
    const Z: Fq = MontFp!("1");

    const C1: Option<Fq> = Some(MontFp!("8"));

    const C2: Option<Fq> = Some(MontFp!(
        "57896044618658097711785492504343953926634992332820282019728792003954417335831"
    ));

    const C3: Option<Fq> = Some(MontFp!(
        "10388779673325959979325452626823788324994718367665745800388075445979975427086"
    ));

    const C4: Option<Fq> = Some(MontFp!(
        "77194726158210796949047323339125271902179989777093709359638389338605889781098"
    ));

    const DST: &'static [u8] = b"secp256k1_sw";
}
