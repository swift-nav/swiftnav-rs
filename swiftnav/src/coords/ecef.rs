use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use nalgebra::Vector3;

use crate::{
    coords::{AzimuthElevation, Ellipsoid, LLHDegrees, LLHRadians, NED, WGS84},
    math,
};

/// WGS84 Earth Centered, Earth Fixed (ECEF) Cartesian coordinates (X, Y, Z).
///
/// Internally stored as an array of 3 [`f64`] values: x, y, z all in meters
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct ECEF(Vector3<f64>);

impl ECEF {
    /// Create an [`ECEF`] object from the given X, Y, Z components
    #[must_use]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(Vector3::new(x, y, z))
    }

    /// Get a reference to the inner array storing the data
    #[must_use]
    pub fn as_array(&self) -> &[f64; 3] {
        &self.0.data.0[0]
    }

    /// Get a mutable reference to the inner array storing the data
    #[must_use]
    pub fn as_array_mut(&mut self) -> &mut [f64; 3] {
        &mut self.0.data.0[0]
    }

    /// Get a reference to the inner [`Vector3`] storing the data
    #[must_use]
    pub fn as_vector(&self) -> &Vector3<f64> {
        &self.0
    }

    /// Get a mutable reference to the inner [`Vector3`] storing the data
    #[must_use]
    pub fn as_vector_mut(&mut self) -> &mut Vector3<f64> {
        &mut self.0
    }

    /// Get the X component
    #[must_use]
    pub fn x(&self) -> f64 {
        self.0.x
    }

    /// Get the Y component
    #[must_use]
    pub fn y(&self) -> f64 {
        self.0.y
    }

    /// Get the Z component
    #[must_use]
    pub fn z(&self) -> f64 {
        self.0.z
    }

    /// Converts a [`ECEF`] position into a [`LLHRadians`] position.
    #[must_use]
    pub fn to_llh(&self) -> LLHRadians {
        // Distance from polar axis.
        let p = (self.x() * self.x() + self.y() * self.y()).sqrt();

        // Compute longitude first, this can be done exactly.
        let longitude = if p == 0.0 {
            0.0
        } else {
            self.y().atan2(self.x())
        };

        // If we are close to the pole then convergence is very slow, treat this is a
        // special case.
        if p < WGS84::A * 1e-16 {
            let latitude = std::f64::consts::FRAC_PI_2.copysign(self.z());
            let height = self.z().abs() - WGS84::B;
            return LLHRadians::new(latitude, longitude, height);
        }

        // Calculate some other constants as defined in the Fukushima paper.
        let p_norm = p / WGS84::A;
        let e_c = (1. - WGS84::E * WGS84::E).sqrt();
        let z = self.z().abs() * e_c / WGS84::A;

        // Initial values for S and C correspond to a zero height solution.
        let mut s = z;
        let mut c = e_c * p_norm;

        // Neither S nor C can be negative on the first iteration so
        // starting prev = -1 will not cause and early exit.
        let mut prev_c = -1.0;
        let mut prev_s = -1.0;

        let mut a_n;
        let mut b_n;
        let mut d_n;
        let mut f_n;

        // Iterate a maximum of 10 times. This should be way more than enough for all
        // same inputs
        for _ in 0..10 {
            // Calculate some intermediate variables used in the update step based on
            // the current state.
            a_n = (s * s + c * c).sqrt();
            d_n = z * a_n * a_n * a_n + WGS84::E * WGS84::E * s * s * s;
            f_n = p_norm * a_n * a_n * a_n - WGS84::E * WGS84::E * c * c * c;
            b_n = 1.5 * WGS84::E * s * c * c * (a_n * (p_norm * s - z * c) - WGS84::E * s * c);

            // Update step.
            s = d_n * f_n - b_n * s;
            c = f_n * f_n - b_n * c;

            // The original algorithm as presented in the paper by Fukushima has a
            // problem with numerical stability. S and C can grow very large or small
            // and over or underflow a double. In the paper this is acknowledged and
            // the proposed resolution is to non-dimensionalise the equations for S and
            // C. However, this does not completely solve the problem. The author caps
            // the solution to only a couple of iterations and in this period over or
            // underflow is unlikely but as we require a bit more precision and hence
            // more iterations so this is still a concern for us.
            //
            // As the only thing that is important is the ratio T = S/C, my solution is
            // to divide both S and C by either S or C. The scaling is chosen such that
            // one of S or C is scaled to unity whilst the other is scaled to a value
            // less than one. By dividing by the larger of S or C we ensure that we do
            // not divide by zero as only one of S or C should ever be zero.
            //
            // This incurs an extra division each iteration which the author was
            // explicitly trying to avoid and it may be that this solution is just
            // reverting back to the method of iterating on T directly, perhaps this
            // bears more thought?

            if s > c {
                c /= s;
                s = 1.0;
            } else {
                s /= c;
                c = 1.0;
            }

            // Check for convergence and exit early if we have converged.
            if (s - prev_s).abs() < 1e-16 && (c - prev_c).abs() < 1e-16 {
                break;
            }
            prev_s = s;
            prev_c = c;
        }

        a_n = (s * s + c * c).sqrt();
        let latitude = 1.0_f64.copysign(self.z()) * (s / (e_c * c)).atan();
        let height = (p * e_c * c + self.z().abs() * s - WGS84::A * e_c * a_n)
            / (e_c * e_c * c * c + s * s).sqrt();
        LLHRadians::new(latitude, longitude, height)
    }

    /// Determine the [`AzimuthElevation`] of a [`ECEF`] point relative to a
    /// reference [`ECEF`] point.
    #[must_use]
    pub fn azel_of(&self, point: &ECEF) -> AzimuthElevation {
        /* Calculate the vector from the reference point in the local North, East,
         * Down frame of the reference point. */
        let ned = self.ned_to(point);

        let azimuth = ned.e().atan2(ned.n());
        /* atan2 returns angle in range [-pi, pi], usually azimuth is defined in the
         * range [0, 2pi]. */
        let azimuth = if azimuth < 0.0 {
            azimuth + 2.0 * std::f64::consts::PI
        } else {
            azimuth
        };

        let elevation = (-ned.d() / ned.as_vector().norm()).asin();
        AzimuthElevation::new(azimuth, elevation)
    }

    /// Calculate the local [`NED`] vector from this point to the other given point
    #[must_use]
    pub fn ned_to(&self, point: &ECEF) -> NED {
        let temp_vector = point - self;
        temp_vector.ned_vector_at(self)
    }

    /// Rotate this ECEF vector into NED coordinates, at a given
    /// reference point. This is approporiate for converting velocity vectors.
    ///
    /// This is the inverse of [`NED::ecef_vector_at`].
    #[must_use]
    pub fn ned_vector_at(&self, point: &ECEF) -> NED {
        let m = math::ecef2ned_matrix(point.to_llh());
        (m * self.as_vector()).into()
    }
}

impl From<[f64; 3]> for ECEF {
    fn from(array: [f64; 3]) -> Self {
        Self::new(array[0], array[1], array[2])
    }
}

impl From<&[f64; 3]> for ECEF {
    fn from(array: &[f64; 3]) -> Self {
        Self::new(array[0], array[1], array[2])
    }
}

impl From<Vector3<f64>> for ECEF {
    fn from(vector: Vector3<f64>) -> Self {
        Self(vector)
    }
}

impl From<(f64, f64, f64)> for ECEF {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Self::new(x, y, z)
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

impl AsRef<[f64; 3]> for ECEF {
    fn as_ref(&self) -> &[f64; 3] {
        self.as_array()
    }
}

impl AsRef<Vector3<f64>> for ECEF {
    fn as_ref(&self) -> &Vector3<f64> {
        self.as_vector()
    }
}

impl AsMut<[f64; 3]> for ECEF {
    fn as_mut(&mut self) -> &mut [f64; 3] {
        self.as_array_mut()
    }
}

impl AsMut<Vector3<f64>> for ECEF {
    fn as_mut(&mut self) -> &mut Vector3<f64> {
        self.as_vector_mut()
    }
}

impl Add for ECEF {
    type Output = Self;
    fn add(self, rhs: ECEF) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl Add<&ECEF> for ECEF {
    type Output = Self;
    fn add(self, rhs: &Self) -> Self {
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
    fn add_assign(&mut self, rhs: Self) {
        *self += &rhs;
    }
}

impl AddAssign<&ECEF> for ECEF {
    fn add_assign(&mut self, rhs: &Self) {
        self.0[0] += rhs.x();
        self.0[1] += rhs.y();
        self.0[2] += rhs.z();
    }
}

impl Sub for ECEF {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        ECEF(self.0 - rhs.0)
    }
}

impl Sub<&ECEF> for ECEF {
    type Output = Self;
    fn sub(self, rhs: &Self) -> Self {
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
    fn sub_assign(&mut self, rhs: Self) {
        *self -= &rhs;
    }
}

impl SubAssign<&ECEF> for ECEF {
    fn sub_assign(&mut self, rhs: &Self) {
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

    #[expect(clippy::float_cmp)]
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
