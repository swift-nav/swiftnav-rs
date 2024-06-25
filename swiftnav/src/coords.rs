// Copyright (c) 2020-2021 Swift Navigation Inc.
// Contact: Swift Navigation <dev@swiftnav.com>
//
// This source is subject to the license found in the file 'LICENSE' which must
// be be distributed together with this source. All other rights reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND,
// EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A PARTICULAR PURPOSE.
//! Coordinates and conversions
//!
//! These four primary coordinates types are defined:
//!  * [LLHDegrees]/[LLHRadians] - Geodetic coordinates, Latitude Lontitude Height
//!  * [ECEF] - Cartesian coordinates, Earth Centered, Earth Fixed
//!  * [NED] - Relative direction coordinates, North East Down
//!  * [AzimuthElevation] - Relative direction coordinates, Azimith Elevation
//!
//! --------
//! Conversion from geodetic coordinates latitude, longitude and height
//! (ϕ, λ, h) into Cartesian coordinates (X, Y, Z) can be
//! achieved with the following formulae:
//!  * X = (N(ϕ) + h) * cos(ϕ) * cos(λ)
//!  * Y = (N(ϕ) + h) * cos(ϕ) * sin(λ)
//!  * Z = [(1-e^2) * N(ϕ) + h] * sin(ϕ)
//!
//! Where the 'radius of curvature', N(ϕ), is defined as:
//!  * N(ϕ) = a / sqrt(1-e^2 / sin^2(ϕ))
//!
//! and `a` is the WGS84 semi-major axis and `e` is the WGS84
//! eccentricity.
//!
//! --------
//! Conversion from Cartesian to geodetic coordinates is a much harder problem
//! than conversion from geodetic to Cartesian. There is no satisfactory closed
//! form solution but many different iterative approaches exist.
//!
//! Here we implement a relatively new algorithm due to Fukushima (2006) that is
//! very computationally efficient, not requiring any transcendental function
//! calls during iteration and very few divisions. It also exhibits cubic
//! convergence rates compared to the quadratic rate of convergence seen with
//! the more common algortihms based on the Newton-Raphson method.
//!
//! # References
//!   * "A comparison of methods used in rectangular to Geodetic Coordinates
//!      Transformations", Burtch R. R. (2006), American Congress for Surveying
//!      and Mapping Annual Conference. Orlando, Florida.
//!   * "Transformation from Cartesian to Geodetic Coordinates Accelerated by
//!      Halley’s Method", T. Fukushima (2006), Journal of Geodesy.

use std::fmt;

/// WGS84 geodetic coordinates (Latitude, Longitude, Height)
///
/// Internally stored as an array of 3 [f64](std::f64) values: latitude, longitude (both in the given angular units) and height above the geoid in meters
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct LLHDegrees([f64; 3]);

impl LLHDegrees {
    pub fn new(lat: f64, lon: f64, height: f64) -> LLHDegrees {
        LLHDegrees([lat, lon, height])
    }

    pub fn from_array(array: &[f64; 3]) -> LLHDegrees {
        LLHDegrees(*array)
    }

    pub fn as_ptr(&self) -> *const [f64; 3] {
        &self.0
    }

    pub fn as_mut_ptr(&mut self) -> *mut [f64; 3] {
        &mut self.0
    }

    pub fn as_array_ref(&self) -> &[f64; 3] {
        &self.0
    }

    pub fn as_mut_array_ref(&mut self) -> &mut [f64; 3] {
        &mut self.0
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

    /// Converts a LLH position from degrees to radians. The position doesn't change,
    /// just the representation of the angular values.
    pub fn to_radians(&self) -> LLHRadians {
        let mut rad = LLHRadians::default();
        unsafe { swiftnav_sys::llhdeg2rad(self.as_ptr(), rad.as_mut_ptr()) };
        rad
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

impl AsRef<[f64; 3]> for LLHDegrees {
    fn as_ref(&self) -> &[f64; 3] {
        &self.0
    }
}

impl AsMut<[f64; 3]> for LLHDegrees {
    fn as_mut(&mut self) -> &mut [f64; 3] {
        &mut self.0
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

/// WGS84 geodetic coordinates (Latitude, Longitude, Height).
///
/// Internally stored as an array of 3 [f64](std::f64) values: latitude, longitude (both in the given angular units) and height above the geoid in meters
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct LLHRadians([f64; 3]);

impl LLHRadians {
    pub fn new(lat: f64, lon: f64, height: f64) -> LLHRadians {
        LLHRadians([lat, lon, height])
    }

    pub fn from_array(array: &[f64; 3]) -> LLHRadians {
        LLHRadians(*array)
    }

    pub fn as_ptr(&self) -> *const [f64; 3] {
        &self.0
    }

    pub fn as_mut_ptr(&mut self) -> *mut [f64; 3] {
        &mut self.0
    }

    pub fn as_array_ref(&self) -> &[f64; 3] {
        &self.0
    }

    pub fn as_mut_array_ref(&mut self) -> &mut [f64; 3] {
        &mut self.0
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
        let mut deg = LLHDegrees::default();
        unsafe { swiftnav_sys::llhrad2deg(self.as_ptr(), deg.as_mut_ptr()) };
        deg
    }

    /// Converts from WGS84 geodetic coordinates (latitude, longitude and height)
    /// into WGS84 Earth Centered, Earth Fixed Cartesian (ECEF) coordinates
    /// (X, Y and Z).
    pub fn to_ecef(&self) -> ECEF {
        let mut ecef = ECEF::default();
        unsafe { swiftnav_sys::wgsllh2ecef(self.as_ptr(), ecef.as_mut_ptr()) };
        ecef
    }
}

impl Default for LLHRadians {
    fn default() -> LLHRadians {
        LLHRadians::new(0., 0., 0.)
    }
}

impl AsRef<[f64; 3]> for LLHRadians {
    fn as_ref(&self) -> &[f64; 3] {
        &self.0
    }
}

impl AsMut<[f64; 3]> for LLHRadians {
    fn as_mut(&mut self) -> &mut [f64; 3] {
        &mut self.0
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

/// WGS84 Earth Centered, Earth Fixed (ECEF) Cartesian coordinates (X, Y, Z).
///
/// Internally stored as an array of 3 [f64](std::f64) values: x, y, z all in meters
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct ECEF([f64; 3]);

impl ECEF {
    pub fn new(x: f64, y: f64, z: f64) -> ECEF {
        ECEF([x, y, z])
    }

    pub fn from_array(array: &[f64; 3]) -> ECEF {
        ECEF(*array)
    }

    pub fn as_ptr(&self) -> *const [f64; 3] {
        &self.0
    }

    pub fn as_mut_ptr(&mut self) -> *mut [f64; 3] {
        &mut self.0
    }

    pub fn as_single_ptr(&self) -> *const f64 {
        &self.0[0]
    }

    pub fn as_array_ref(&self) -> &[f64; 3] {
        &self.0
    }

    pub fn as_mut_array_ref(&mut self) -> &mut [f64; 3] {
        &mut self.0
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
        let mut llh = LLHRadians::from_array(&[0.0; 3]);
        unsafe { swiftnav_sys::wgsecef2llh(self.as_ptr(), llh.as_mut_ptr()) };
        llh
    }

    /// Determine the azimuth and elevation of a point in WGS84 Earth Centered,
    /// Earth Fixed (ECEF) Cartesian coordinates from a reference point given in
    /// WGS84 ECEF coordinates.
    ///
    /// First the vector between the points is converted into the local North, East,
    /// Down frame of the reference point. Then we can directly calculate the
    /// azimuth and elevation.
    pub fn azel_of(&self, point: &ECEF) -> AzimuthElevation {
        let mut azel = AzimuthElevation::new(0.0, 0.0);
        unsafe {
            swiftnav_sys::wgsecef2azel(point.as_ptr(), self.as_ptr(), &mut azel.az, &mut azel.el)
        };
        azel
    }

    /// Rotate a vector from ECEF coordinates into NED coordinates, at a given
    /// reference point. This is approporiate for converting velocity vectors.
    ///
    /// This is the inverse of [NED::ecef_vector_at].
    pub fn ned_vector_at(&self, point: &ECEF) -> NED {
        let mut ned = NED::default();
        unsafe { swiftnav_sys::wgsecef2ned(self.as_ptr(), point.as_ptr(), ned.as_mut_ptr()) };
        ned
    }
}

impl Default for ECEF {
    fn default() -> Self {
        Self::new(0., 0., 0.)
    }
}

impl AsRef<[f64; 3]> for ECEF {
    fn as_ref(&self) -> &[f64; 3] {
        &self.0
    }
}

impl AsMut<[f64; 3]> for ECEF {
    fn as_mut(&mut self) -> &mut [f64; 3] {
        &mut self.0
    }
}

/// Local North East Down reference frame coordinates
///
/// Internally stored as an array of 3 [f64](std::f64) values: N, E, D all in meters
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct NED([f64; 3]);

impl NED {
    pub fn new(n: f64, e: f64, d: f64) -> NED {
        NED([n, e, d])
    }

    pub fn from_array(array: &[f64; 3]) -> NED {
        NED(*array)
    }

    pub fn as_ptr(&self) -> *const [f64; 3] {
        &self.0
    }

    pub fn as_mut_ptr(&mut self) -> *mut [f64; 3] {
        &mut self.0
    }

    pub fn as_array_ref(&self) -> &[f64; 3] {
        &self.0
    }

    pub fn as_mut_array_ref(&mut self) -> &mut [f64; 3] {
        &mut self.0
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
        let mut ecef = ECEF::default();
        unsafe { swiftnav_sys::wgsned2ecef(self.as_ptr(), ref_ecef.as_ptr(), ecef.as_mut_ptr()) };
        ecef
    }
}

impl Default for NED {
    fn default() -> Self {
        Self::new(0., 0., 0.)
    }
}

impl AsRef<[f64; 3]> for NED {
    fn as_ref(&self) -> &[f64; 3] {
        &self.0
    }
}

impl AsMut<[f64; 3]> for NED {
    fn as_mut(&mut self) -> &mut [f64; 3] {
        &mut self.0
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct AzimuthElevation {
    pub az: f64,
    pub el: f64,
}

impl AzimuthElevation {
    pub fn new(az: f64, el: f64) -> AzimuthElevation {
        AzimuthElevation { az, el }
    }
}

impl Default for AzimuthElevation {
    fn default() -> Self {
        Self::new(0., 0.)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct SexagesimalAngle {
    pub degrees: i32,
    pub minutes: i32,
    pub seconds: f64,
}

impl SexagesimalAngle {
    pub fn new(degrees: i32, minutes: i32, seconds: f64) -> SexagesimalAngle {
        SexagesimalAngle {
            degrees,
            minutes,
            seconds,
        }
    }

    pub fn from_degree_f64(degrees: f64) -> SexagesimalAngle {
        let degrees_int = degrees.trunc();
        let minutes = (degrees - degrees_int) * 60.0;
        let minutes_int = minutes.trunc();
        let seconds = (minutes - minutes_int) * 60.0;
        SexagesimalAngle {
            degrees: degrees_int as i32,
            minutes: minutes_int as i32,
            seconds,
        }
    }

    pub fn to_degrees(&self) -> f64 {
        self.degrees as f64 + (self.minutes as f64 + self.seconds / 60.0) / 60.0
    }
}

impl fmt::Display for SexagesimalAngle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}° {}' {:.5}\"",
            self.degrees, self.minutes, self.seconds
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const D2R: f64 = std::f64::consts::PI / 180.0;
    /* Maximum allowable error in quantities with units of length (in meters). */
    const MAX_DIST_ERROR_M: f64 = 1e-6;
    /* Maximum allowable error in quantities with units of angle (in sec of arc).
     * 1 second of arc on the equator is ~31 meters. */
    const MAX_ANGLE_ERROR_DEF: f64 = 1e-7;
    const MAX_ANGLE_ERROR_RAD: f64 = MAX_ANGLE_ERROR_DEF * D2R;

    #[test]
    fn llhrad2deg() {
        let zeros = LLHRadians::from_array(&[0.0; 3]);

        let deg = zeros.to_degrees();
        assert_eq!(0.0, deg.latitude());
        assert_eq!(0.0, deg.longitude());
        assert_eq!(0.0, deg.height());

        let swift_home = LLHDegrees::from_array(&[37.779804, -122.391751, 60.0]);
        let rads = swift_home.to_radians();

        assert!((rads.latitude() - 0.659381970558).abs() < MAX_ANGLE_ERROR_RAD);
        assert!((rads.longitude() + 2.136139032231).abs() < MAX_ANGLE_ERROR_RAD);
        assert!(rads.height() == swift_home.height());
    }

    const LLH_VALUES: [LLHRadians; 10] = [
        LLHRadians([0.0, 0.0, 0.0]), /* On the Equator and Prime Meridian. */
        LLHRadians([0.0, 180.0 * D2R, 0.0]), /* On the Equator. */
        LLHRadians([0.0, 90.0 * D2R, 0.0]), /* On the Equator. */
        LLHRadians([0.0, -90.0 * D2R, 0.0]), /* On the Equator. */
        LLHRadians([90.0 * D2R, 0.0, 0.0]), /* North pole. */
        LLHRadians([-90.0 * D2R, 0.0, 0.0]), /* South pole. */
        LLHRadians([90.0 * D2R, 0.0, 22.0]), /* 22m above the north pole. */
        LLHRadians([-90.0 * D2R, 0.0, 22.0]), /* 22m above the south pole. */
        LLHRadians([0.0, 0.0, 22.0]), /* 22m above the Equator and Prime Meridian. */
        LLHRadians([0.0, 180.0 * D2R, 22.0]), /* 22m above the Equator. */
    ];

    /* Semi-major axis. */
    const EARTH_A: f64 = 6378137.0;
    /* Semi-minor axis. */
    const EARTH_B: f64 = 6356752.31424517929553985595703125;

    const ECEF_VALUES: [ECEF; 10] = [
        ECEF([EARTH_A, 0.0, 0.0]),
        ECEF([-EARTH_A, 0.0, 0.0]),
        ECEF([0.0, EARTH_A, 0.0]),
        ECEF([0.0, -EARTH_A, 0.0]),
        ECEF([0.0, 0.0, EARTH_B]),
        ECEF([0.0, 0.0, -EARTH_B]),
        ECEF([0.0, 0.0, (EARTH_B + 22.0)]),
        ECEF([0.0, 0.0, -(EARTH_B + 22.0)]),
        ECEF([(22.0 + EARTH_A), 0.0, 0.0]),
        ECEF([-(22.0 + EARTH_A), 0.0, 0.0]),
    ];

    #[test]
    fn wgsllh2ecef() {
        for (llh_input, expected_ecef) in LLH_VALUES.iter().zip(ECEF_VALUES.iter()) {
            let ecef = llh_input.to_ecef();

            assert!(!ecef.x().is_nan());
            assert!(!ecef.y().is_nan());
            assert!(!ecef.z().is_nan());

            let x_err = ecef.x() - expected_ecef.x();
            assert!(x_err.abs() < MAX_DIST_ERROR_M);

            let y_err = ecef.y() - expected_ecef.y();
            assert!(y_err.abs() < MAX_DIST_ERROR_M);

            let z_err = ecef.z() - expected_ecef.z();
            assert!(z_err.abs() < MAX_DIST_ERROR_M);
        }
    }

    #[test]
    fn wgsecef2llh() {
        for (ecef_input, expected_llh) in ECEF_VALUES.iter().zip(LLH_VALUES.iter()) {
            let llh = ecef_input.to_llh();

            assert!(!llh.latitude().is_nan());
            assert!(!llh.longitude().is_nan());
            assert!(!llh.height().is_nan());

            let lat_err = llh.latitude() - expected_llh.latitude();
            assert!(lat_err.abs() < MAX_ANGLE_ERROR_RAD);

            let lon_err = llh.longitude() - expected_llh.longitude();
            assert!(lon_err.abs() < MAX_ANGLE_ERROR_RAD);

            let height_err = llh.height() - expected_llh.height();
            assert!(height_err.abs() < MAX_DIST_ERROR_M);
        }
    }
}
