use nalgebra::Vector3;

use crate::{coords::ECEF, math};

/// Local North East Down reference frame coordinates
///
/// Internally stored as an array of 3 [f64](std::f64) values are all in meters
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct NED(Vector3<f64>);

impl NED {
    /// Create a [`NED`] object from the given north, east, down components
    #[must_use]
    pub fn new(n: f64, e: f64, d: f64) -> NED {
        NED(Vector3::new(n, e, d))
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

    /// Get the north component
    #[must_use]
    pub fn n(&self) -> f64 {
        self.0.x
    }

    /// Get the east component
    #[must_use]
    pub fn e(&self) -> f64 {
        self.0.y
    }

    /// Get the down component
    #[must_use]
    pub fn d(&self) -> f64 {
        self.0.z
    }

    /// Rotate a local [`NED`] vector into a [`ECEF`] vector, at a given
    /// reference point. This is approporiate for converting velocity vectors.
    ///
    /// This is the inverse of [`ECEF::ned_vector_at`].
    #[must_use]
    pub fn ecef_vector_at(&self, ref_ecef: &ECEF) -> ECEF {
        let m = math::ecef2ned_matrix(ref_ecef.to_llh());
        (m.transpose() * self.as_vector()).into()
    }
}

impl From<[f64; 3]> for NED {
    fn from(array: [f64; 3]) -> Self {
        Self::new(array[0], array[1], array[2])
    }
}

impl From<&[f64; 3]> for NED {
    fn from(array: &[f64; 3]) -> Self {
        Self::new(array[0], array[1], array[2])
    }
}

impl From<Vector3<f64>> for NED {
    fn from(vector: Vector3<f64>) -> Self {
        Self(vector)
    }
}

impl From<(f64, f64, f64)> for NED {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Self::new(x, y, z)
    }
}
