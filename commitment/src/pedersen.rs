use crate::Result;
use ark_ec::CurveGroup;
use ark_std::rand::Rng;

pub struct Pedersen<C: CurveGroup> {
    pub g: Vec<C::Affine>,
    pub h: C::Affine,
}

impl<C: CurveGroup> Pedersen<C> {
    pub fn setup<R: Rng>(rng: &mut R, len: usize) -> Pedersen<C> {
        let mut g = Vec::with_capacity(len);
        for _ in 0..len {
            g.push(C::rand(rng).into_affine())
        }

        let h = C::rand(rng).into_affine();

        Pedersen { g, h }
    }

    pub fn commit(&self, v: &[C::ScalarField], r: &C::ScalarField) -> Result<C::Affine> {
        if v.len() > self.g.len() {
            return Err(format!("maximum commitment is {}", self.g.len()));
        }

        Ok((C::msm_unchecked(&self.g, v) + self.h * r).into())
    }
}
