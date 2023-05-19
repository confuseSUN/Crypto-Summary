use ark_ec::short_weierstrass::SWCurveConfig;
use ark_ff::Field;
use ark_ff::MontFp;
use ark_ff::Zero;
use ark_secp256k1::Affine;
use ark_secp256k1::{Config, Fq};
use sha2::Sha256;
use std::ops::Neg;
use std::ops::{Add, Div, Mul};

use super::sw_map::HashToCurve;

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
fn test_hash_to_curve_for_secp256k1() {
    let msg = b"hello, hash to secp256k1 ";
    let point = Secp256K1SWMap::hash::<Sha256>(msg).unwrap();

    let x =
        MontFp!("64043297498491436634439171231145164731426160760324877170764636047178421698471");
    let y =
        MontFp!("91198262659971141530092929731991797815001480798760841105513408186364380289421");
    let expect_point = Affine::new_unchecked(x, y);

    assert!(expect_point.is_on_curve());

    assert_eq!(point, expect_point);
}

#[test]
fn cal_z_for_secp256k1() {
    let candidate = [-5, -4, -3, -2, -1, 1, 2, 3, 4, 5];

    let a = Config::COEFF_A;
    let b = Config::COEFF_B;
    let two = Fq::from(2);
    let three = Fq::from(3);
    let four = Fq::from(4);

    let four_a = four.mul(&a);

    for x in candidate {
        let z = Fq::from(x);

        // require g(Z) != 0
        let y = z.mul(&z).add(&a);
        let y = y.mul(&z).add(&b);
        if y == Fq::zero() {
            continue;
        }

        // require -(3 * Z^2 + 4 * A) / (4 * g(Z)) != 0  and -(3 * Z^2 + 4 * A) / (4 * g(Z)) is square
        let four_y = four.mul(&y);
        let four_y_inv = four_y.inverse().unwrap();
        let tmp = z.mul(&z).mul(&three).add(&four_a);
        let tmp = tmp.neg();
        let tmp = tmp.mul(&four_y_inv);
        if tmp == Fq::zero() || !tmp.legendre().is_qr() {
            continue;
        }

        // require at least one of g(Z) and g(-Z / 2) is square
        let z_neg = z.neg();
        let z_neg_div_two = z_neg.div(&two);
        let y1 = z_neg_div_two.mul(&z_neg_div_two).add(&a);
        let y1 = y1.mul(&z_neg_div_two).add(&b);

        if !y.legendre().is_qr() && !y1.legendre().is_qr() {
            continue;
        }

        println!("z: {}", x);
    }
}
