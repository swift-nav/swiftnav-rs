use nalgebra::{ArrayStorage, Vector3};
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use crate::coords::LLHDegrees;

use super::{AzimuthElevation, Ellipsoid, LLHRadians, NED, WGS84};

/// WGS84 Earth Centered, Earth Fixed (ECEF) Cartesian coordinates (X, Y, Z).
///
/// Internally stored as an array of 3 [`f64`] values: x, y, z all in meters
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct ECEF(Vector3<f64>);

impl ECEF {
    /// Create an [`ECEF`] object from the given X, Y, Z components
    #[must_use]
    pub fn new(x: f64, y: f64, z: f64) -> ECEF {
        ECEF(Vector3::new(x, y, z))
    }

    /// Create an [`ECEF`] object from an array
    ///
    /// Element 0 is X, element 1 is Y, and element 2 is Z
    #[must_use]
    pub const fn from_array(array: &[f64; 3]) -> ECEF {
        ECEF(Vector3::from_array_storage(ArrayStorage([*array; 1])))
    }

    /// Create an [`ECEF`] object from a [`nalgebra::Vector3<f64>`]
    #[must_use]
    pub(crate) fn from_vector3(vector: Vector3<f64>) -> ECEF {
        ECEF(vector)
    }

    /// Get a reference to the inner array storing the data
    #[must_use]
    pub(crate) fn as_array_ref(&self) -> &[f64; 3] {
        &self.0.data.0[0]
    }

    /// Get a mutable reference to the inner array storing the data
    #[must_use]
    pub(crate) fn as_mut_array_ref(&mut self) -> &mut [f64; 3] {
        &mut self.0.data.0[0]
    }

    /// Get a reference to the inner [`Vector3<f64>`]
    #[must_use]
    pub(crate) fn as_vector_ref(&self) -> &Vector3<f64> {
        &self.0
    }

    /// Get the X component
    #[must_use]
    pub fn x(&self) -> f64 {
        self.0[0]
    }

    /// Get the Y component
    #[must_use]
    pub fn y(&self) -> f64 {
        self.0[1]
    }

    /// Get the Z component
    #[must_use]
    pub fn z(&self) -> f64 {
        self.0[2]
    }

    /// Converts a [`ECEF`] position into a [`LLHRadians`] position.
    #[must_use]
    pub fn to_llh(&self) -> LLHRadians {
        WGS84::ecef2llh(self)
    }

    /// Determine the [`AzimuthElevation`] of a [`ECEF`] point relative to a
    /// reference [`ECEF`] point.
    #[must_use]
    pub fn azel_of(&self, point: &ECEF) -> AzimuthElevation {
        WGS84::ecef2azel(point, self)
    }

    /// Rotate a vector from ECEF coordinates into NED coordinates, at a given
    /// reference point. This is approporiate for converting velocity vectors.
    ///
    /// This is the inverse of [NED::ecef_vector_at].
    #[must_use]
    pub fn ned_vector_at(&self, point: &ECEF) -> NED {
        WGS84::ecef2ned(self, point)
    }
}

impl From<LLHRadians> for ECEF {
    fn from(value: LLHRadians) -> Self {
        value.to_ecef()
    }
}

impl From<LLHDegrees> for ECEF {
    fn from(value: LLHDegrees) -> Self {
        value.to_ecef()
    }
}

impl Add for ECEF {
    type Output = ECEF;
    fn add(self, rhs: ECEF) -> ECEF {
        ECEF(self.0 + rhs.0)
    }
}

impl Add<&ECEF> for ECEF {
    type Output = ECEF;
    fn add(self, rhs: &ECEF) -> ECEF {
        self + *rhs
    }
}

impl Add<&ECEF> for &ECEF {
    type Output = ECEF;
    fn add(self, rhs: &ECEF) -> ECEF {
        *self + *rhs
    }
}

impl AddAssign for ECEF {
    fn add_assign(&mut self, rhs: ECEF) {
        *self += &rhs;
    }
}

impl AddAssign<&ECEF> for ECEF {
    fn add_assign(&mut self, rhs: &ECEF) {
        self.0[0] += rhs.x();
        self.0[1] += rhs.y();
        self.0[2] += rhs.z();
    }
}

impl Sub for ECEF {
    type Output = ECEF;
    fn sub(self, rhs: ECEF) -> ECEF {
        ECEF(self.0 - rhs.0)
    }
}

impl Sub<&ECEF> for ECEF {
    type Output = ECEF;
    fn sub(self, rhs: &ECEF) -> ECEF {
        self - *rhs
    }
}

impl Sub<&ECEF> for &ECEF {
    type Output = ECEF;
    fn sub(self, rhs: &ECEF) -> ECEF {
        *self - *rhs
    }
}

impl SubAssign for ECEF {
    fn sub_assign(&mut self, rhs: ECEF) {
        *self -= &rhs;
    }
}

impl SubAssign<&ECEF> for ECEF {
    fn sub_assign(&mut self, rhs: &ECEF) {
        self.0[0] -= rhs.x();
        self.0[1] -= rhs.y();
        self.0[2] -= rhs.z();
    }
}

impl Mul<ECEF> for f64 {
    type Output = ECEF;
    fn mul(self, rhs: ECEF) -> ECEF {
        ECEF(self * rhs.0)
    }
}

impl Mul<&ECEF> for f64 {
    type Output = ECEF;
    fn mul(self, rhs: &ECEF) -> ECEF {
        self * *rhs
    }
}

impl MulAssign<f64> for ECEF {
    fn mul_assign(&mut self, rhs: f64) {
        *self *= &rhs;
    }
}

impl MulAssign<&f64> for ECEF {
    fn mul_assign(&mut self, rhs: &f64) {
        self.0[0] *= *rhs;
        self.0[1] *= *rhs;
        self.0[2] *= *rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ecef_ops() {
        let a = ECEF::new(1.0, 2.0, 3.0);
        let b = ECEF::new(4.0, 5.0, 6.0);

        let result = a + b;
        assert_eq!(5.0, result.x());
        assert_eq!(7.0, result.y());
        assert_eq!(9.0, result.z());

        let result = a + a + a;
        assert_eq!(3.0, result.x());
        assert_eq!(6.0, result.y());
        assert_eq!(9.0, result.z());

        let result = a - b;
        assert_eq!(-3.0, result.x());
        assert_eq!(-3.0, result.y());
        assert_eq!(-3.0, result.z());

        let result = 2.0 * a;
        assert_eq!(2.0, result.x());
        assert_eq!(4.0, result.y());
        assert_eq!(6.0, result.z());

        let mut result = a;
        result += b;
        assert_eq!(5.0, result.x());
        assert_eq!(7.0, result.y());
        assert_eq!(9.0, result.z());

        let mut result = a;
        result -= b;
        assert_eq!(-3.0, result.x());
        assert_eq!(-3.0, result.y());
        assert_eq!(-3.0, result.z());

        let mut result = a;
        result *= 2.0;
        assert_eq!(2.0, result.x());
        assert_eq!(4.0, result.y());
        assert_eq!(6.0, result.z());
    }
}
