use nalgebra::{ArrayStorage, Vector3};
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use super::{AzimuthElevation, Ellipsoid, LLHRadians, NED, WGS84};

/// WGS84 Earth Centered, Earth Fixed (ECEF) Cartesian coordinates (X, Y, Z).
///
/// Internally stored as an array of 3 [f64](std::f64) values: x, y, z all in meters
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct ECEF(Vector3<f64>);

impl ECEF {
    pub fn new(x: f64, y: f64, z: f64) -> ECEF {
        ECEF(Vector3::new(x, y, z))
    }

    pub const fn from_array(array: &[f64; 3]) -> ECEF {
        ECEF(Vector3::from_array_storage(ArrayStorage([*array; 1])))
    }

    pub(crate) fn from_vector3(vector: Vector3<f64>) -> ECEF {
        ECEF(vector)
    }

    pub fn as_ptr(&self) -> *const f64 {
        self.0.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut f64 {
        self.0.as_mut_ptr()
    }

    pub fn as_array_ref(&self) -> &[f64; 3] {
        &self.0.data.0[0]
    }

    pub fn as_mut_array_ref(&mut self) -> &mut [f64; 3] {
        &mut self.0.data.0[0]
    }

    pub(crate) fn as_vector_ref(&self) -> &Vector3<f64> {
        &self.0
    }

    pub fn x(&self) -> f64 {
        self.0[0]
    }

    pub fn y(&self) -> f64 {
        self.0[1]
    }

    pub fn z(&self) -> f64 {
        self.0[2]
    }

    /// Converts from WGS84 Earth Centered, Earth Fixed (ECEF) Cartesian
    /// coordinates (X, Y and Z) into WGS84 geodetic coordinates (latitude,
    /// longitude and height).
    pub fn to_llh(&self) -> LLHRadians {
        WGS84::ecef2llh(self)
    }

    /// Determine the azimuth and elevation of a point in WGS84 Earth Centered,
    /// Earth Fixed (ECEF) Cartesian coordinates from a reference point given in
    /// WGS84 ECEF coordinates.
    ///
    /// First the vector between the points is converted into the local North, East,
    /// Down frame of the reference point. Then we can directly calculate the
    /// azimuth and elevation.
    pub fn azel_of(&self, point: &ECEF) -> AzimuthElevation {
        WGS84::ecef2azel(point, self)
    }

    /// Rotate a vector from ECEF coordinates into NED coordinates, at a given
    /// reference point. This is approporiate for converting velocity vectors.
    ///
    /// This is the inverse of [NED::ecef_vector_at].
    pub fn ned_vector_at(&self, point: &ECEF) -> NED {
        WGS84::ecef2ned(self, point)
    }
}

impl Default for ECEF {
    fn default() -> Self {
        Self::new(0., 0., 0.)
    }
}

impl AsRef<[f64; 3]> for ECEF {
    fn as_ref(&self) -> &[f64; 3] {
        &self.0.data.0[0]
    }
}

impl AsMut<[f64; 3]> for ECEF {
    fn as_mut(&mut self) -> &mut [f64; 3] {
        &mut self.0.data.0[0]
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
