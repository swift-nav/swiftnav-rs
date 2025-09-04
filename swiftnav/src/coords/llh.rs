use super::{Ellipsoid, ECEF, WGS84};
use nalgebra::Vector3;

/// WGS84 geodetic coordinates (Latitude, Longitude, Height), with angles in degrees.
///
/// Internally stored as an array of 3 [f64](std::f64) values: latitude, longitude, and height above the ellipsoid in meters
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct LLHDegrees(Vector3<f64>);

impl LLHDegrees {
    /// Create an [`LLHDegrees`] object from the given latitude, longitude, height components
    #[must_use]
    pub fn new(lat: f64, lon: f64, height: f64) -> Self {
        Self(Vector3::new(lat, lon, height))
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

    /// Get the latitude component
    #[must_use]
    pub fn latitude(&self) -> f64 {
        self.0.x
    }

    /// Get the longitude component
    #[must_use]
    pub fn longitude(&self) -> f64 {
        self.0.y
    }

    /// Get the height component
    #[must_use]
    pub fn height(&self) -> f64 {
        self.0.z
    }

    /// Converts a [`LLHDegrees`] position to [`LLHRadians`].
    ///
    /// The position doesn't change, just the representation of the angular values.
    #[must_use]
    pub fn to_radians(&self) -> LLHRadians {
        LLHRadians::new(self.0.x.to_radians(), self.0.y.to_radians(), self.0.z)
    }

    /// Converts a [`LLHDegrees`] position to [`ECEF`]
    ///
    /// Uses the [`WGS84`] Ellipsoid
    #[must_use]
    pub fn to_ecef(&self) -> ECEF {
        self.to_radians().to_ecef()
    }
}

impl From<[f64; 3]> for LLHDegrees {
    fn from(array: [f64; 3]) -> Self {
        Self::new(array[0], array[1], array[2])
    }
}

impl From<&[f64; 3]> for LLHDegrees {
    fn from(array: &[f64; 3]) -> Self {
        Self::new(array[0], array[1], array[2])
    }
}

impl From<Vector3<f64>> for LLHDegrees {
    fn from(vector: Vector3<f64>) -> Self {
        Self(vector)
    }
}

impl From<(f64, f64, f64)> for LLHDegrees {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Self::new(x, y, z)
    }
}

impl From<LLHRadians> for LLHDegrees {
    fn from(rad: LLHRadians) -> Self {
        rad.to_degrees()
    }
}

impl From<ECEF> for LLHDegrees {
    fn from(ecef: ECEF) -> Self {
        ecef.to_llh().to_degrees()
    }
}

impl AsRef<[f64; 3]> for LLHDegrees {
    fn as_ref(&self) -> &[f64; 3] {
        self.as_array()
    }
}

impl AsRef<Vector3<f64>> for LLHDegrees {
    fn as_ref(&self) -> &Vector3<f64> {
        self.as_vector()
    }
}

impl AsMut<[f64; 3]> for LLHDegrees {
    fn as_mut(&mut self) -> &mut [f64; 3] {
        self.as_array_mut()
    }
}

impl AsMut<Vector3<f64>> for LLHDegrees {
    fn as_mut(&mut self) -> &mut Vector3<f64> {
        self.as_vector_mut()
    }
}

/// WGS84 geodetic coordinates (Latitude, Longitude, Height), with angles in radians.
///
/// Internally stored as an array of 3 [f64](std::f64) values: latitude, longitude, and height above the ellipsoid in meters
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct LLHRadians(Vector3<f64>);

impl LLHRadians {
    /// Create an [`LLHRadians`] object from the given latitude, longitude, height components
    #[must_use]
    pub fn new(lat: f64, lon: f64, height: f64) -> Self {
        Self(Vector3::new(lat, lon, height))
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

    /// Get the latitude component
    #[must_use]
    pub fn latitude(&self) -> f64 {
        self.0.x
    }

    /// Get the longitude component
    #[must_use]
    pub fn longitude(&self) -> f64 {
        self.0.y
    }

    /// Get the height component
    #[must_use]
    pub fn height(&self) -> f64 {
        self.0.z
    }

    /// Converts a [`LLHRadians`] to [`LLHDegrees`].
    ///
    /// The position doesn't change, just the representation of the angular values.
    #[must_use]
    pub fn to_degrees(&self) -> LLHDegrees {
        LLHDegrees::new(self.0.x.to_degrees(), self.0.y.to_degrees(), self.0.z)
    }

    /// Converts a [`LLHRadians`] position to [`ECEF`]
    ///
    /// Uses the [`WGS84`] Ellipsoid
    #[must_use]
    #[expect(clippy::many_single_char_names, reason = "It's math, whatyagonnado?")]
    pub fn to_ecef(&self) -> ECEF {
        let d = WGS84::E * (self.latitude()).sin();
        let n = WGS84::A / (1. - d * d).sqrt();

        let x = (n + self.height()) * self.latitude().cos() * self.longitude().cos();
        let y = (n + self.height()) * self.latitude().cos() * self.longitude().sin();
        let z = ((1.0 - WGS84::E * WGS84::E) * n + self.height()) * self.latitude().sin();

        ECEF::new(x, y, z)
    }
}

impl From<[f64; 3]> for LLHRadians {
    fn from(array: [f64; 3]) -> Self {
        Self::new(array[0], array[1], array[2])
    }
}

impl From<&[f64; 3]> for LLHRadians {
    fn from(array: &[f64; 3]) -> Self {
        Self::new(array[0], array[1], array[2])
    }
}

impl From<Vector3<f64>> for LLHRadians {
    fn from(vector: Vector3<f64>) -> Self {
        Self(vector)
    }
}

impl From<(f64, f64, f64)> for LLHRadians {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Self::new(x, y, z)
    }
}

impl From<LLHDegrees> for LLHRadians {
    fn from(deg: LLHDegrees) -> Self {
        deg.to_radians()
    }
}

impl From<ECEF> for LLHRadians {
    fn from(ecef: ECEF) -> Self {
        ecef.to_llh()
    }
}

impl AsRef<[f64; 3]> for LLHRadians {
    fn as_ref(&self) -> &[f64; 3] {
        self.as_array()
    }
}

impl AsRef<Vector3<f64>> for LLHRadians {
    fn as_ref(&self) -> &Vector3<f64> {
        self.as_vector()
    }
}

impl AsMut<[f64; 3]> for LLHRadians {
    fn as_mut(&mut self) -> &mut [f64; 3] {
        self.as_array_mut()
    }
}

impl AsMut<Vector3<f64>> for LLHRadians {
    fn as_mut(&mut self) -> &mut Vector3<f64> {
        self.as_vector_mut()
    }
}
