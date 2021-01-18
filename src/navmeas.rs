use crate::{c_bindings, ephemeris::SatelliteState, signal::GnssSignal};
use std::time::Duration;

const NAV_MEAS_FLAG_CODE_VALID: u16 = 1 << 0;
const NAV_MEAS_FLAG_MEAS_DOPPLER_VALID: u16 = 1 << 2;
const NAV_MEAS_FLAG_CN0_VALID: u16 = 1 << 5;

#[derive(Clone)]
pub struct NavigationMeasurement(c_bindings::navigation_measurement_t);

impl NavigationMeasurement {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_pseudorange(&mut self, value: f64) {
        self.0.pseudorange = value;
        self.0.flags |= NAV_MEAS_FLAG_CODE_VALID;
    }

    pub fn invalidate_pseudorange(&mut self) {
        self.0.flags &= !NAV_MEAS_FLAG_CODE_VALID;
    }

    pub fn set_measured_doppler(&mut self, value: f64) {
        self.0.measured_doppler = value;
        self.0.flags |= NAV_MEAS_FLAG_MEAS_DOPPLER_VALID;
    }

    pub fn invalidate_measured_doppler(&mut self) {
        self.0.flags &= !NAV_MEAS_FLAG_MEAS_DOPPLER_VALID;
    }

    pub fn set_satellite_state(&mut self, sat_state: &SatelliteState) {
        self.0.sat_pos = *sat_state.pos.as_array_ref();
        self.0.sat_vel = *sat_state.vel.as_array_ref();
        self.0.sat_acc = *sat_state.acc.as_array_ref();
        self.0.sat_clock_err = sat_state.clock_err;
        self.0.sat_clock_err_rate = sat_state.clock_rate_err;
    }

    pub fn set_cn0(&mut self, value: f64) {
        self.0.cn0 = value;
        self.0.flags |= NAV_MEAS_FLAG_CN0_VALID;
    }

    pub fn invalidate_cn0(&mut self) {
        self.0.flags &= !NAV_MEAS_FLAG_CN0_VALID;
    }

    pub fn set_lock_time(&mut self, value: Duration) {
        self.0.lock_time = value.as_secs_f64();
    }

    pub fn set_sid(&mut self, value: GnssSignal) {
        self.0.sid = value.to_gnss_signal_t();
    }

    pub fn flags_are_valid(&self) -> bool {
        unsafe { c_bindings::nav_meas_flags_valid(self.0.flags) }
    }

    pub fn pseudorange_is_valid(&self) -> bool {
        unsafe { c_bindings::pseudorange_valid(&self.0) }
    }
}

impl Default for NavigationMeasurement {
    fn default() -> Self {
        unsafe { std::mem::zeroed::<NavigationMeasurement>() }
    }
}
