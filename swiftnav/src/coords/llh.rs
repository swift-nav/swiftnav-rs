use super::{Ellipsoid, ECEF, WGS84};
use nalgebra::{ArrayStorage, Vector3};

/// WGS84 geodetic coordinates (Latitude, Longitude, Height)
///
/// Internally stored as an array of 3 [f64](std::f64) values: latitude, longitude (both in the given angular units) and height above the geoid in meters
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct LLHDegrees(Vector3<f64>);

impl LLHDegrees {
    pub fn new(lat: f64, lon: f64, height: f64) -> LLHDegrees {
        LLHDegrees(Vector3::new(lat, lon, height))
    }

    pub const fn from_array(array: &[f64; 3]) -> LLHDegrees {
        LLHDegrees(Vector3::from_array_storage(ArrayStorage([*array; 1])))
    }

    // pub(crate) fn from_vector3(vector: Vector3<f64>) -> LLHDegrees {
    //     LLHDegrees(vector)
    // }

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

    // pub(crate) fn as_vector_ref(&self) -> &Vector3<f64> {
    //     &self.0
    // }

    pub fn latitude(&self) -> f64 {
        self.0[0]
    }

    pub fn longitude(&self) -> f64 {
        self.0[1]
    }

    pub fn height(&self) -> f64 {
        self.0[2]
    }

    /// Converts a LLH position from degrees to radians. The position doesn't change,
    /// just the representation of the angular values.
    pub fn to_radians(&self) -> LLHRadians {
        LLHRadians::new(
            self.0.x * std::f64::consts::PI / 180.0,
            self.0.y * std::f64::consts::PI / 180.0,
            self.0.z,
        )
    }

    /// Converts from WGS84 geodetic coordinates (latitude, longitude and height)
    /// into WGS84 Earth Centered, Earth Fixed Cartesian (ECEF) coordinates
    /// (X, Y and Z).
    pub fn to_ecef(&self) -> ECEF {
        self.to_radians().to_ecef()
    }
}

impl Default for LLHDegrees {
    fn default() -> LLHDegrees {
        LLHDegrees::new(0., 0., 0.)
    }
}

// impl AsRef<[f64; 3]> for LLHDegrees {
//     fn as_ref(&self) -> &[f64; 3] {
//         &self.0
//     }
// }

// impl AsMut<[f64; 3]> for LLHDegrees {
//     fn as_mut(&mut self) -> &mut [f64; 3] {
//         &mut self.0
//     }
// }

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

/// WGS84 geodetic coordinates (Latitude, Longitude, Height).
///
/// Internally stored as an array of 3 [f64](std::f64) values: latitude, longitude (both in the given angular units) and height above the geoid in meters
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct LLHRadians(Vector3<f64>);

impl LLHRadians {
    pub fn new(lat: f64, lon: f64, height: f64) -> LLHRadians {
        LLHRadians(Vector3::new(lat, lon, height))
    }

    pub const fn from_array(array: &[f64; 3]) -> LLHRadians {
        LLHRadians(Vector3::from_array_storage(ArrayStorage([*array; 1])))
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

    pub fn latitude(&self) -> f64 {
        self.0[0]
    }

    pub fn longitude(&self) -> f64 {
        self.0[1]
    }

    pub fn height(&self) -> f64 {
        self.0[2]
    }

    /// Converts a LLH position from radians to degrees. The position doesn't change,
    /// just the representation of the angular values.
    pub fn to_degrees(&self) -> LLHDegrees {
        LLHDegrees::new(
            self.0.x * 180.0 / std::f64::consts::PI,
            self.0.y * 180.0 / std::f64::consts::PI,
            self.0.z,
        )
    }

    /// Converts from WGS84 geodetic coordinates (latitude, longitude and height)
    /// into WGS84 Earth Centered, Earth Fixed Cartesian (ECEF) coordinates
    /// (X, Y and Z).
    pub fn to_ecef(&self) -> ECEF {
        WGS84::llh2ecef(self)
    }
}

impl Default for LLHRadians {
    fn default() -> LLHRadians {
        LLHRadians::new(0., 0., 0.)
    }
}

impl AsRef<[f64; 3]> for LLHRadians {
    fn as_ref(&self) -> &[f64; 3] {
        &self.0.data.0[0]
    }
}

impl AsMut<[f64; 3]> for LLHRadians {
    fn as_mut(&mut self) -> &mut [f64; 3] {
        &mut self.0.data.0[0]
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
