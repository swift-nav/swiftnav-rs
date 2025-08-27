use super::{Ellipsoid, ECEF, WGS84};
use nalgebra::{ArrayStorage, Vector3};

/// WGS84 geodetic coordinates (Latitude, Longitude, Height), with angles in degrees.
///
/// Internally stored as an array of 3 [f64](std::f64) values: latitude, longitude, and height above the ellipsoid in meters
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct LLHDegrees(Vector3<f64>);

impl LLHDegrees {
    /// Create an [`LLHDegrees`] object from the given latitude, longitude, height components
    #[must_use]
    pub fn new(lat: f64, lon: f64, height: f64) -> LLHDegrees {
        LLHDegrees(Vector3::new(lat, lon, height))
    }

    /// Create an [`LLHDegrees`] object from an array.
    ///
    /// Element 0 is latitude, element 1 is longitude, and element 2 is height
    #[must_use]
    pub const fn from_array(array: &[f64; 3]) -> LLHDegrees {
        LLHDegrees(Vector3::from_array_storage(ArrayStorage([*array; 1])))
    }

    /// Get the latitude component
    #[must_use]
    pub fn latitude(&self) -> f64 {
        self.0[0]
    }

    /// Get the longitude component
    #[must_use]
    pub fn longitude(&self) -> f64 {
        self.0[1]
    }

    /// Get the height component
    #[must_use]
    pub fn height(&self) -> f64 {
        self.0[2]
    }

    /// Converts a [`LLHDegrees`] position to [`LLHRadians`].
    ///
    /// The position doesn't change, just the representation of the angular values.
    #[must_use]
    pub fn to_radians(&self) -> LLHRadians {
        LLHRadians::new(
            self.0.x * std::f64::consts::PI / 180.0,
            self.0.y * std::f64::consts::PI / 180.0,
            self.0.z,
        )
    }

    /// Converts a [`LLHDegrees`] position to [`ECEF`]
    ///
    /// Uses the [`WGS84`] Ellipsoid
    #[must_use]
    pub fn to_ecef(&self) -> ECEF {
        self.to_radians().to_ecef()
    }
}

impl From<LLHRadians> for LLHDegrees {
    fn from(rad: LLHRadians) -> LLHDegrees {
        rad.to_degrees()
    }
}

impl From<ECEF> for LLHDegrees {
    fn from(ecef: ECEF) -> LLHDegrees {
        ecef.to_llh().to_degrees()
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
    pub fn new(lat: f64, lon: f64, height: f64) -> LLHRadians {
        LLHRadians(Vector3::new(lat, lon, height))
    }

    /// Create an [`LLHRadians`] object from an array.
    ///
    /// Element 0 is latitude, element 1 is longitude, and element 2 is height
    #[must_use]
    pub const fn from_array(array: &[f64; 3]) -> LLHRadians {
        LLHRadians(Vector3::from_array_storage(ArrayStorage([*array; 1])))
    }

    /// Get the latitude component
    #[must_use]
    pub fn latitude(&self) -> f64 {
        self.0[0]
    }

    /// Get the longitude component
    #[must_use]
    pub fn longitude(&self) -> f64 {
        self.0[1]
    }

    /// Get the height component
    #[must_use]
    pub fn height(&self) -> f64 {
        self.0[2]
    }

    /// Converts a [`LLHRadians`] to [`LLHDegrees`].
    ///
    /// The position doesn't change, just the representation of the angular values.
    #[must_use]
    pub fn to_degrees(&self) -> LLHDegrees {
        LLHDegrees::new(
            self.0.x * 180.0 / std::f64::consts::PI,
            self.0.y * 180.0 / std::f64::consts::PI,
            self.0.z,
        )
    }

    /// Converts a [`LLHRadians`] position to [`ECEF`]
    ///
    /// Uses the [`WGS84`] Ellipsoid
    #[must_use]
    pub fn to_ecef(&self) -> ECEF {
        WGS84::llh2ecef(self)
    }
}

impl From<LLHDegrees> for LLHRadians {
    fn from(deg: LLHDegrees) -> LLHRadians {
        deg.to_radians()
    }
}

impl From<ECEF> for LLHRadians {
    fn from(ecef: ECEF) -> LLHRadians {
        ecef.to_llh()
    }
}
