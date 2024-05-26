// Copyright (c) 2020-2021 Swift Navigation Inc.
// Contact: Swift Navigation <dev@swiftnav.com>
//
// This source is subject to the license found in the file 'LICENSE' which must
// be be distributed together with this source. All other rights reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND,
// EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A PARTICULAR PURPOSE.
//! Ionosphere delay calculation
//!
//! Ionospheric delays are typically modeled with the Klobuchar model. The model
//! parameters are broadcast by the GPS constellation. A function to decode the
//! parameters from the raw subframe is provided.
//!
//! # References
//!  * IS-GPS-200H, Section 20.3.3.5.2.5 and Figure 20-4

use crate::time::GpsTime;
use std::error::Error;
use std::fmt::{Display, Formatter};

/// Represents an ionosphere model
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Ionosphere(swiftnav_sys::ionosphere_t);

/// An error indicating that the iono model failed to be decoded
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct IonoDecodeFailure;

impl Display for IonoDecodeFailure {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Error decoding iono model")
    }
}

impl Error for IonoDecodeFailure {}

impl Ionosphere {
    /// Construct an ionosphere model from already decoded parameters
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        toa: GpsTime,
        a0: f64,
        a1: f64,
        a2: f64,
        a3: f64,
        b0: f64,
        b1: f64,
        b2: f64,
        b3: f64,
    ) -> Ionosphere {
        Ionosphere(swiftnav_sys::ionosphere_t {
            toa: toa.to_gps_time_t(),
            a0,
            a1,
            a2,
            a3,
            b0,
            b1,
            b2,
            b3,
        })
    }

    /// Decodes ionospheric parameters from GLS LNAV message subframe 4.
    ///
    /// The method decodes ionosphere data from GPS LNAV subframe 4 words 3-5.
    ///
    /// In inputs are the word values from Subframe 4 page 18.
    ///
    /// # References
    ///   * IS-GPS-200H, Section 20.3.3.5.1.7
    pub fn decode_parameters(words: &[u32; 8]) -> Result<Ionosphere, IonoDecodeFailure> {
        let mut iono = Ionosphere(swiftnav_sys::ionosphere_t {
            toa: GpsTime::unknown(),
            a0: 0.0,
            a1: 0.0,
            a2: 0.0,
            a3: 0.0,
            b0: 0.0,
            b1: 0.0,
            b2: 0.0,
            b3: 0.0,
        });

        let success = unsafe { swiftnav_sys::decode_iono_parameters(words, &mut iono.0) };

        if success {
            Ok(iono)
        } else {
            Err(IonoDecodeFailure)
        }
    }

    /// Calculate ionospheric delay using Klobuchar model.
    ///
    /// \param t_gps GPS time at which to calculate the ionospheric delay
    /// \param lat_u Latitude of the receiver \[rad\]
    /// \param lon_u Longitude of the receiver \[rad\]
    /// \param a Azimuth of the satellite, clockwise positive from North \[rad\]
    /// \param e Elevation of the satellite \[rad\]
    /// \param i Ionosphere parameters struct from GPS NAV data
    ///
    /// \return Ionospheric delay distance for GPS L1 frequency \[m\]
    pub fn calc_delay(&self, t: &GpsTime, lat_u: f64, lon_u: f64, a: f64, e: f64) -> f64 {
        unsafe { swiftnav_sys::calc_ionosphere(t.c_ptr(), lat_u, lon_u, a, e, &self.0) }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ionosphere::Ionosphere, time::GpsTime};

    const D2R: f64 = std::f64::consts::PI / 180.0;

    #[test]
    fn calc_ionosphere() {
        let t = GpsTime::new(1875, 479820.0).unwrap();
        let i = Ionosphere::new(
            t, 0.1583e-7, -0.7451e-8, -0.5960e-7, 0.1192e-6, 0.1290e6, -0.2130e6, 0.6554e5,
            0.3277e6,
        );
        let lat_u = -35.3 * D2R;
        let lon_u = 149.1 * D2R;
        let a = 0.0 * D2R;
        let e = 15.0 * D2R;
        let d_true = 7.202;

        let d_tol = 1e-3;

        let d_l1 = i.calc_delay(&t, lat_u, lon_u, a, e);
        let d_err = (d_l1 - d_true).abs();

        assert!(
            d_err < d_tol,
            "Distance didn't match hardcoded correct value {}. Saw: {}",
            d_true,
            d_l1
        );

        let t = GpsTime::new(1042, 593100.).unwrap();
        let i = Ionosphere::new(
            t, 0.3820e-7, 0.1490e-7, -0.1790e-6, 0.0, 0.1430e6, 0.0, -0.3280e6, 0.1130e6,
        );
        let lat_u = 40.0 * D2R;
        let lon_u = 260.0 * D2R;
        let a = 210.0 * D2R;
        let e = 20.0 * D2R;
        let d_true = 23.784;

        let d_l1 = i.calc_delay(&t, lat_u, lon_u, a, e);
        let d_err = (d_l1 - d_true).abs();

        assert!(
            d_err < d_tol,
            "Distance didn't match hardcoded correct values {}. Saw: {}",
            d_true,
            d_l1
        );

        let t = GpsTime::new(1042, 345600.).unwrap();
        let i = Ionosphere::new(
            t, 1.304e-8, 0., -5.96e-8, 5.96e-8, 1.106e5, -65540.0, -2.621e5, 3.932e5,
        );
        let lat_u = 0.70605;
        let lon_u = -0.076233;
        let a = 2.62049;
        let e = 0.2939;
        let d_true = 3.4929;

        let d_l1 = i.calc_delay(&t, lat_u, lon_u, a, e);
        let d_err = (d_l1 - d_true).abs();

        assert!(
            d_err < d_tol,
            "Distance didn't match hardcoded correct values {}. Saw: {}",
            d_true,
            d_l1
        );
    }

    #[test]
    fn test_decode_iono_parameters() {
        const TOL: f64 = 1e-12;
        // struct {
        // u32 frame_words[8];
        // ionosphere_t result;
        // } t_case = {.frame_words =
        let frame_words: [u32; 8] = [
            /* 4th SF real data at 11-May-2016 */
            0x1e0300c9, 0x7fff8c24, 0x23fbdc2, 0, 0, 0, 0, 0,
        ];
        let result = Ionosphere::new(
            /* reference data provided by u-blox receiver */
            GpsTime::new_unchecked(0, 0.),
            0.0000000111758,
            0.0000000223517,
            -0.0000000596046,
            -0.0000001192092,
            98304.0,
            131072.0,
            -131072.0,
            -589824.0,
        );

        let i = Ionosphere::decode_parameters(&frame_words).unwrap();

        assert!(
            (i.0.a0 - result.0.a0).abs() < TOL,
            "alfa 0 == {:30.20}, expected {:30.20}, tolerance = {:30.20}",
            i.0.a0,
            result.0.a0,
            TOL
        );
        assert!(
            (i.0.a1 - result.0.a1).abs() < TOL,
            "alfa 1 == {:30.20}, expected {:30.20}, tolerance = {:30.20}",
            i.0.a1,
            result.0.a1,
            TOL
        );
        assert!(
            (i.0.a2 - result.0.a2).abs() < TOL,
            "alfa 2 == {:30.20}, expected {:30.20}, tolerance = {:30.20}",
            i.0.a2,
            result.0.a2,
            TOL
        );
        assert!(
            (i.0.a3 - result.0.a3).abs() < TOL,
            "alfa 3 == {:30.20}, expected {:30.20}, tolerance = {:30.20}",
            i.0.a3,
            result.0.a3,
            TOL
        );
        assert!(
            (i.0.b0 - result.0.b0).abs() < TOL,
            "beta 0 == {:30.20}, expected {:30.20}, tolerance = {:30.20}",
            i.0.b0,
            result.0.b0,
            TOL
        );
        assert!(
            (i.0.b1 - result.0.b1).abs() < TOL,
            "beta 1 == {:30.20}, expected {:30.20}, tolerance = {:30.20}",
            i.0.b1,
            result.0.b1,
            TOL,
        );
        assert!(
            (i.0.b2 - result.0.b2).abs() < TOL,
            "beta 2 == {:30.20}, expected {:30.20}, tolerance = {:30.20}",
            i.0.b2,
            result.0.b2,
            TOL,
        );
        assert!(
            (i.0.b3 - result.0.b3).abs() < TOL,
            "beta 3 == {:30.20}, expected {:30.20}, tolerance = {:30.20}",
            i.0.b3,
            result.0.b3,
            TOL,
        );
    }
}
