use std::marker::PhantomData;

use crate::Result;
use ark_ec::CurveGroup;
use ark_std::rand::Rng;

pub struct Parameters<C: CurveGroup> {
    pub g: Vec<C::Affine>,
    pub h: C::Affine,
}

pub struct Pedersen<C: CurveGroup> {
    _p: PhantomData<C>,
}

impl<C: CurveGroup> Pedersen<C> {
    pub fn setup<R: Rng>(rng: &mut R, len: usize) -> Parameters<C> {
        let mut g = Vec::with_capacity(len);
        for _ in 0..len {
            g.push(C::rand(rng).into_affine())
        }

        let h = C::rand(rng).into_affine();

        Parameters { g, h }
    }

    pub fn commit(
        params: &Parameters<C>,
        v: &[C::ScalarField],
        r: &C::ScalarField,
    ) -> Result<C::Affine> {
        if v.len() > params.g.len() {
            return Err(format!("maximum commitment is {}", params.g.len()));
        }

        Ok((C::msm_unchecked(&params.g, v) + params.h * r).into())
    }
}
