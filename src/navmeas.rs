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

use crate::{c_bindings, ephemeris::SatelliteState, signal::GnssSignal};
use std::time::Duration;

const NAV_MEAS_FLAG_CODE_VALID: u16 = 1 << 0;
const NAV_MEAS_FLAG_MEAS_DOPPLER_VALID: u16 = 1 << 2;
const NAV_MEAS_FLAG_CN0_VALID: u16 = 1 << 5;

/// Represents a single raw GNSS measurement
#[derive(Clone)]
#[repr(transparent)]
pub struct NavigationMeasurement(c_bindings::navigation_measurement_t);

impl NavigationMeasurement {
    /// Makes a navigation measurement with all fields invalidated
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the pseudorange measurement value and marks it as valid
    ///
    /// Units of meters, time of flight multiplied by speed of light
    pub fn set_pseudorange(&mut self, value: f64) {
        self.0.pseudorange = value;
        self.0.flags |= NAV_MEAS_FLAG_CODE_VALID;
    }

    /// Marks the pseudorange measurement as invalid
    pub fn invalidate_pseudorange(&mut self) {
        self.0.flags &= !NAV_MEAS_FLAG_CODE_VALID;
    }

    /// Sets the measured doppler and marks it as valid
    ///
    /// Units of Hertz
    pub fn set_measured_doppler(&mut self, value: f64) {
        self.0.measured_doppler = value;
        self.0.flags |= NAV_MEAS_FLAG_MEAS_DOPPLER_VALID;
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

    /// Sets the signal ID of the measured signal
    pub fn set_sid(&mut self, value: GnssSignal) {
        self.0.sid = value.to_gnss_signal_t();
    }

    /// Checks to see if all of the measurement flags marked as valid
    pub fn flags_are_all_valid(&self) -> bool {
        unsafe { c_bindings::nav_meas_flags_valid(self.0.flags) }
    }

    /// Checks to see if the pseudorange measurement is marked as valid
    pub fn pseudorange_is_valid(&self) -> bool {
        unsafe { c_bindings::pseudorange_valid(&self.0) }
    }
}

impl Default for NavigationMeasurement {
    fn default() -> Self {
        unsafe { std::mem::zeroed::<NavigationMeasurement>() }
    }
}

/// Encodes a [`Duration`] as an SBP lock time
///
/// Note: It is encoded according to DF402 from the RTCM 10403.2 Amendment 2
/// specification.  Valid values range from 0 to 15 and the most significant
/// nibble is reserved for future use.
pub fn encode_lock_time(nav_meas_lock_time: Duration) -> u8 {
    unsafe { c_bindings::encode_lock_time(nav_meas_lock_time.as_secs_f64()) }
}

/// Decodes an SBP lock time value into a [`Duration`]
///
/// Note: It is encoded according to DF402 from the RTCM 10403.2 Amendment 2
/// specification.  Valid values range from 0 to 15 and the most significant
/// nibble is reserved for future use.
pub fn decode_lock_time(sbp_lock_time: u8) -> Duration {
    let value = unsafe { c_bindings::decode_lock_time(sbp_lock_time) };
    Duration::from_secs_f64(value)
}
