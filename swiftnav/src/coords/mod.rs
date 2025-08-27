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
//!  * [LLHDegrees]/[LLHRadians] - Geodetic coordinates, Latitude Lontitude Height
//!  * [ECEF] - Cartesian coordinates, Earth Centered, Earth Fixed
//!  * [NED] - Local direction coordinates, North East Down
//!  * [AzimuthElevation] - Relative direction coordinates, Azimith Elevation
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

use crate::{
    reference_frame::{get_transformation, ReferenceFrame, TransformationNotFound},
    time::GpsTime,
};
use nalgebra::{ArrayStorage, Vector2};

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
        AzimuthElevation(Vector2::new(az, el))
    }

    /// Create an [`AzimuthElevation`] object from an array
    ///
    /// Element 0 is azimuth, element 1 is elevation
    #[must_use]
    pub const fn from_array(array: &[f64; 2]) -> AzimuthElevation {
        AzimuthElevation(Vector2::from_array_storage(ArrayStorage([*array; 1])))
    }

    /// Get the Azimuth component
    #[must_use]
    pub fn az(&self) -> f64 {
        self.0[0]
    }

    /// Get the Elevation component
    #[must_use]
    pub fn el(&self) -> f64 {
        self.0[1]
    }
}

/// Complete coordinate used for transforming between reference frames
///
/// Velocities are optional, but when present they will be transformed
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
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

    /// Create a new [`Coordinate`] object with a velocity value
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

    /// Create a new [`Coordinate`] object with no velocity
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
    pub fn reference_frame(&self) -> ReferenceFrame {
        self.reference_frame
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
    pub fn adjust_epoch(&self, new_epoch: &GpsTime) -> Self {
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

    /// Transform the coordinate from into a new reference frame
    pub fn transform_to(&self, new_frame: ReferenceFrame) -> Result<Self, TransformationNotFound> {
        let transformation = get_transformation(self.reference_frame, new_frame)?;
        Ok(transformation.transform(self))
    }
}

#[cfg(test)]
mod tests {
    use float_eq::assert_float_eq;

    use crate::time::UtcTime;

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
        assert!(
            rads.height() == swift_home.height(),
            "rads.height() = {}, swift_home.height() = {}",
            rads.height(),
            swift_home.height()
        );
    }

    const LLH_VALUES: [LLHRadians; 10] = [
        LLHRadians::from_array(&[0.0, 0.0, 0.0]), /* On the Equator and Prime Meridian. */
        LLHRadians::from_array(&[0.0, 180.0 * D2R, 0.0]), /* On the Equator. */
        LLHRadians::from_array(&[0.0, 90.0 * D2R, 0.0]), /* On the Equator. */
        LLHRadians::from_array(&[0.0, -90.0 * D2R, 0.0]), /* On the Equator. */
        LLHRadians::from_array(&[90.0 * D2R, 0.0, 0.0]), /* North pole. */
        LLHRadians::from_array(&[-90.0 * D2R, 0.0, 0.0]), /* South pole. */
        LLHRadians::from_array(&[90.0 * D2R, 0.0, 22.0]), /* 22m above the north pole. */
        LLHRadians::from_array(&[-90.0 * D2R, 0.0, 22.0]), /* 22m above the south pole. */
        LLHRadians::from_array(&[0.0, 0.0, 22.0]), /* 22m above the Equator and Prime Meridian. */
        LLHRadians::from_array(&[0.0, 180.0 * D2R, 22.0]), /* 22m above the Equator. */
    ];

    /* Semi-major axis. */
    const EARTH_A: f64 = 6378137.0;
    /* Semi-minor axis. */
    const EARTH_B: f64 = 6_356_752.314_245_179;

    const ECEF_VALUES: [ECEF; 10] = [
        ECEF::from_array(&[EARTH_A, 0.0, 0.0]),
        ECEF::from_array(&[-EARTH_A, 0.0, 0.0]),
        ECEF::from_array(&[0.0, EARTH_A, 0.0]),
        ECEF::from_array(&[0.0, -EARTH_A, 0.0]),
        ECEF::from_array(&[0.0, 0.0, EARTH_B]),
        ECEF::from_array(&[0.0, 0.0, -EARTH_B]),
        ECEF::from_array(&[0.0, 0.0, (EARTH_B + 22.0)]),
        ECEF::from_array(&[0.0, 0.0, -(EARTH_B + 22.0)]),
        ECEF::from_array(&[(22.0 + EARTH_A), 0.0, 0.0]),
        ECEF::from_array(&[-(22.0 + EARTH_A), 0.0, 0.0]),
    ];

    #[test]
    fn llh2ecef() {
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
    fn ecef2llh() {
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

        let new_coord = initial_coord.adjust_epoch(&new_epoch);

        assert_eq!(initial_coord.reference_frame, new_coord.reference_frame);
        assert_float_eq!(new_coord.position.x(), 1.0, abs <= 0.001);
        assert_float_eq!(new_coord.position.y(), 2.0, abs <= 0.001);
        assert_float_eq!(new_coord.position.z(), 3.0, abs <= 0.001);
        assert_float_eq!(new_coord.velocity.unwrap().x(), 1.0, abs <= 0.001);
        assert_float_eq!(new_coord.velocity.unwrap().y(), 2.0, abs <= 0.001);
        assert_float_eq!(new_coord.velocity.unwrap().z(), 3.0, abs <= 0.001);
        assert_eq!(new_epoch, new_coord.epoch());
    }
}
