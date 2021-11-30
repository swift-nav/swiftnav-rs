// Copyright (c) 2020-2021 Swift Navigation Inc.
// Contact: Swift Navigation <dev@swiftnav.com>
//
// This source is subject to the license found in the file 'LICENSE' which must
// be be distributed together with this source. All other rights reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND,
// EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A PARTICULAR PURPOSE.
//! Decoding and evaluation of satellite almanacs
//!
//! GNSS satellites broadcast almanac data which contains course position information
//! as well as satellite health information. Libswiftnav is able to decode the raw
//! ephemeris data and then evaluate the ephemeris.
//!
//! Satellite almanac data is considered valid for a longer time than ephemeris
//! data, but the satellite position is also much less accurate. As such it should
//! only be used for very rough estimates or for determining if a satellite is visible
//! at a particular location and time. For accurate positioning the broadcast ephemeris
//! should be used instead.

use std::{error::Error, fmt};

use crate::{
    coords::{AzimuthElevation, ECEF},
    signal::{Constellation, GnssSignal, InvalidGnssSignal},
    time::{GpsTime, InvalidGpsTime},
};

/// Representation of a satellite state from evaluating the almanac at a
/// certain time.
pub struct SatelliteState {
    /// Calculated satellite position, in meters
    pub pos: ECEF,
    /// Calculated satellite velocity, in meters/second
    pub vel: ECEF,
    /// Calculated satellite acceleration, meters/second/second
    pub acc: ECEF,
    /// Calculated satellite clock error, in seconds
    pub clock_err: f64,
    /// Calculated satellite clock error rate, in seconds/second
    pub clock_rate_err: f64,
}

/// An error indicating that the constellation of an almanac is
/// not currently supported
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct UnsupportedConstellation(Constellation);

impl fmt::Display for UnsupportedConstellation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unsupported almanac constellation ({:?})", self.0)
    }
}

impl Error for UnsupportedConstellation {}

/// An error indicating that an almanac was evaluated at a time that
/// it was not valid at
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct InvalidTime(GpsTime);

impl fmt::Display for InvalidTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid time ({:?})", self)
    }
}

impl Error for InvalidTime {}

/// An error indicating that the raw data being decoded as almanac
/// data was not valid and decoding failed
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct DecodingError;

impl fmt::Display for DecodingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Almanac decoding error")
    }
}

impl Error for DecodingError {}

/// Orbital terms of an almanac
pub enum AlmanacTerms {
    Kepler(swiftnav_sys::almanac_kepler_t),
    Xyz(swiftnav_sys::almanac_xyz_t),
}

impl AlmanacTerms {
    /// Create new keplarian almanac terms from already decoded data
    #[allow(clippy::too_many_arguments)]
    pub fn new_kepler(
        m0: f64,
        ecc: f64,
        sqrta: f64,
        omega0: f64,
        omegadot: f64,
        w: f64,
        inc: f64,
        af0: f64,
        af1: f64,
    ) -> AlmanacTerms {
        AlmanacTerms::Kepler(swiftnav_sys::almanac_kepler_t {
            m0,
            ecc,
            sqrta,
            omega0,
            omegadot,
            w,
            inc,
            af0,
            af1,
        })
    }

    /// Create new XYZ almanac terms from already decoded data
    pub fn new_xyz(pos: [f64; 3], vel: [f64; 3], acc: [f64; 3]) -> AlmanacTerms {
        AlmanacTerms::Xyz(swiftnav_sys::almanac_xyz_t { pos, vel, acc })
    }
}

/// Representation of a satellite almanac
pub struct Almanac(swiftnav_sys::almanac_t);

impl Almanac {
    /// Create new almanac from already decoded data
    pub fn new(
        sid: GnssSignal,
        toa: GpsTime,
        ura: f32,
        fit_interval: u32,
        valid: u8,
        health_bits: u8,
        terms: AlmanacTerms,
    ) -> Result<Almanac, UnsupportedConstellation> {
        match sid.to_constellation() {
            Constellation::Gps | Constellation::Sbas => Ok(Almanac(swiftnav_sys::almanac_t {
                sid: sid.to_gnss_signal_t(),
                toa: toa.to_gps_time_t(),
                ura,
                fit_interval,
                valid,
                health_bits,
                data: match terms {
                    AlmanacTerms::Kepler(kepler) => {
                        swiftnav_sys::almanac_t__bindgen_ty_1 { kepler }
                    }
                    AlmanacTerms::Xyz(xyz) => swiftnav_sys::almanac_t__bindgen_ty_1 { xyz },
                },
            })),
            other_constellation => Err(UnsupportedConstellation(other_constellation)),
        }
    }

    /// Decodes almanac from GPS LNAV message subframe words 3-10.
    ///
    /// References:
    /// -# IS-GPS-200D, Section 20.3.3.5
    pub fn decode_gps(words: &[u32; 8]) -> Result<Almanac, DecodingError> {
        let mut almanac = unsafe { std::mem::zeroed::<Almanac>() };

        let result = unsafe { swiftnav_sys::almanac_decode(words, almanac.mut_c_ptr()) };

        if result {
            Ok(almanac)
        } else {
            Err(DecodingError)
        }
    }

    pub(crate) fn c_ptr(&self) -> *const swiftnav_sys::almanac_t {
        &self.0
    }

    pub(crate) fn mut_c_ptr(&mut self) -> *mut swiftnav_sys::almanac_t {
        &mut self.0
    }

    /// Calculate satellite position, velocity and clock offset from an almanac
    pub fn calc_satellite_state(&self, t: GpsTime) -> Result<SatelliteState, InvalidTime> {
        let mut sat = SatelliteState {
            pos: ECEF::default(),
            vel: ECEF::default(),
            acc: ECEF::default(),
            clock_err: 0.0,
            clock_rate_err: 0.0,
        };

        let result = unsafe {
            swiftnav_sys::calc_sat_state_almanac(
                self.c_ptr(),
                t.c_ptr(),
                sat.pos.as_mut_array_ref(),
                sat.vel.as_mut_array_ref(),
                sat.acc.as_mut_array_ref(),
                &mut sat.clock_err,
                &mut sat.clock_rate_err,
            )
        };

        if result == 0 {
            Ok(sat)
        } else {
            Err(InvalidTime(t))
        }
    }

    /// Calculate the azimuth and elevation of a satellite from a reference
    /// position given the satellite almanac
    pub fn calc_satellite_az_el(
        &self,
        t: GpsTime,
        pos: ECEF,
    ) -> Result<AzimuthElevation, InvalidTime> {
        let mut azel = AzimuthElevation::default();

        let result = unsafe {
            swiftnav_sys::calc_sat_az_el_almanac(
                self.c_ptr(),
                t.c_ptr(),
                pos.as_array_ref(),
                &mut azel.az,
                &mut azel.el,
            )
        };

        if result == 0 {
            Ok(azel)
        } else {
            Err(InvalidTime(t))
        }
    }

    /// Calculate the Doppler shift of a satellite as observed at a reference
    /// position given the satellite almanac
    pub fn calc_satellite_doppler(&self, t: GpsTime, pos: ECEF) -> Result<f64, InvalidTime> {
        let mut doppler = 0.0;

        let result = unsafe {
            swiftnav_sys::calc_sat_doppler_almanac(
                self.c_ptr(),
                t.c_ptr(),
                pos.as_array_ref(),
                &mut doppler,
            )
        };

        if result == 0 {
            Ok(doppler)
        } else {
            Err(InvalidTime(t))
        }
    }

    /// Checks to see if the almanac data is usable at a particular time
    pub fn is_valid(&self, t: crate::time::GpsTime) -> bool {
        let result = unsafe { swiftnav_sys::almanac_valid(self.c_ptr(), t.c_ptr()) };

        result == 1
    }

    /// Checks to see if the almanac data is healthy according to the health bits
    pub fn is_healthy(&self) -> bool {
        let result = unsafe { swiftnav_sys::almanac_healthy(self.c_ptr()) };

        result == 1
    }

    /// Gets the signal ID of the almanac
    pub fn sid(&self) -> Result<GnssSignal, InvalidGnssSignal> {
        GnssSignal::from_gnss_signal_t(self.0.sid)
    }

    /// Gets the time of the almanac
    pub fn toa(&self) -> Result<GpsTime, InvalidGpsTime> {
        GpsTime::from_gps_time_t(self.0.toa)
    }

    /// User range accuracy, in meters
    pub fn ura(&self) -> f32 {
        self.0.ura
    }

    /// Curve fit interval, in seconds
    pub fn fit_interval(&self) -> u32 {
        self.0.fit_interval
    }

    /// Satellite health status:
    /// - MSB 3: NAV data health status. See IS-GPS-200H
    ///   Table 20-VII: NAV Data Health Indications;
    /// - LSB 5: Signal health status. See IS-GPS-200H
    ///   Table 20-VIII. Codes for Health of SV Signal
    ///   Components
    pub fn health_bits(&self) -> u8 {
        self.0.health_bits
    }
}
