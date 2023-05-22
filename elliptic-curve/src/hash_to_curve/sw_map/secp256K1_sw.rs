use ark_ff::MontFp;
use ark_secp256k1::Fq;

use super::SWMap;

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

#[cfg(test)]
mod tests {
    use ark_ec::short_weierstrass::SWCurveConfig;
    use ark_ff::Field;
    use ark_ff::Zero;
    use ark_secp256k1::{Config, Fq};
    use std::ops::*;

    #[test]
    fn cal_z_() {
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
}
