use ark_ff::Field;
use ark_ff::MontFp;
use ark_secp256k1::Fq;
use std::ops::*;

use super::OptimizedSWUMap;

pub struct Secp256K1OSWUMap;

const K10: Fq =
    MontFp!("64328938465175664124206102782604393251816658147578091133031991115504908150983");
const K11: Fq =
    MontFp!("3540463234204664767867377763959255381561641196938647754971861192896365225345");
const K12: Fq =
    MontFp!("37676595701789655284650173187508961899444205326770530105295841645151729341026");
const K13: Fq =
    MontFp!("64328938465175664124206102782604393251816658147578091133031991115504908150924");

const K20: Fq =
    MontFp!("95592507323525948732419199626899895302164312317343489384240252208201861084315");
const K21: Fq =
    MontFp!("107505182841474506714709588670204841388457878609653642868747406790547894725908");

const K30: Fq =
    MontFp!("34308767181427020866243254817389009734302217678708315270950395261602617680444");
const K31: Fq =
    MontFp!("90176424683627901097894375140309208301239340832535417794535213712559228940707");
const K32: Fq =
    MontFp!("18838297850894827642325086593754480949722102663385265052647920822575864670513");
const K33: Fq =
    MontFp!("21442979488391888041402034260868131083938886049192697044343997038501636050308");

const K40: Fq =
    MontFp!("115792089237316195423570985008687907853269984665640564039457584007908834670907");
const K41: Fq =
    MontFp!("55193343495945455350115628863323870199952967620749340073805588608787913909619");
const K42: Fq =
    MontFp!("45465685024895564648493397996619354229416833248839900263663526177913007417199");

impl OptimizedSWUMap<ark_secp256k1::Config> for Secp256K1OSWUMap {
    const Z: Fq = MontFp!("-1");

    const C1: Option<Fq> = Some(MontFp!(
        "5324262023205125242632636178842408935272934169651804884418803605709653231043"
    ));

    const A: Fq =
        MontFp!("28734576633528757162648956269730739219262246272443394170905244663053633733939");

    const B: Fq = MontFp!("1771");

    const DST: &'static [u8] = b"secp256k1_oswu";

    #[allow(unused_variables)]
    fn isogeny_map(isogeny_x: &Fq, isogeny_y: &Fq, isogeny_z: &Fq) -> (Fq, Fq, Fq) {
        let isogeny_z_2 = isogeny_z.square();
        let isogeny_z_3 = isogeny_z_2.mul(isogeny_z);
        let isogeny_z_4 = isogeny_z_2.square();
        let isogeny_z_6 = isogeny_z_3.square();

        let x_2 = isogeny_x.square();
        let x_3 = x_2.mul(isogeny_x);

        let x_num = x_3
            .mul(&K13)
            .add(x_2.mul(K12).mul(&isogeny_z_2))
            .add(isogeny_x.mul(&K11).mul(&isogeny_z_4))
            .add(K10.mul(&isogeny_z_6));
        let x_den = x_2
            .mul(&isogeny_z_2)
            .add(isogeny_x.mul(&K21).mul(&isogeny_z_4))
            .add(&K20.mul(&isogeny_z_6));

        let y_num = x_3
            .mul(&K33)
            .add(x_2.mul(&K32).mul(&isogeny_z_2))
            .add(isogeny_x.mul(&K31).mul(&isogeny_z_4))
            .add(&K30.mul(&isogeny_z_6))
            .mul(isogeny_y);
        let y_den = x_3
            .add(x_2.mul(&K42).mul(&isogeny_z_2))
            .add(isogeny_x.mul(&K41).mul(&isogeny_z_4))
            .add(&K40.mul(&isogeny_z_6))
            .mul(&isogeny_z_3);

        let z = x_den.mul(&y_den);
        let x = x_num.mul(&y_den).mul(&z);
        let y = y_num.mul(&x_den).mul(&z).mul(&z);
        (x, y, z)
    }
}
