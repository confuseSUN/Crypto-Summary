use ark_ec::short_weierstrass::{Affine, SWCurveConfig};
use ark_ff::Field;
use ark_std::One;

pub struct Jacobian<P: SWCurveConfig> {
    /// `X / Z^2` jacobian of the affine `X`
    pub x: P::BaseField,
    /// `Y / Z^3` jacobian of the affine `Y`
    pub y: P::BaseField,
    /// Z
    pub z: P::BaseField,
}

impl<P: SWCurveConfig> Jacobian<P> {
    pub fn new_unchecked(x: P::BaseField, y: P::BaseField, z: P::BaseField) -> Self {
        Self { x, y, z }
    }

    /// Checks whether `self.z.is_zero()`.
    fn is_zero(&self) -> bool {
        self.z == P::BaseField::ZERO
    }
}

// The jacobian point X, Y, Z is represented in the affine
// coordinates as X/Z^2, Y/Z^3.
impl<P: SWCurveConfig> From<Jacobian<P>> for Affine<P> {
    #[inline]
    fn from(p: Jacobian<P>) -> Affine<P> {
        if p.is_zero() {
            Affine::identity()
        } else if p.z.is_one() {
            Affine::new_unchecked(p.x, p.y)
        } else {
            let zinv = p.z.inverse().unwrap();
            let zinv_squared = zinv.square();

            // X/Z^2
            let x = p.x * &zinv_squared;

            // Y/Z^3
            let y = p.y * &(zinv_squared * &zinv);

            Affine::new_unchecked(x, y)
        }
    }
}
