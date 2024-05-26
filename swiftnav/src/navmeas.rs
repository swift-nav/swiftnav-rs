// Copyright (c) 2020-2021 Swift Navigation Inc.
// Contact: Swift Navigation <dev@swiftnav.com>
//
// This source is subject to the license found in the file 'LICENSE' which must
// be be distributed together with this source. All other rights reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND,
// EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A PARTICULAR PURPOSE.
//! Raw GNSS measurement representation
//!
//! Raw measurements of GNSS signals have several aspects to them, from the time
//! of flight of the signal (a.k.a. the pseudorange) to the relative velocity of
//! the satellite (a.k.a. doppler) and the signal quality (a.k.a. CN0). The
//! [`NavigationMeasurement`] struct stores all the needed components of a
//! single signal measurement. Several measurements from the same point in time
//! can be used in conjunction with [satellite ephemeris](crate::ephemeris::Ephemeris)
//! and the [PVT solver function](crate::solver::calc_pvt) to get a position,
//! velocity and time estimate.

use crate::{ephemeris::SatelliteState, signal::GnssSignal};
use std::time::Duration;

const NAV_MEAS_FLAG_CODE_VALID: u16 = 1 << 0;
const NAV_MEAS_FLAG_MEAS_DOPPLER_VALID: u16 = 1 << 2;
const NAV_MEAS_FLAG_CN0_VALID: u16 = 1 << 5;
pub const NAV_MEAS_FLAG_RAIM_EXCLUSION: u16 = 1 << 6;

/// Represents a single raw GNSS measurement
#[derive(Debug, Clone, PartialOrd, PartialEq)]
#[repr(transparent)]
pub struct NavigationMeasurement(swiftnav_sys::navigation_measurement_t);

impl NavigationMeasurement {
    /// Makes a navigation measurement with all fields invalidated
    pub fn new() -> Self {
        unsafe { std::mem::zeroed::<NavigationMeasurement>() }
    }

    /// Sets the pseudorange measurement value and marks it as valid
    ///
    /// Units of meters, time of flight multiplied by speed of light
    pub fn set_pseudorange(&mut self, value: f64) {
        self.0.raw_pseudorange = value;
        self.0.flags |= NAV_MEAS_FLAG_CODE_VALID;
    }

    /// Gets the pseudorange measurement, if a valid one has been set
    pub fn pseudorange(&self) -> Option<f64> {
        if self.0.flags & NAV_MEAS_FLAG_CODE_VALID != 0 {
            Some(self.0.raw_pseudorange)
        } else {
            None
        }
    }

    /// Marks the pseudorange measurement as invalid
    pub fn invalidate_pseudorange(&mut self) {
        self.0.flags &= !NAV_MEAS_FLAG_CODE_VALID;
    }

    /// Sets the measured doppler and marks it as valid
    ///
    /// Units of Hertz
    pub fn set_measured_doppler(&mut self, value: f64) {
        self.0.raw_measured_doppler = value;
        self.0.flags |= NAV_MEAS_FLAG_MEAS_DOPPLER_VALID;
    }

    /// Gets the measured doppler measurement, if a valid one has been set
    pub fn measured_doppler(&self) -> Option<f64> {
        if self.0.flags & NAV_MEAS_FLAG_MEAS_DOPPLER_VALID != 0 {
            Some(self.0.raw_measured_doppler)
        } else {
            None
        }
    }

    /// Marks the measured doppler measurement as invalid
    pub fn invalidate_measured_doppler(&mut self) {
        self.0.flags &= !NAV_MEAS_FLAG_MEAS_DOPPLER_VALID;
    }

    /// Sets the state of the satellite from which the signal originated
    ///
    /// The satellite state is obtained by evaluating the satellite [ephemeris](crate::ephemeris::Ephemeris::calc_satellite_state) at the time of reception of the signal
    pub fn set_satellite_state(&mut self, sat_state: &SatelliteState) {
        self.0.sat_pos = *sat_state.pos.as_array_ref();
        self.0.sat_vel = *sat_state.vel.as_array_ref();
        self.0.sat_acc = *sat_state.acc.as_array_ref();
        self.0.sat_clock_err = sat_state.clock_err;
        self.0.sat_clock_err_rate = sat_state.clock_rate_err;
    }

    /// Sets the signal CN0 measurement and marks it as valid
    ///
    /// Units of dB-Hz
    pub fn set_cn0(&mut self, value: f64) {
        self.0.cn0 = value;
        self.0.flags |= NAV_MEAS_FLAG_CN0_VALID;
    }

    /// Gets the signal CN0 measurement, if a valid one has been set
    pub fn cn0(&self) -> Option<f64> {
        if self.0.flags & NAV_MEAS_FLAG_CN0_VALID != 0 {
            Some(self.0.cn0)
        } else {
            None
        }
    }

    /// Marks the CN0 measurement as invalid
    pub fn invalidate_cn0(&mut self) {
        self.0.flags &= !NAV_MEAS_FLAG_CN0_VALID;
    }

    /// Sets the time the signal has been continuously tracked
    ///
    /// Sometimes referred to as the PLL lock time
    pub fn set_lock_time(&mut self, value: Duration) {
        self.0.lock_time = value.as_secs_f64();
    }

    pub fn lock_time(&self) -> Duration {
        Duration::from_secs_f64(self.0.lock_time)
    }

    /// Sets the signal ID of the measured signal
    pub fn set_sid(&mut self, value: GnssSignal) {
        self.0.sid = value.to_gnss_signal_t();
    }

    pub fn sid(&self) -> GnssSignal {
        GnssSignal::from_gnss_signal_t(self.0.sid).unwrap()
    }

    /// Sets the measurement flags
    pub fn set_flags(&mut self, flags: u16) {
        self.0.flags = flags;
    }

    pub fn flags(&self) -> u16 {
        self.0.flags
    }

    /// Checks to see if all of the measurement flags marked as valid
    pub fn flags_are_all_valid(&self) -> bool {
        unsafe { swiftnav_sys::nav_meas_flags_valid(self.0.flags) }
    }

    /// Checks to see if the pseudorange measurement is marked as valid
    pub fn pseudorange_is_valid(&self) -> bool {
        unsafe { swiftnav_sys::pseudorange_valid(&self.0) }
    }
}

impl Default for NavigationMeasurement {
    fn default() -> Self {
        Self::new()
    }
}

/// Encodes a [`Duration`] as an SBP lock time
///
/// Note: It is encoded according to DF402 from the RTCM 10403.2 Amendment 2
/// specification.  Valid values range from 0 to 15 and the most significant
/// nibble is reserved for future use.
pub fn encode_lock_time(nav_meas_lock_time: Duration) -> u8 {
    unsafe { swiftnav_sys::encode_lock_time(nav_meas_lock_time.as_secs_f64()) }
}

/// Decodes an SBP lock time value into a [`Duration`]
///
/// Note: It is encoded according to DF402 from the RTCM 10403.2 Amendment 2
/// specification.  Valid values range from 0 to 15 and the most significant
/// nibble is reserved for future use.
pub fn decode_lock_time(sbp_lock_time: u8) -> Duration {
    let value = unsafe { swiftnav_sys::decode_lock_time(sbp_lock_time) };
    Duration::from_secs_f64(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode() {
        let mut ret;

        ret = encode_lock_time(Duration::from_secs_f64(0.0));
        assert_eq!(ret, 0, "Incorrect return ({} vs {})", ret, 0);

        ret = encode_lock_time(Duration::from_secs_f64(0.05));
        assert_eq!(ret, 1, "Incorrect return ({} vs {})", ret, 1);

        ret = encode_lock_time(Duration::from_secs_f64(0.1));
        assert_eq!(ret, 2, "Incorrect return ({} vs {})", ret, 2);

        ret = encode_lock_time(Duration::from_secs_f64(0.2));
        assert_eq!(ret, 3, "Incorrect return ({} vs {})", ret, 3);

        ret = encode_lock_time(Duration::from_secs_f64(0.5));
        assert_eq!(ret, 4, "Incorrect return ({} vs {})", ret, 4);

        ret = encode_lock_time(Duration::from_secs_f64(1.0));
        assert_eq!(ret, 5, "Incorrect return ({} vs {})", ret, 5);

        ret = encode_lock_time(Duration::from_secs_f64(2.0));
        assert_eq!(ret, 6, "Incorrect return ({} vs {})", ret, 6);

        ret = encode_lock_time(Duration::from_secs_f64(4.0));
        assert_eq!(ret, 7, "Incorrect return ({} vs {})", ret, 7);

        ret = encode_lock_time(Duration::from_secs_f64(5.0));
        assert_eq!(ret, 8, "Incorrect return ({} vs {})", ret, 8);

        ret = encode_lock_time(Duration::from_secs_f64(10.0));
        assert_eq!(ret, 9, "Incorrect return ({} vs {})", ret, 9);

        ret = encode_lock_time(Duration::from_secs_f64(20.0));
        assert_eq!(ret, 10, "Incorrect return ({} vs {})", ret, 10);

        ret = encode_lock_time(Duration::from_secs_f64(50.0));
        assert_eq!(ret, 11, "Incorrect return ({} vs {})", ret, 11);

        ret = encode_lock_time(Duration::from_secs_f64(100.0));
        assert_eq!(ret, 12, "Incorrect return ({} vs {})", ret, 12);

        ret = encode_lock_time(Duration::from_secs_f64(200.0));
        assert_eq!(ret, 13, "Incorrect return ({} vs {})", ret, 13);

        ret = encode_lock_time(Duration::from_secs_f64(500.0));
        assert_eq!(ret, 14, "Incorrect return ({} vs {})", ret, 14);

        ret = encode_lock_time(Duration::from_secs_f64(1000.0));
        assert_eq!(ret, 15, "Incorrect return ({} vs {})", ret, 15);

        ret = encode_lock_time(Duration::new(u64::MAX, 1_000_000_000 - 1));
        assert_eq!(ret, 15, "Incorrect return ({} vs {})", ret, 15);
    }

    #[test]
    fn decode() {
        let mut ret;
        let mut exp;

        ret = decode_lock_time(0);
        exp = Duration::from_secs_f64(0.0);
        assert_eq!(ret, exp, "Incorrect return ({:?} vs {:?})", ret, exp);

        ret = decode_lock_time(0xF0);
        exp = Duration::from_secs_f64(0.0);
        assert_eq!(ret, exp, "Incorrect return ({:?} vs {:?})", ret, exp);

        ret = decode_lock_time(1);
        exp = Duration::from_secs_f64(0.032);
        assert_eq!(ret, exp, "Incorrect return ({:?} vs {:?})", ret, exp);

        ret = decode_lock_time(2);
        exp = Duration::from_secs_f64(0.064);
        assert_eq!(ret, exp, "Incorrect return ({:?} vs {:?})", ret, exp);

        ret = decode_lock_time(3);
        exp = Duration::from_secs_f64(0.128);
        assert_eq!(ret, exp, "Incorrect return ({:?} vs {:?})", ret, exp);

        ret = decode_lock_time(4);
        exp = Duration::from_secs_f64(0.256);
        assert_eq!(ret, exp, "Incorrect return ({:?} vs {:?})", ret, exp);

        ret = decode_lock_time(5);
        exp = Duration::from_secs_f64(0.512);
        assert_eq!(ret, exp, "Incorrect return ({:?} vs {:?})", ret, exp);

        ret = decode_lock_time(6);
        exp = Duration::from_secs_f64(1.024);
        assert_eq!(ret, exp, "Incorrect return ({:?} vs {:?})", ret, exp);

        ret = decode_lock_time(7);
        exp = Duration::from_secs_f64(2.048);
        assert_eq!(ret, exp, "Incorrect return ({:?} vs {:?})", ret, exp);

        ret = decode_lock_time(8);
        exp = Duration::from_secs_f64(4.096);
        assert_eq!(ret, exp, "Incorrect return ({:?} vs {:?})", ret, exp);

        ret = decode_lock_time(9);
        exp = Duration::from_secs_f64(8.192);
        assert_eq!(ret, exp, "Incorrect return ({:?} vs {:?})", ret, exp);

        ret = decode_lock_time(10);
        exp = Duration::from_secs_f64(16.384);
        assert_eq!(ret, exp, "Incorrect return ({:?} vs {:?})", ret, exp);

        ret = decode_lock_time(11);
        exp = Duration::from_secs_f64(32.768);
        assert_eq!(ret, exp, "Incorrect return ({:?} vs {:?})", ret, exp);

        ret = decode_lock_time(12);
        exp = Duration::from_secs_f64(65.536);
        assert_eq!(ret, exp, "Incorrect return ({:?} vs {:?})", ret, exp);

        ret = decode_lock_time(13);
        exp = Duration::from_secs_f64(131.072);
        assert_eq!(ret, exp, "Incorrect return ({:?} vs {:?})", ret, exp);

        ret = decode_lock_time(14);
        exp = Duration::from_secs_f64(262.144);
        assert_eq!(ret, exp, "Incorrect return ({:?} vs {:?})", ret, exp);

        ret = decode_lock_time(15);
        exp = Duration::from_secs_f64(524.288);
        assert_eq!(ret, exp, "Incorrect return ({:?} vs {:?})", ret, exp);
    }

    #[test]
    fn round_trip() {
        let value_to_encode = Duration::from_secs_f64(260.0);

        let encoded_value = encode_lock_time(value_to_encode);
        let decoded_value = decode_lock_time(encoded_value);

        assert_eq!(
            encoded_value, 13,
            "Incorrect return ({} vs {})",
            encoded_value, 13
        );

        assert_eq!(
            decoded_value,
            Duration::from_secs_f64(131.072),
            "Incorrect return ({:?} vs {:?})",
            decoded_value,
            Duration::from_secs_f64(131.072)
        );

        assert!(
            decoded_value < value_to_encode,
            "Minimum lock time not less than original lock time ({:?} < {:?})",
            decoded_value,
            value_to_encode
        );
    }
}
