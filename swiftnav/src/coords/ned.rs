use nalgebra::{ArrayStorage, Vector3};

use super::{Ellipsoid, ECEF, WGS84};

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

    /// Create a [`NED`] object from an array.
    ///
    /// Element 0 is north, element 1 is east, and element 2 is down
    #[must_use]
    pub fn from_array(array: &[f64; 3]) -> NED {
        NED(Vector3::from_array_storage(ArrayStorage([*array; 1])))
    }

    /// Create a [`NED`] object from a [`Vector3<f64>`] object
    #[must_use]
    pub(crate) fn from_vector3(vector: Vector3<f64>) -> NED {
        NED(vector)
    }

    /// Get a reference to the inner [`Vector3<f64>`]
    #[must_use]
    pub(crate) fn as_vector_ref(&self) -> &Vector3<f64> {
        &self.0
    }

    /// Get the north component
    #[must_use]
    pub fn n(&self) -> f64 {
        self.0[0]
    }

    /// Get the east component
    #[must_use]
    pub fn e(&self) -> f64 {
        self.0[1]
    }

    /// Get the down component
    #[must_use]
    pub fn d(&self) -> f64 {
        self.0[2]
    }

    /// Rotate a local [`NED`] vector into a [`ECEF`] vector, at a given
    /// reference point. This is approporiate for converting velocity vectors.
    ///
    /// This is the inverse of [ECEF::ned_vector_at].
    #[must_use]
    pub fn ecef_vector_at(&self, ref_ecef: &ECEF) -> ECEF {
        WGS84::ned2ecef(self, ref_ecef)
    }
}
