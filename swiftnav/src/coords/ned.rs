use nalgebra::{ArrayStorage, Vector3};

use super::{Ellipsoid, ECEF, WGS84};

/// Local North East Down reference frame coordinates
///
/// Internally stored as an array of 3 [f64](std::f64) values: N, E, D all in meters
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct NED(Vector3<f64>);

impl NED {
    pub fn new(n: f64, e: f64, d: f64) -> NED {
        NED(Vector3::new(n, e, d))
    }

    pub fn from_array(array: &[f64; 3]) -> NED {
        NED(Vector3::from_array_storage(ArrayStorage([*array; 1])))
    }

    pub(crate) fn from_vector3(vector: Vector3<f64>) -> NED {
        NED(vector)
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

    pub fn n(&self) -> f64 {
        self.0[0]
    }

    pub fn e(&self) -> f64 {
        self.0[1]
    }

    pub fn d(&self) -> f64 {
        self.0[2]
    }

    /// Rotate a vector from NED coordinates into ECEF coordinates, at a given
    /// reference point. This is approporiate for converting velocity vectors.
    ///
    /// This is the inverse of [ECEF::ned_vector_at].
    pub fn ecef_vector_at(&self, ref_ecef: &ECEF) -> ECEF {
        WGS84::ned2ecef(self, ref_ecef)
    }
}

impl Default for NED {
    fn default() -> Self {
        Self::new(0., 0., 0.)
    }
}

impl AsRef<[f64; 3]> for NED {
    fn as_ref(&self) -> &[f64; 3] {
        &self.0.data.0[0]
    }
}

impl AsMut<[f64; 3]> for NED {
    fn as_mut(&mut self) -> &mut [f64; 3] {
        &mut self.0.data.0[0]
    }
}
