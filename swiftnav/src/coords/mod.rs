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
//! These four coordinates types are defined:
//!  * [`LLHDegrees`]/[`LLHRadians`] - Geodetic coordinates, Latitude Lontitude Height
//!  * [`ECEF`] - Cartesian coordinates, Earth Centered, Earth Fixed
//!  * [`NED`] - Local direction coordinates, North East Down
//!  * [`AzimuthElevation`] - Relative direction coordinates, Azimith Elevation
//!
//! # Geodetic to Cartesian
//!
//! Conversion from geodetic coordinates latitude, longitude and height
//! ($\phi$, $\lambda$, $h$) into Cartesian coordinates ($X$, $Y$, $Z$) can be
//! achieved with the following formulae:
//! $$X = (N(\phi) + h) \cos{\phi}\cos{\lambda}$$
//! $$Y = (N(\phi) + h) \cos{\phi}\sin{\lambda}$$
//! $$Z = \left[(1-e^2)N(\phi) + h\right] \sin{\phi}$$
//!
//! Where the 'radius of curvature', $N(\phi)$, is defined as:
//!
//! $$N(\phi) = \frac{a}{\sqrt{1-e^2\sin^2 \phi}}$$
//!
//! and $a$ is the WGS84 semi-major axis and $e$ is the WGS84
//! eccentricity.
//!
//! # Cartesian to Geodetic
//!
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
//! ## References
//! * "A comparison of methods used in rectangular to Geodetic Coordinates
//!   Transformations", Burtch R. R. (2006), American Congress for Surveying
//!   and Mapping Annual Conference. Orlando, Florida.
//! * "Transformation from Cartesian to Geodetic Coordinates Accelerated by
//!   Halley’s Method", T. Fukushima (2006), Journal of Geodesy.
//!
//! # Examples
//!
//! ```rust
//! # use swiftnav::coords::{ECEF, LLHDegrees};
//! # use float_eq::assert_float_eq;
//! let position_llh = LLHDegrees::new(37.791, -122.395, 0.0);
//! let position_ecef = position_llh.to_ecef();
//!
//! // Approximate velocity of the local crust in meters/year
//! let velocity_vector = ECEF::new(-0.02, 0.03, 0.01);
//! let velocity_ned = velocity_vector.ned_vector_at(&position_ecef);
//! assert_float_eq!(velocity_ned.n(), 0.02, abs <= 0.01);
//! assert_float_eq!(velocity_ned.e(), -0.03, abs <= 0.01);
//! assert_float_eq!(velocity_ned.d(), 0.0, abs <= 0.01);
//!
//! let years = 3.5;
//! let future_ecef = position_ecef + (years * velocity_vector);
//!
//! let sutro_tower = LLHDegrees::new(37.75523, -122.45284, 254.2);
//! let bearing = position_ecef.azel_of(&sutro_tower.into());
//! assert_float_eq!(bearing.az(), 4.05, abs <= 0.01);
//! assert_float_eq!(bearing.el(), 0.04, abs <= 0.01);
//! ```

mod ecef;
mod ellipsoid;
mod llh;
mod ned;

pub use ecef::*;
pub use ellipsoid::*;
pub use llh::*;
pub use ned::*;

use crate::{reference_frame::ReferenceFrame, time::GpsTime};
use nalgebra::Vector2;

/// WGS84 local horizontal coordinates consisting of an Azimuth and Elevation, with angles stored as radians
///
/// Azimuth can range from $0$ to $2\pi$. North has an azimuth of $0$, east has an azimuth of $\frac{\pi}{2}$
///
/// Elevation can range from $-\frac{\pi}{2}$ to $\frac{\pi}{2}$. Up has an elevation of $\frac{\pi}{2}$, down an elevation of $-\frac{\pi}{2}$
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct AzimuthElevation(Vector2<f64>);

impl AzimuthElevation {
    /// Create an [`AzimuthElevation`] object from the given azimuth and elevation
    #[must_use]
    pub fn new(az: f64, el: f64) -> AzimuthElevation {
        Self(Vector2::new(az, el))
    }

    /// Get a reference to the inner array storing the data
    #[must_use]
    pub fn as_array(&self) -> &[f64; 2] {
        &self.0.data.0[0]
    }

    /// Get a mutable reference to the inner array storing the data
    #[must_use]
    pub fn as_array_mut(&mut self) -> &mut [f64; 2] {
        &mut self.0.data.0[0]
    }

    /// Get a reference to the inner [`Vector2`] storing the data
    #[must_use]
    pub fn as_vector(&self) -> &Vector2<f64> {
        &self.0
    }

    /// Get a mutable reference to the inner [`Vector2`] storing the data
    #[must_use]
    pub fn as_vector_mut(&mut self) -> &mut Vector2<f64> {
        &mut self.0
    }

    /// Get the Azimuth component
    #[must_use]
    pub fn az(&self) -> f64 {
        self.0.x
    }

    /// Get the Elevation component
    #[must_use]
    pub fn el(&self) -> f64 {
        self.0.y
    }
}

impl From<[f64; 2]> for AzimuthElevation {
    fn from(array: [f64; 2]) -> Self {
        Self::new(array[0], array[1])
    }
}

impl From<&[f64; 2]> for AzimuthElevation {
    fn from(array: &[f64; 2]) -> Self {
        Self::new(array[0], array[1])
    }
}

impl From<Vector2<f64>> for AzimuthElevation {
    fn from(vector: Vector2<f64>) -> Self {
        Self(vector)
    }
}

impl From<(f64, f64)> for AzimuthElevation {
    fn from((x, y): (f64, f64)) -> Self {
        Self::new(x, y)
    }
}

impl AsRef<[f64; 2]> for AzimuthElevation {
    fn as_ref(&self) -> &[f64; 2] {
        self.as_array()
    }
}

impl AsRef<Vector2<f64>> for AzimuthElevation {
    fn as_ref(&self) -> &Vector2<f64> {
        self.as_vector()
    }
}

impl AsMut<[f64; 2]> for AzimuthElevation {
    fn as_mut(&mut self) -> &mut [f64; 2] {
        self.as_array_mut()
    }
}

impl AsMut<Vector2<f64>> for AzimuthElevation {
    fn as_mut(&mut self) -> &mut Vector2<f64> {
        self.as_vector_mut()
    }
}

/// Complete coordinate used for transforming between reference frames
///
/// Velocities are optional, but when present they will be transformed
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Coordinate {
    reference_frame: ReferenceFrame,
    position: ECEF,
    velocity: Option<ECEF>,
    epoch: GpsTime,
}

impl Coordinate {
    /// Create a new [`Coordinate`] object
    #[must_use]
    pub fn new(
        reference_frame: ReferenceFrame,
        position: ECEF,
        velocity: Option<ECEF>,
        epoch: GpsTime,
    ) -> Self {
        Coordinate {
            reference_frame,
            position,
            velocity,
            epoch,
        }
    }

    /// Create a new [`Coordinate`] object with no velocity value
    #[must_use]
    pub fn without_velocity(
        reference_frame: ReferenceFrame,
        position: ECEF,
        epoch: GpsTime,
    ) -> Self {
        Coordinate {
            reference_frame,
            position,
            velocity: None,
            epoch,
        }
    }

    /// Create a new [`Coordinate`] object with a velocity
    #[must_use]
    pub fn with_velocity(
        reference_frame: ReferenceFrame,
        position: ECEF,
        velocity: ECEF,
        epoch: GpsTime,
    ) -> Self {
        Coordinate {
            reference_frame,
            position,
            velocity: Some(velocity),
            epoch,
        }
    }

    /// Get the reference frame of the coordinate
    #[must_use]
    pub fn reference_frame(&self) -> &ReferenceFrame {
        &self.reference_frame
    }

    /// Get the position of the coordinate
    #[must_use]
    pub fn position(&self) -> ECEF {
        self.position
    }

    /// Get the velocity of the coordinate
    #[must_use]
    pub fn velocity(&self) -> Option<ECEF> {
        self.velocity
    }

    /// Get the epoch of the coordinate
    #[must_use]
    pub fn epoch(&self) -> GpsTime {
        self.epoch
    }

    /// Use the velocity term to adjust the epoch of the coordinate.
    /// When a coordinate has no velocity the position won't be changed.
    #[must_use]
    pub fn adjust_epoch(self, new_epoch: &GpsTime) -> Self {
        let dt =
            new_epoch.to_fractional_year_hardcoded() - self.epoch.to_fractional_year_hardcoded();
        let v = self.velocity.unwrap_or_default();

        Coordinate {
            position: self.position + dt * v,
            velocity: self.velocity,
            epoch: *new_epoch,
            reference_frame: self.reference_frame,
        }
    }
}

#[cfg(test)]
mod tests {
    use float_eq::assert_float_eq;

    use crate::time::UtcTime;

    use super::*;
    use proptest::prelude::*;

    /* Maximum allowable error in quantities with units of length (in meters). */
    const MAX_DIST_ERROR_M: f64 = 1e-6;
    /* Maximum allowable error in quantities with units of angle (in sec of arc).
     * 1 second of arc on the equator is ~31 meters. */
    const MAX_ANGLE_ERROR_SECS: f64 = 1e-7;
    const MAX_ANGLE_ERROR_RAD: f64 = (MAX_ANGLE_ERROR_SECS / 3600.0).to_radians();

    #[test]
    fn llhrad2deg() {
        let zeros = LLHRadians::default();

        let deg = zeros.to_degrees();
        assert_eq!(0.0, deg.latitude());
        assert_eq!(0.0, deg.longitude());
        assert_eq!(0.0, deg.height());

        let swift_home: LLHDegrees = [37.779804, -122.391751, 60.0].into();
        let rads = swift_home.to_radians();

        assert!((rads.latitude() - 0.659381970558).abs() < MAX_ANGLE_ERROR_RAD);
        assert!((rads.longitude() + 2.136139032231).abs() < MAX_ANGLE_ERROR_RAD);
        assert!(
            rads.height() == swift_home.height(),
            "rads.height() = {}, swift_home.height() = {}",
            rads.height(),
            swift_home.height()
        );
    }

    const LLH_VALUES: [[f64; 3]; 10] = [
        [0.0, 0.0, 0.0],                     /* On the Equator and Prime Meridian. */
        [0.0, 180.0_f64.to_radians(), 0.0],  /* On the Equator. */
        [0.0, 90.0_f64.to_radians(), 0.0],   /* On the Equator. */
        [0.0, -90.0_f64.to_radians(), 0.0],  /* On the Equator. */
        [90.0_f64.to_radians(), 0.0, 0.0],   /* North pole. */
        [-90.0_f64.to_radians(), 0.0, 0.0],  /* South pole. */
        [90.0_f64.to_radians(), 0.0, 22.0],  /* 22m above the north pole. */
        [-90.0_f64.to_radians(), 0.0, 22.0], /* 22m above the south pole. */
        [0.0, 0.0, 22.0],                    /* 22m above the Equator and Prime Meridian. */
        [0.0, 180.0_f64.to_radians(), 22.0], /* 22m above the Equator. */
    ];

    /* Semi-major axis. */
    const EARTH_A: f64 = 6378137.0;
    /* Semi-minor axis. */
    const EARTH_B: f64 = 6_356_752.314_245_179;

    const ECEF_VALUES: [[f64; 3]; 10] = [
        [EARTH_A, 0.0, 0.0],
        [-EARTH_A, 0.0, 0.0],
        [0.0, EARTH_A, 0.0],
        [0.0, -EARTH_A, 0.0],
        [0.0, 0.0, EARTH_B],
        [0.0, 0.0, -EARTH_B],
        [0.0, 0.0, (EARTH_B + 22.0)],
        [0.0, 0.0, -(EARTH_B + 22.0)],
        [(22.0 + EARTH_A), 0.0, 0.0],
        [-(22.0 + EARTH_A), 0.0, 0.0],
    ];

    #[test]
    fn llh2ecef() {
        for (llh_input, expected_ecef) in LLH_VALUES.iter().zip(ECEF_VALUES.iter()) {
            let llh_input: LLHRadians = llh_input.into();
            let expected_ecef: ECEF = expected_ecef.into();

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
    fn ecef2llh() {
        for (ecef_input, expected_llh) in ECEF_VALUES.iter().zip(LLH_VALUES.iter()) {
            let ecef_input: ECEF = ecef_input.into();
            let expected_llh: LLHRadians = expected_llh.into();

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

    #[test]
    fn llh2ecef2llh() {
        for llh_input in LLH_VALUES.iter() {
            let llh_input: LLHRadians = llh_input.into();
            let llh_output = llh_input.to_ecef().to_llh();

            assert!(!llh_output.latitude().is_nan());
            assert!(!llh_output.longitude().is_nan());
            assert!(!llh_output.height().is_nan());

            let lat_err = llh_input.latitude() - llh_output.latitude();
            assert!(lat_err.abs() < MAX_ANGLE_ERROR_RAD);

            let lon_err = llh_input.longitude() - llh_output.longitude();
            assert!(lon_err.abs() < MAX_ANGLE_ERROR_RAD);

            let hgt_err = llh_input.height() - llh_output.height();
            assert!(hgt_err.abs() < MAX_DIST_ERROR_M);
        }
    }

    #[test]
    fn ecef2llh2ecef() {
        for ecef_input in ECEF_VALUES.iter() {
            let ecef_input: ECEF = ecef_input.into();
            let ecef_output = ecef_input.to_llh().to_ecef();

            assert!(!ecef_output.x().is_nan());
            assert!(!ecef_output.y().is_nan());
            assert!(!ecef_output.z().is_nan());

            let x_err = ecef_input.x() - ecef_output.x();
            assert!(x_err.abs() < MAX_DIST_ERROR_M);

            let y_err = ecef_input.y() - ecef_output.y();
            assert!(y_err.abs() < MAX_DIST_ERROR_M);

            let z_err = ecef_input.z() - ecef_output.z();
            assert!(z_err.abs() < MAX_DIST_ERROR_M);
        }
    }

    #[test]
    fn ecef2ned() {
        let ecef_position = LLHDegrees::new(0.0, 0.0, 0.0).to_ecef();

        // Make sure that the X unit vector at null-island points up
        let ecef_vec = ECEF::new(1.0, 0.0, 0.0);
        let ned_vec = ecef_vec.ned_vector_at(&ecef_position);
        assert_float_eq!(ned_vec.n(), 0.0, abs <= MAX_DIST_ERROR_M);
        assert_float_eq!(ned_vec.e(), 0.0, abs <= MAX_DIST_ERROR_M);
        assert_float_eq!(ned_vec.d(), -1.0, abs <= MAX_DIST_ERROR_M);

        // Make sure that the Y unit vector at null-island points east
        let ecef_vec = ECEF::new(0.0, 1.0, 0.0);
        let ned_vec = ecef_vec.ned_vector_at(&ecef_position);
        assert_float_eq!(ned_vec.n(), 0.0, abs <= MAX_DIST_ERROR_M);
        assert_float_eq!(ned_vec.e(), 1.0, abs <= MAX_DIST_ERROR_M);
        assert_float_eq!(ned_vec.d(), 0.0, abs <= MAX_DIST_ERROR_M);

        // Make sure that the Z unit vector at null-island points north
        let ecef_vec = ECEF::new(0.0, 0.0, 1.0);
        let ned_vec = ecef_vec.ned_vector_at(&ecef_position);
        assert_float_eq!(ned_vec.n(), 1.0, abs <= MAX_DIST_ERROR_M);
        assert_float_eq!(ned_vec.e(), 0.0, abs <= MAX_DIST_ERROR_M);
        assert_float_eq!(ned_vec.d(), 0.0, abs <= MAX_DIST_ERROR_M);

        // Move to the north pole
        let ecef_position = LLHDegrees::new(90.0, 0.0, 0.0).to_ecef();

        // Make sure that the X unit vector at null-island points south
        let ecef_vec = ECEF::new(1.0, 0.0, 0.0);
        let ned_vec = ecef_vec.ned_vector_at(&ecef_position);
        assert_float_eq!(ned_vec.n(), -1.0, abs <= MAX_DIST_ERROR_M);
        assert_float_eq!(ned_vec.e(), 0.0, abs <= MAX_DIST_ERROR_M);
        assert_float_eq!(ned_vec.d(), 0.0, abs <= MAX_DIST_ERROR_M);

        // Make sure that the Y unit vector at null-island points east
        let ecef_vec = ECEF::new(0.0, 1.0, 0.0);
        let ned_vec = ecef_vec.ned_vector_at(&ecef_position);
        assert_float_eq!(ned_vec.n(), 0.0, abs <= MAX_DIST_ERROR_M);
        assert_float_eq!(ned_vec.e(), 1.0, abs <= MAX_DIST_ERROR_M);
        assert_float_eq!(ned_vec.d(), 0.0, abs <= MAX_DIST_ERROR_M);

        // Make sure that the Z unit vector at null-island points up
        let ecef_vec = ECEF::new(0.0, 0.0, 1.0);
        let ned_vec = ecef_vec.ned_vector_at(&ecef_position);
        assert_float_eq!(ned_vec.n(), 0.0, abs <= MAX_DIST_ERROR_M);
        assert_float_eq!(ned_vec.e(), 0.0, abs <= MAX_DIST_ERROR_M);
        assert_float_eq!(ned_vec.d(), -1.0, abs <= MAX_DIST_ERROR_M);
    }

    #[test]
    fn coordinate_epoch() {
        let initial_epoch = UtcTime::from_parts(2020, 1, 1, 0, 0, 0.).to_gps_hardcoded();
        let new_epoch = UtcTime::from_parts(2021, 1, 1, 0, 0, 0.).to_gps_hardcoded();
        let initial_coord = Coordinate::with_velocity(
            ReferenceFrame::ITRF2020,
            ECEF::new(0.0, 0.0, 0.0),
            ECEF::new(1.0, 2.0, 3.0),
            initial_epoch,
        );

        let new_coord = initial_coord.clone().adjust_epoch(&new_epoch);

        assert_eq!(initial_coord.reference_frame, new_coord.reference_frame);
        assert_float_eq!(new_coord.position.x(), 1.0, abs <= 0.001);
        assert_float_eq!(new_coord.position.y(), 2.0, abs <= 0.001);
        assert_float_eq!(new_coord.position.z(), 3.0, abs <= 0.001);
        assert_float_eq!(new_coord.velocity.unwrap().x(), 1.0, abs <= 0.001);
        assert_float_eq!(new_coord.velocity.unwrap().y(), 2.0, abs <= 0.001);
        assert_float_eq!(new_coord.velocity.unwrap().z(), 3.0, abs <= 0.001);
        assert_eq!(new_epoch, new_coord.epoch());
    }

    // Property-based tests using proptest
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]

        /// Property: Converting LLH->ECEF->LLH should always result in the original value
        #[test]
        fn prop_llh2ecef2llh_identity(lat in -90.0..90.0, lon in -180.0..180.0, height in (-0.5*EARTH_A)..(4.0*EARTH_A)) {
            let llh_input = LLHDegrees::new(lat, lon, height).to_radians();
            let ecef = llh_input.to_ecef();
            let llh_output = ecef.to_llh();

            assert!(!llh_output.latitude().is_nan());
            assert!(!llh_output.longitude().is_nan());
            assert!(!llh_output.height().is_nan());

            let lat_err = llh_input.latitude() - llh_output.latitude();
            assert!(lat_err.abs() < MAX_ANGLE_ERROR_RAD,
                "Converting random WGS84 LLH to ECEF and back again does not return the original values. Initial: {:?}, ECEF: {:?}, Final: {:?}, Lat error (rad): {}", llh_input, ecef, llh_output, lat_err);

            let lon_err = llh_input.longitude() - llh_output.longitude();
            assert!(lon_err.abs() < MAX_ANGLE_ERROR_RAD,
                "Converting random WGS84 LLH to ECEF and back again does not return the original values. Initial: {:?}, ECEF: {:?}, Final: {:?}, Lon error (rad): {}", llh_input, ecef, llh_output, lon_err);

            let hgt_err = llh_input.height() - llh_output.height();
            assert!(hgt_err.abs() < MAX_DIST_ERROR_M,
                "Converting random WGS84 LLH to ECEF and back again does not return the original values. Initial: {:?}, ECEF: {:?}, Final: {:?}, Height error (mm): {}", llh_input, ecef, llh_output, hgt_err*1000.0);
        }

        /// Property: Converting ECEF->LLH->ECEF should always result in the original value
        #[test]
        fn prop_ecef2llh2ecef_identity(x in (-4.0*EARTH_A)..(4.0*EARTH_A), y in (-4.0*EARTH_A)..(4.0*EARTH_A), z in (-4.0*EARTH_A)..(4.0*EARTH_A)) {
            // We know our implementation breaks down as the coordinates get near the center
            // of the earth, so skip over coordinates that aren't at least half way to the ellipsoid
            prop_assume!((x*x + y*y + z*z).sqrt().abs() > 0.5*EARTH_A);

            let ecef_input = ECEF::new(x, y, z);
            let llh = ecef_input.to_llh();
            let ecef_output = llh.to_ecef();

            assert!(!ecef_output.x().is_nan());
            assert!(!ecef_output.y().is_nan());
            assert!(!ecef_output.z().is_nan());

            let x_err = ecef_input.x() - ecef_output.x();
            assert!(x_err.abs() < MAX_DIST_ERROR_M,
                "Converting random WGS84 ECEF to LLH and back again does not return the original values. Initial: {:?}, LLH: {:?}, Final: {:?}, X error (mm): {}", ecef_input, llh.to_degrees(), ecef_output, x_err*1000.0);

            let y_err = ecef_input.y() - ecef_output.y();
            assert!(y_err.abs() < MAX_DIST_ERROR_M,
                "Converting random WGS84 ECEF to LLH and back again does not return the original values. Initial: {:?}, LLH: {:?}, Final: {:?}, Y error (mm): {}", ecef_input, llh.to_degrees(), ecef_output, y_err*1000.0);

            let z_err = ecef_input.z() - ecef_output.z();
            assert!(z_err.abs() < MAX_DIST_ERROR_M,
                "Converting random WGS84 ECEF to LLH and back again does not return the original values. Initial: {:?}, LLH: {:?}, Final: {:?}, Z error (mm): {}", ecef_input, llh.to_degrees(), ecef_output, z_err*1000.0);
        }

        /// Property: Converting ECEF->NED using the same point should always result in a value of 0 NED
        #[test]
        fn prop_ecef2ned_identity(x in -1e8..1e8, y in -1e8..1e8, z in -1e8..1e8) {
            let ecef = ECEF::new(x, y, z);
            let ned_output = ecef.ned_to(&ecef);

            assert!(!ned_output.n().is_nan());
            assert!(!ned_output.e().is_nan());
            assert!(!ned_output.d().is_nan());

            assert!(ned_output.n().abs() < 1e-8,
                "NED vector to reference ECEF point has nonzero element north: {} mm (point was {:?})",
                ned_output.n()*1000.0,
                ecef);
            assert!(ned_output.e().abs() < 1e-8,
                "NED vector to reference ECEF point has nonzero element east: {} mm (point was {:?})",
                ned_output.e()*1000.0,
                ecef);
            assert!(ned_output.d().abs() < 1e-8,
                "NED vector to reference ECEF point has nonzero element down: {} mm (point was {:?})",
                ned_output.d()*1000.0,
                ecef);
        }
    }
}
