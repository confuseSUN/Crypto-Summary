use ark_ff::MontFp;
use ark_secp256k1::Affine;
use sha2::Sha256;

use crate::hash_to_curve::simplified_swu::secp256K1_sswu::Secp256K1SSWUMap;
use crate::hash_to_curve::simplified_swu::SimplifiedSWUMap;
use crate::hash_to_curve::sw_map::secp256K1_sw::Secp256K1SWMap;
use crate::hash_to_curve::sw_map::SWMap;

#[test]
fn test_sw_map_for_secp256k1() {
    let msg = b"hello, hash to secp256k1 ";
    let point = Secp256K1SWMap::hash::<Sha256>(msg).unwrap();
    assert!(point.is_on_curve());

    let expect_point = Affine::new_unchecked(
        MontFp!("30086692596842889525644937022152848460581736901329875745404997501727200602196"),
        MontFp!("27705106490549855196627610533769201525949575606514908196706981264818627387209"),
    );
    assert_eq!(point, expect_point);
}

#[test]
fn test_sswu_map_for_secp256k1() {
    let msg = b"hello, hash to secp256k1 ";
    let point = Secp256K1SSWUMap::hash::<Sha256>(msg).unwrap();
    assert!(point.is_on_curve());

    let expect_point = Affine::new_unchecked(
        MontFp!("10743741680020334228777834318532104455308224940808944015622063197025843808663"),
        MontFp!("38146701389086009568131611577699099700782252859687674831782271987177742184954"),
    );
    assert_eq!(point, expect_point);
}
