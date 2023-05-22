use ark_ec::hashing::curve_maps::swu::parity;
use ark_ec::short_weierstrass::Affine;
use ark_ec::short_weierstrass::Projective;
use ark_ec::short_weierstrass::SWCurveConfig;
use ark_ec::AffineRepr;
use ark_ff::field_hashers::{DefaultFieldHasher, HashToField};
use ark_ff::Field;
use ark_ff::One;
use sha2::digest::DynDigest;
use std::ops::*;

/// Module for the secp256K1 instance of the Optimized SWU map
#[allow(non_snake_case)]
pub mod secp256K1_oswu;

/// Module for jacobian
pub mod jacobian;

/// Trait for implementing a Optimized Shallue-van de Woestijne-Ulas method that is effective for
/// Weierstrass equation y^2 = x^3 + A * x +B where A != 0 and B != 0.
///
/// see <https://eprint.iacr.org/2019/403.pdf> [section 4.2]
pub trait OptimizedSWUMap<P: SWCurveConfig> {
    /// The constant z
    const Z: P::BaseField;

    /// The constant c1
    const C1: Option<P::BaseField>;

    /// The parameter A of isogeny curve
    const A: P::BaseField;

    /// The parameter B of isogeny curve
    const B: P::BaseField;

    /// Domain separation
    const DST: &'static [u8];

    /// Mapping an arbitrary field element to a point on the elliptic curve,
    /// This step matching step 2 and step 3
    fn map_to_curve(u: &P::BaseField) -> Result<Affine<P>, String> {
        Err("failed to hashing to the curve".to_string())
    }

    /// Mapping an arbitrary message to a field element,
    /// This step matching step 1
    fn hash_to_field<H: Default + DynDigest + Clone>(msg: &[u8], dst: &[u8]) -> Vec<P::BaseField> {
        let field_hasher = <DefaultFieldHasher<H> as HashToField<P::BaseField>>::new(dst);

        let field_elems: Vec<P::BaseField> = field_hasher.hash_to_field(msg, 2);

        field_elems
    }

    /// Perform hashing to curve
    ///
    /// To implement hashing to curve, the following steps are usually required:
    /// step 1 : u = hash_to_field(msg, 2)
    /// step 2 : Q0 = map_to_curve(u[0])
    /// step 3 : Q1 = map_to_curve(u[1])
    /// step 4 : R = Q0 + Q1
    /// step 5 : P = clear_cofactor(R)
    /// step 6 : return P
    fn hash<H: Default + DynDigest + Clone>(msg: &[u8]) -> Result<Affine<P>, String> {
        let rand_field_elems: Vec<P::BaseField> = Self::hash_to_field::<H>(msg, Self::DST);

        let rand_curve_elem_0 = Self::map_to_curve(&rand_field_elems[0])?;
        let rand_curve_elem_1 = Self::map_to_curve(&rand_field_elems[1])?;

        let rand_curve_elem: Affine<P> = rand_curve_elem_0.add(&rand_curve_elem_1).into();
        let rand_subgroup_elem = rand_curve_elem.clear_cofactor();

        Ok(rand_subgroup_elem)
    }

    /// The isogeny map from isogeny curve to origin curve
    fn isogeny_map(x: &P::BaseField) -> (P::BaseField, P::BaseField);

    /// The constant c1 equals ：
    /// c1 = - B/A
    fn c1() -> P::BaseField {
        if let Some(c1) = Self::C1 {
            c1
        } else {
            let b_neg = Self::B.neg();
            b_neg.div(&Self::A)
        }
    }
}
