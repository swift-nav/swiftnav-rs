// Copyright (c) 2020-2021 Swift Navigation Inc.
// Contact: Swift Navigation <dev@swiftnav.com>
//
// This source is subject to the license found in the file 'LICENSE' which must
// be be distributed together with this source. All other rights reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND,
// EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A PARTICULAR PURPOSE.
//! Single epoch PVT solver
//!
//! Several [raw measurements](crate::navmeas::NavigationMeasurement) from the
//! same point in time can be processed to get an estimated PVT (position,
//! velocity, and time) solution.

use crate::coords::{LLHRadians, ECEF, NED};
use crate::navmeas::NavigationMeasurement;
use crate::signal::GnssSignal;
use crate::time::GpsTime;
use std::borrow::Cow;
use std::ffi;
use std::fmt;

/// A position velocity and time solution
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct GnssSolution(swiftnav_sys::gnss_solution);

impl GnssSolution {
    fn new() -> GnssSolution {
        unsafe { std::mem::zeroed::<GnssSolution>() }
    }

    /// Checks to see if the position solution is valid
    pub fn pos_valid(&self) -> bool {
        self.0.valid == 1
    }

    /// Checks to see if the velocity solution is valid
    pub fn vel_valid(&self) -> bool {
        self.0.velocity_valid == 1
    }

    /// Gets the received position in latitude, longitude, and height coordinates
    pub fn pos_llh(&self) -> Option<LLHRadians> {
        if self.pos_valid() {
            Some(self.0.pos_llh.into())
        } else {
            None
        }
    }

    /// Gets the received position in earth centered earth fixed cartesian coordinates
    pub fn pos_ecef(&self) -> Option<ECEF> {
        if self.pos_valid() {
            Some(self.0.pos_ecef.into())
        } else {
            None
        }
    }

    /// Gets the receiver velocity in local north east down coordinates
    pub fn vel_ned(&self) -> Option<NED> {
        if self.vel_valid() {
            Some(self.0.vel_ned.into())
        } else {
            None
        }
    }

    /// Gets the receiver velocity in earth centered earth fixed cartesian coordinates
    pub fn vel_ecef(&self) -> Option<ECEF> {
        if self.vel_valid() {
            Some(self.0.vel_ecef.into())
        } else {
            None
        }
    }

    /// Gets the receiver position covariance matrix
    ///
    /// This is the row-first upper diagonal matrix of error covariances
    /// in x, y, z (all receiver clock covariance terms are ignored).
    ///
    /// Index 6 is the GDOP.
    pub fn err_cov(&self) -> Option<&[f64; 7]> {
        if self.pos_valid() {
            Some(&self.0.err_cov)
        } else {
            None
        }
    }

    /// Gets the receiver velocity covariance matrix
    ///
    /// See [`GnssSolution::err_cov`] for representation, minus the DOP element
    pub fn vel_cov(&self) -> Option<&[f64; 7]> {
        if self.vel_valid() {
            Some(&self.0.vel_cov)
        } else {
            None
        }
    }

    /// Gets the receiver clock offset
    pub fn clock_offset(&self) -> f64 {
        self.0.clock_offset
    }

    /// Gets the receiver clock offset variance
    pub fn clock_offset_var(&self) -> f64 {
        self.0.clock_offset_var
    }

    /// Gets the receiver clock drift
    pub fn clock_drift(&self) -> f64 {
        self.0.clock_drift
    }

    /// Gets the receiver clock drift variance
    pub fn clock_drift_var(&self) -> f64 {
        self.0.clock_drift_var
    }

    /// Gets the corrected time of the measurement
    pub fn time(&self) -> GpsTime {
        GpsTime::new(self.0.time.wn, self.0.time.tow).unwrap()
    }

    /// Gets the number of satellites used in the solution
    pub fn sats_used(&self) -> u8 {
        self.0.n_sats_used
    }

    /// Gets the number of signals used in the solution
    pub fn signals_used(&self) -> u8 {
        self.0.n_sigs_used
    }
}

/// Dilution of precision (DOP) of a solution
///
/// DOP is a measurement of how the satellite geometry impacts the precision of
/// the solution
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Dops(swiftnav_sys::dops_t);

impl Dops {
    fn new() -> Dops {
        unsafe { std::mem::zeroed::<Dops>() }
    }

    /// Gets the position (3D) dilution of precision
    pub fn pdop(&self) -> f64 {
        self.0.pdop
    }

    /// Gets the geometric dilution of precision
    pub fn gdop(&self) -> f64 {
        self.0.gdop
    }

    /// Gets the time dilution of precision
    pub fn tdop(&self) -> f64 {
        self.0.tdop
    }

    /// Gets the horizontal dilution of precision
    pub fn hdop(&self) -> f64 {
        self.0.hdop
    }

    /// Gets the vertical dilution of precision
    pub fn vdop(&self) -> f64 {
        self.0.vdop
    }
}

/// Different strategies of how to choose which measurements to use in a solution
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum ProcessingStrategy {
    GpsOnly,
    AllConstellations,
    GpsL1caWhenPossible,
    L1Only,
}

impl ProcessingStrategy {
    pub(crate) fn to_processing_strategy_t(self) -> swiftnav_sys::processing_strategy_t {
        match self {
            ProcessingStrategy::GpsOnly => swiftnav_sys::processing_strategy_t_GPS_ONLY,
            ProcessingStrategy::AllConstellations => {
                swiftnav_sys::processing_strategy_t_ALL_CONSTELLATIONS
            }
            ProcessingStrategy::GpsL1caWhenPossible => {
                swiftnav_sys::processing_strategy_t_GPS_L1CA_WHEN_POSSIBLE
            }
            ProcessingStrategy::L1Only => swiftnav_sys::processing_strategy_t_L1_ONLY,
        }
    }
}

/// Holds the settings to customize how the GNSS solution is calculated
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct PvtSettings {
    strategy: ProcessingStrategy,
    disable_raim: bool,
    disable_velocity: bool,
}

impl PvtSettings {
    /// Creates a default, least common denominator, set of settings
    ///
    /// Note: The default settings consist of
    ///  * Processing all constellations and signals
    ///  * Disabling RAIM
    ///  * Disabling velocity calculation
    pub fn new() -> PvtSettings {
        PvtSettings {
            strategy: ProcessingStrategy::AllConstellations,
            disable_raim: true,
            disable_velocity: true,
        }
    }

    /// Sets the processing strategy to use
    pub fn set_strategy(self, strategy: ProcessingStrategy) -> PvtSettings {
        PvtSettings {
            strategy,
            disable_raim: self.disable_raim,
            disable_velocity: self.disable_velocity,
        }
    }

    /// Enables use of RAIM (receiver autonomous integrity monitoring)
    ///
    /// RAIM is an algorithm to detect and remove invalid measurements. Enabling
    /// RAIM means additional computations must take place to ensure the validity
    /// of the solution
    pub fn enable_raim(self) -> PvtSettings {
        PvtSettings {
            strategy: self.strategy,
            disable_raim: false,
            disable_velocity: self.disable_velocity,
        }
    }

    /// Disables use of RAIM
    ///
    /// See [`PvtSettings::enable_raim()`] for more details
    pub fn disable_raim(self) -> PvtSettings {
        PvtSettings {
            strategy: self.strategy,
            disable_raim: true,
            disable_velocity: self.disable_velocity,
        }
    }

    /// Enables calculation of a velocity solution
    ///
    /// Note: this requires the presence of doppler measurements
    pub fn enable_velocity(self) -> PvtSettings {
        PvtSettings {
            strategy: self.strategy,
            disable_raim: self.disable_raim,
            disable_velocity: false,
        }
    }

    /// Disables calculation of a velocity solution
    pub fn disable_velocity(self) -> PvtSettings {
        PvtSettings {
            strategy: self.strategy,
            disable_raim: self.disable_raim,
            disable_velocity: true,
        }
    }
}

impl Default for PvtSettings {
    fn default() -> PvtSettings {
        PvtSettings::new()
    }
}

/// Set of signals used in calculating a GNSS solution
#[derive(Clone)]
pub struct SidSet(swiftnav_sys::gnss_sid_set_t);

impl SidSet {
    /// Makes an empty set
    pub fn new() -> SidSet {
        unsafe {
            let mut ss = std::mem::zeroed::<SidSet>();
            swiftnav_sys::sid_set_init(&mut ss.0);
            ss
        }
    }

    /// Gets the number of satellites in the set
    pub fn sat_count(&self) -> u32 {
        unsafe { swiftnav_sys::sid_set_get_sat_count(&self.0) }
    }

    /// Gets the number of signals in the set
    pub fn sig_count(&self) -> u32 {
        unsafe { swiftnav_sys::sid_set_get_sig_count(&self.0) }
    }

    /// Checks to see if a signal is present within the set
    pub fn contains(&self, sid: GnssSignal) -> bool {
        unsafe { swiftnav_sys::sid_set_contains(&self.0, sid.to_gnss_signal_t()) }
    }
}

impl Default for SidSet {
    fn default() -> SidSet {
        SidSet::new()
    }
}

/// Causes of a failed PVT solution
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum PvtError {
    /// The PDOP of the solution was unacceptably high
    HighPdop,
    /// Alitutde of the solution was unacceptable
    UnreasonableAltitude,
    /// The velocity of the solution was >= 1000 kts
    HighVelocity,
    /// RAIM check and repair was unsuccessful
    RaimRepairFailed,
    /// RAIM check and repair was impossible due to not enough measurements
    RaimRepairImpossible,
    /// The least squares iteration failed to converge
    FailedToConverge,
    /// There were not enough measurements for a solution
    NotEnoughMeasurements,
}

impl PvtError {
    pub(crate) fn from_i8(val: i8) -> PvtError {
        match val {
            -1 => PvtError::HighPdop,
            -2 => PvtError::UnreasonableAltitude,
            -3 => PvtError::HighVelocity,
            -4 => PvtError::RaimRepairFailed,
            -5 => PvtError::RaimRepairImpossible,
            -6 => PvtError::FailedToConverge,
            -7 => PvtError::NotEnoughMeasurements,
            _ => panic!("Invalid PVT Error code: {}", val),
        }
    }

    pub fn as_string_lossy(&self) -> Cow<'static, str> {
        let index = *self as usize;
        unsafe {
            let c_char_ptr = swiftnav_sys::pvt_err_msg[index];
            ffi::CStr::from_ptr(c_char_ptr).to_string_lossy()
        }
    }
}

impl fmt::Display for PvtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PVT Error: {}", self.as_string_lossy())
    }
}

impl std::error::Error for PvtError {}

/// Indicates action taken while successfully calculating a solution
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum PvtStatus {
    /// Solution OK and RAIM check passed
    RaimPassed,
    /// Repaired solution, using fewer observations. The SidSet contained the removed measurements
    RepairedSolution,
    /// Solution OK, but RAIM check was not used (exactly 4 measurements given) or disabled
    RaimSkipped,
}

impl PvtStatus {
    pub(crate) fn from_i8(val: i8) -> PvtStatus {
        match val {
            0 => PvtStatus::RaimPassed,
            1 => PvtStatus::RepairedSolution,
            2 => PvtStatus::RaimSkipped,
            _ => panic!("Invalid PVT success code: {}", val),
        }
    }
}

/// Try to calculate a single point GNSS solution
pub fn calc_pvt(
    measurements: &[NavigationMeasurement],
    tor: GpsTime,
    settings: PvtSettings,
) -> Result<(PvtStatus, GnssSolution, Dops, SidSet), PvtError> {
    assert!(measurements.len() <= u8::MAX as usize);

    let mut solution = GnssSolution::new();
    let mut dops = Dops::new();
    let mut sidset = SidSet::new();

    // TODO expose this via the PvtSettings
    let obs_config = swiftnav_sys::obs_mask_config_t {
        cn0_mask: swiftnav_sys::cn0_mask_t {
            enable: false,
            threshold_dbhz: 0.0,
        },
    };

    let result = unsafe {
        let meas_ptr =
            measurements.as_ptr() as *const [swiftnav_sys::navigation_measurement_t; 0usize];
        swiftnav_sys::calc_PVT(
            measurements.len() as u8,
            meas_ptr,
            &tor.to_gps_time_t(),
            settings.disable_raim,
            settings.disable_velocity,
            &obs_config,
            settings.strategy.to_processing_strategy_t(),
            &mut solution.0,
            &mut dops.0,
            &mut sidset.0,
        )
    };

    if result >= 0 {
        Ok((PvtStatus::from_i8(result), solution, dops, sidset))
    } else {
        Err(PvtError::from_i8(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ephemeris::SatelliteState;
    use crate::signal::Code;
    use std::time::Duration;

    fn make_tor() -> GpsTime {
        GpsTime::new(1939, 42.0).unwrap()
    }

    fn make_nm1() -> NavigationMeasurement {
        let mut nm = NavigationMeasurement::new();
        nm.set_sid(GnssSignal::new(9, Code::GpsL1ca).unwrap());
        nm.set_pseudorange(23946993.888943646);
        nm.set_satellite_state(&SatelliteState {
            pos: ECEF::new(-19477278.087422125, -7649508.9457812719, 16674633.163554827),
            vel: ECEF::new(0.0, 0.0, 0.0),
            acc: ECEF::new(0.0, 0.0, 0.0),
            clock_err: 0.0,
            clock_rate_err: 0.0,
            iodc: 0,
            iode: 0,
        });
        nm.set_lock_time(Duration::from_secs_f64(5.0));
        nm.set_measured_doppler(0.);
        nm
    }

    fn make_nm1_no_doppler() -> NavigationMeasurement {
        let mut nm = NavigationMeasurement::new();
        nm.set_sid(GnssSignal::new(9, Code::GpsL1ca).unwrap());
        nm.set_pseudorange(23946993.888943646);
        nm.set_satellite_state(&SatelliteState {
            pos: ECEF::new(-19477278.087422125, -7649508.9457812719, 16674633.163554827),
            vel: ECEF::new(0.0, 0.0, 0.0),
            acc: ECEF::new(0.0, 0.0, 0.0),
            clock_err: 0.0,
            clock_rate_err: 0.0,
            iodc: 0,
            iode: 0,
        });
        nm.set_lock_time(Duration::from_secs_f64(5.0));
        nm
    }

    fn make_nm2() -> NavigationMeasurement {
        let mut nm = NavigationMeasurement::new();
        nm.set_sid(GnssSignal::new(1, Code::GpsL1ca).unwrap());
        nm.set_pseudorange(22932174.156858064);
        nm.set_satellite_state(&SatelliteState {
            pos: ECEF::new(-9680013.5408340245, -15286326.354385279, 19429449.383770257),
            vel: ECEF::new(0.0, 0.0, 0.0),
            acc: ECEF::new(0.0, 0.0, 0.0),
            clock_err: 0.0,
            clock_rate_err: 0.0,
            iodc: 0,
            iode: 0,
        });
        nm.set_lock_time(Duration::from_secs_f64(5.0));
        nm.set_measured_doppler(0.);
        nm
    }

    fn make_nm3() -> NavigationMeasurement {
        let mut nm = NavigationMeasurement::new();
        nm.set_sid(GnssSignal::new(2, Code::GpsL1ca).unwrap());
        nm.set_pseudorange(24373231.648055989);
        nm.set_satellite_state(&SatelliteState {
            pos: ECEF::new(-19858593.085281931, -3109845.8288993631, 17180320.439503901),
            vel: ECEF::new(0.0, 0.0, 0.0),
            acc: ECEF::new(0.0, 0.0, 0.0),
            clock_err: 0.0,
            clock_rate_err: 0.0,
            iodc: 0,
            iode: 0,
        });
        nm.set_lock_time(Duration::from_secs_f64(5.0));
        nm.set_measured_doppler(0.);
        nm
    }

    fn make_nm4() -> NavigationMeasurement {
        let mut nm = NavigationMeasurement::new();
        nm.set_sid(GnssSignal::new(3, Code::GpsL1ca).unwrap());
        nm.set_pseudorange(24779663.252316438);
        nm.set_satellite_state(&SatelliteState {
            pos: ECEF::new(6682497.8716542246, -14006962.389166718, 21410456.27567846),
            vel: ECEF::new(0.0, 0.0, 0.0),
            acc: ECEF::new(0.0, 0.0, 0.0),
            clock_err: 0.0,
            clock_rate_err: 0.0,
            iodc: 0,
            iode: 0,
        });
        nm.set_lock_time(Duration::from_secs_f64(5.0));
        nm.set_measured_doppler(0.);
        nm
    }

    fn make_nm5() -> NavigationMeasurement {
        let mut nm = NavigationMeasurement::new();
        nm.set_sid(GnssSignal::new(4, Code::GpsL1ca).unwrap());
        nm.set_pseudorange(26948717.022331879);
        nm.set_satellite_state(&SatelliteState {
            pos: ECEF::new(7415370.9916331079, -24974079.044485383, -3836019.0262199985),
            vel: ECEF::new(0.0, 0.0, 0.0),
            acc: ECEF::new(0.0, 0.0, 0.0),
            clock_err: 0.0,
            clock_rate_err: 0.0,
            iodc: 0,
            iode: 0,
        });
        nm.set_lock_time(Duration::from_secs_f64(5.0));
        nm.set_measured_doppler(0.);
        nm
    }

    fn make_nm6() -> NavigationMeasurement {
        let mut nm = NavigationMeasurement::new();
        nm.set_sid(GnssSignal::new(5, Code::GpsL1ca).unwrap());
        nm.set_pseudorange(23327405.435463827);
        nm.set_satellite_state(&SatelliteState {
            pos: ECEF::new(-2833466.1648670658, -22755197.793894723, 13160322.082875408),
            vel: ECEF::new(0.0, 0.0, 0.0),
            acc: ECEF::new(0.0, 0.0, 0.0),
            clock_err: 0.0,
            clock_rate_err: 0.0,
            iodc: 0,
            iode: 0,
        });
        nm.set_lock_time(Duration::from_secs_f64(5.0));
        nm.set_measured_doppler(0.);
        nm
    }

    fn make_nm6b() -> NavigationMeasurement {
        let mut nm = NavigationMeasurement::new();
        nm.set_sid(GnssSignal::new(5, Code::GpsL1ca).unwrap());
        nm.set_pseudorange(23327405.435463827);
        nm.set_satellite_state(&SatelliteState {
            pos: ECEF::new(-2833466.1648670658, -22755197.793894723, 13160322.082875408),
            vel: ECEF::new(0.0, 0.0, 0.0),
            acc: ECEF::new(0.0, 0.0, 0.0),
            clock_err: 0.0,
            clock_rate_err: 0.0,
            iodc: 0,
            iode: 0,
        });
        nm.set_lock_time(Duration::from_secs_f64(5.0));
        nm.set_cn0(40.);
        nm.set_measured_doppler(10000.); // Doppler outlier
        nm
    }

    fn make_nm7() -> NavigationMeasurement {
        let mut nm = NavigationMeasurement::new();
        nm.set_sid(GnssSignal::new(6, Code::GpsL1ca).unwrap());
        nm.set_pseudorange(27371419.016328193);
        nm.set_satellite_state(&SatelliteState {
            pos: ECEF::new(14881660.383624561, -5825253.4316490609, 21204679.68313824),
            vel: ECEF::new(0.0, 0.0, 0.0),
            acc: ECEF::new(0.0, 0.0, 0.0),
            clock_err: 0.0,
            clock_rate_err: 0.0,
            iodc: 0,
            iode: 0,
        });
        nm.set_lock_time(Duration::from_secs_f64(5.0));
        nm.set_measured_doppler(0.);
        nm
    }

    fn make_nm8() -> NavigationMeasurement {
        let mut nm = NavigationMeasurement::new();
        nm.set_sid(GnssSignal::new(7, Code::GpsL1ca).unwrap());
        nm.set_pseudorange(26294221.697782904);
        nm.set_satellite_state(&SatelliteState {
            pos: ECEF::new(12246530.477279386, -22184711.955107089, 7739084.285506918),
            vel: ECEF::new(0.0, 0.0, 0.0),
            acc: ECEF::new(0.0, 0.0, 0.0),
            clock_err: 0.0,
            clock_rate_err: 0.0,
            iodc: 0,
            iode: 0,
        });
        nm.set_lock_time(Duration::from_secs_f64(5.0));
        nm.set_measured_doppler(0.);
        nm
    }

    fn make_nm9() -> NavigationMeasurement {
        let mut nm = NavigationMeasurement::new();
        nm.set_sid(GnssSignal::new(8, Code::GpsL1ca).unwrap());
        nm.set_pseudorange(25781999.479948733);
        nm.set_satellite_state(&SatelliteState {
            pos: ECEF::new(-25360766.249484103, -1659033.490658124, 7821492.0398916304),
            vel: ECEF::new(0.0, 0.0, 0.0),
            acc: ECEF::new(0.0, 0.0, 0.0),
            clock_err: 0.0,
            clock_rate_err: 0.0,
            iodc: 0,
            iode: 0,
        });
        nm.set_lock_time(Duration::from_secs_f64(5.0));
        nm.set_measured_doppler(0.);
        nm
    }

    fn make_nm10() -> NavigationMeasurement {
        let mut nm = NavigationMeasurement::new();
        nm.set_sid(GnssSignal::new(8, Code::GpsL2cm).unwrap());
        nm.set_pseudorange(25781999.479948733);
        nm.set_satellite_state(&SatelliteState {
            pos: ECEF::new(-25360766.249484103, -1659033.490658124, 7821492.0398916304),
            vel: ECEF::new(0.0, 0.0, 0.0),
            acc: ECEF::new(0.0, 0.0, 0.0),
            clock_err: 0.0,
            clock_rate_err: 0.0,
            iodc: 0,
            iode: 0,
        });
        nm.set_lock_time(Duration::from_secs_f64(5.0));
        nm.set_measured_doppler(0.);
        nm
    }

    fn make_nm10b() -> NavigationMeasurement {
        let mut nm = NavigationMeasurement::new();
        nm.set_sid(GnssSignal::new(8, Code::GpsL2cm).unwrap());
        nm.set_pseudorange(25781999.479948733 + 30000.);
        nm.set_satellite_state(&SatelliteState {
            pos: ECEF::new(25360766.249484103, -1659033.490658124, 7821492.0398916304),
            vel: ECEF::new(0.0, 0.0, 0.0),
            acc: ECEF::new(0.0, 0.0, 0.0),
            clock_err: 0.0,
            clock_rate_err: 0.0,
            iodc: 0,
            iode: 0,
        });
        nm.set_lock_time(Duration::from_secs_f64(5.0));
        nm.set_measured_doppler(0.);
        nm
    }

    fn make_nm11() -> NavigationMeasurement {
        let mut nm = NavigationMeasurement::new();
        nm.set_sid(GnssSignal::new(11, Code::GpsL2cm).unwrap());
        nm.set_pseudorange(25781999.479948733);
        nm.set_satellite_state(&SatelliteState {
            pos: ECEF::new(-25360766.249484103, -1659033.490658124, 7821492.0398916304),
            vel: ECEF::new(0.0, 0.0, 0.0),
            acc: ECEF::new(0.0, 0.0, 0.0),
            clock_err: 0.0,
            clock_rate_err: 0.0,
            iodc: 0,
            iode: 0,
        });
        nm.set_lock_time(Duration::from_secs_f64(5.0));
        nm.set_measured_doppler(0.);
        nm
    }

    // Note this is a copy of GPS nm1 but set to code GAL_E1B, do not combine
    // them in the same test case
    fn make_gal_nm1() -> NavigationMeasurement {
        let mut nm = NavigationMeasurement::new();
        nm.set_sid(GnssSignal::new(9, Code::GalE1b).unwrap());
        nm.set_pseudorange(23946993.888943646);
        nm.set_satellite_state(&SatelliteState {
            pos: ECEF::new(-19477278.087422125, -7649508.9457812719, 16674633.163554827),
            vel: ECEF::new(0.0, 0.0, 0.0),
            acc: ECEF::new(0.0, 0.0, 0.0),
            clock_err: 0.0,
            clock_rate_err: 0.0,
            iodc: 0,
            iode: 0,
        });
        nm.set_lock_time(Duration::from_secs_f64(5.0));
        nm.set_measured_doppler(0.);
        nm
    }

    // Note this is a copy of GPS nm2 but set to code GAL_E1B, do not combine
    // them in the same test case
    fn make_gal_nm2() -> NavigationMeasurement {
        let mut nm = NavigationMeasurement::new();
        nm.set_sid(GnssSignal::new(1, Code::GalE1b).unwrap());
        nm.set_pseudorange(22932174.156858064);
        nm.set_satellite_state(&SatelliteState {
            pos: ECEF::new(-9680013.5408340245, -15286326.354385279, 19429449.383770257),
            vel: ECEF::new(0.0, 0.0, 0.0),
            acc: ECEF::new(0.0, 0.0, 0.0),
            clock_err: 0.0,
            clock_rate_err: 0.0,
            iodc: 0,
            iode: 0,
        });
        nm.set_lock_time(Duration::from_secs_f64(5.0));
        nm.set_measured_doppler(0.);
        nm
    }

    #[test]
    fn pvt_failed_repair() {
        let nms = [make_nm1(), make_nm2(), make_nm3(), make_nm4(), make_nm5()];
        let settings = PvtSettings {
            strategy: ProcessingStrategy::AllConstellations,
            disable_raim: false,
            disable_velocity: true,
        };

        let result = calc_pvt(&nms, make_tor(), settings);

        assert!(result.is_err(), "PVT should fail");
        let err = result.err().unwrap();
        /* PVT repair requires at least 6 measurements. */
        assert_eq!(err, PvtError::RaimRepairFailed);
    }

    #[test]
    fn pvt_repair() {
        let expected_removed_sid = GnssSignal::new(9, Code::GpsL1ca).unwrap();

        let nms = [
            make_nm1(),
            make_nm2(),
            make_nm3(),
            make_nm4(),
            make_nm5(),
            make_nm6(),
        ];
        let settings = PvtSettings {
            strategy: ProcessingStrategy::AllConstellations,
            disable_raim: false,
            disable_velocity: true,
        };

        let result = calc_pvt(&nms, make_tor(), settings);

        assert!(result.is_ok());
        let (status, soln, _, raim_emoved_sids) = result.unwrap();
        assert_eq!(
            status,
            PvtStatus::RepairedSolution,
            "Return code should be pvt repaired. Saw: {:?}",
            status
        );
        assert_eq!(
            soln.signals_used(),
            (nms.len() - 1) as u8,
            "n_sigs_used should be {}. Saw: {}",
            nms.len() - 1,
            soln.signals_used()
        );
        assert_eq!(
            soln.sats_used(),
            (nms.len() - 1) as u8,
            "n_sats_used should be {}. Saw: {}",
            nms.len() - 1,
            soln.sats_used()
        );
        assert!(
            raim_emoved_sids.contains(expected_removed_sid),
            "Unexpected RAIM removed SID!"
        );
    }

    #[test]
    fn pvt_raim_singular() {
        /* test the case of bug 946 where extreme pseudorange errors lead to singular
         * geometry */

        let mut nm1_broken = make_nm1();
        nm1_broken.set_pseudorange(nm1_broken.pseudorange().unwrap() + 5e8);
        let mut nm2_broken = make_nm2();
        nm2_broken.set_pseudorange(nm2_broken.pseudorange().unwrap() - 2e7);

        let nms = [
            nm1_broken,
            nm2_broken,
            make_nm3(),
            make_nm4(),
            make_nm5(),
            make_nm6(),
            make_nm7(),
            make_nm9(),
            make_nm10(),
        ];
        let settings = PvtSettings {
            strategy: ProcessingStrategy::AllConstellations,
            disable_raim: false,
            disable_velocity: true,
        };

        let result = calc_pvt(&nms, make_tor(), settings);

        assert!(result.is_err(), "PVT should fail");
        let err = result.err().unwrap();
        assert_eq!(
            err,
            PvtError::RaimRepairFailed,
            "Return code should be RAIM failed. Saw: {:?}",
            err
        );
    }

    #[test]
    fn pvt_vel_repair() {
        let expected_removed_sid = GnssSignal::new(5, Code::GpsL1ca).unwrap();

        let nms = [
            make_nm2(),
            make_nm3(),
            make_nm4(),
            make_nm5(),
            make_nm6b(),
            make_nm7(),
        ];
        let settings = PvtSettings {
            strategy: ProcessingStrategy::AllConstellations,
            disable_raim: false,
            disable_velocity: false,
        };

        let result = calc_pvt(&nms, make_tor(), settings);

        assert!(result.is_ok(), "PVT should succeed");
        let (pvt_status, soln, _, sid_set) = result.unwrap();
        assert_eq!(
            pvt_status,
            PvtStatus::RepairedSolution,
            "Return code should be pvt repaired. Saw: {:?}",
            pvt_status
        );
        assert_eq!(
            soln.signals_used(),
            (nms.len() - 1) as u8,
            "n_sigs_used should be {}. Saw: {}",
            nms.len() - 1,
            soln.signals_used()
        );
        assert_eq!(
            soln.sats_used(),
            (nms.len() - 1) as u8,
            "n_sats_used should be {}. Saw: {}",
            nms.len() - 1,
            soln.sats_used()
        );
        assert!(
            sid_set.contains(expected_removed_sid),
            "Unexpected RAIM removed SID!"
        );
    }

    #[test]
    fn pvt_repair_multifailure() {
        let expected_removed_sid = GnssSignal::new(9, Code::GpsL1ca).unwrap();

        let nms = [
            make_nm1(),
            make_nm2(),
            make_nm3(),
            make_nm7(),
            make_nm10b(),
            make_nm5(),
            make_nm6(),
        ];
        let settings = PvtSettings {
            strategy: ProcessingStrategy::AllConstellations,
            disable_raim: false,
            disable_velocity: false,
        };

        let result = calc_pvt(&nms, make_tor(), settings);

        assert!(result.is_ok(), "PVT should succeed");
        let (pvt_status, soln, _, sid_set) = result.unwrap();
        assert_eq!(
            pvt_status,
            PvtStatus::RepairedSolution,
            "Return code should be pvt repaired. Saw: {:?}",
            pvt_status
        );
        assert_eq!(
            soln.signals_used(),
            (nms.len() - 2) as u8,
            "n_sigs_used should be {}. Saw: {}",
            nms.len() - 2,
            soln.signals_used()
        );
        assert_eq!(
            soln.sats_used(),
            (nms.len() - 2) as u8,
            "n_sats_used should be {}. Saw: {}",
            nms.len() - 2,
            soln.sats_used()
        );
        assert!(
            sid_set.contains(expected_removed_sid),
            "Unexpected RAIM removed SID!"
        );
    }

    #[test]
    fn pvt_raim_gps_l1ca_only() {
        /* 9 L1CA signals (one broken) and 1 L2CM signal */
        let expected_removed_sid = GnssSignal::new(9, Code::GpsL1ca).unwrap();

        let nms = [
            make_nm1(),
            make_nm2(),
            make_nm3(),
            make_nm4(),
            make_nm5(),
            make_nm6(),
            make_nm7(),
            make_nm8(),
            make_nm9(),
            make_nm10(),
        ];
        let settings = PvtSettings {
            strategy: ProcessingStrategy::GpsL1caWhenPossible,
            disable_raim: false,
            disable_velocity: false,
        };

        let result = calc_pvt(&nms, make_tor(), settings);

        assert!(result.is_ok(), "PVT should succeed");
        let (pvt_status, soln, _, sid_set) = result.unwrap();
        assert_eq!(
            pvt_status,
            PvtStatus::RepairedSolution,
            "Return code should be pvt repaired. Saw: {:?}",
            pvt_status
        );
        assert_eq!(
            soln.signals_used(),
            (nms.len() - 2) as u8,
            "n_sigs_used should be {}. Saw: {}",
            nms.len() - 2,
            soln.signals_used()
        );
        assert_eq!(
            soln.sats_used(),
            (nms.len() - 2) as u8,
            "n_sats_used should be {}. Saw: {}",
            nms.len() - 2,
            soln.sats_used()
        );
        assert!(
            sid_set.contains(expected_removed_sid),
            "Unexpected RAIM removed SID!"
        );
    }

    #[test]
    fn pvt_outlier_gps_l1ca_only() {
        let nms = [
            make_nm2(),
            make_nm3(),
            make_nm4(),
            make_nm5(),
            make_nm6(),
            make_nm7(),
            make_nm8(),
            make_nm9(),
            make_nm10b(),
        ];
        let settings = PvtSettings {
            strategy: ProcessingStrategy::GpsL1caWhenPossible,
            disable_raim: false,
            disable_velocity: false,
        };

        let result = calc_pvt(&nms, make_tor(), settings);

        assert!(result.is_ok(), "PVT should succeed");
        let (pvt_status, soln, _, _sid_set) = result.unwrap();
        assert_eq!(
            pvt_status,
            PvtStatus::RaimPassed,
            "Return code should be pvt repaired. Saw: {:?}",
            pvt_status
        );
        assert_eq!(
            soln.signals_used(),
            (nms.len() - 1) as u8,
            "n_sigs_used should be {}. Saw: {}",
            nms.len() - 1,
            soln.signals_used()
        );
        assert_eq!(
            soln.sats_used(),
            (nms.len() - 1) as u8,
            "n_sats_used should be {}. Saw: {}",
            nms.len() - 1,
            soln.sats_used()
        );
    }

    #[test]
    fn pvt_flag_outlier_bias() {
        /* 8 L1CA signals and 2 L2CM signals */

        /* add a common bias of 120 m to the L2CM measurements */
        let mut nm10_bias = make_nm10();
        nm10_bias.set_pseudorange(nm10_bias.pseudorange().unwrap() + 120.);
        let mut nm11_bias = make_nm11();
        nm11_bias.set_pseudorange(nm11_bias.pseudorange().unwrap() + 120.);

        /* healthy measurements, with bias on L2 */
        let nms = [
            make_nm2(),
            make_nm3(),
            make_nm4(),
            make_nm5(),
            make_nm6(),
            make_nm7(),
            make_nm8(),
            nm10_bias.clone(),
            nm11_bias.clone(),
        ];
        let settings = PvtSettings {
            strategy: ProcessingStrategy::GpsL1caWhenPossible,
            disable_raim: false,
            disable_velocity: false,
        };

        let result = calc_pvt(&nms, make_tor(), settings);

        assert!(result.is_ok(), "PVT should succeed");
        let (pvt_status, soln, _, _) = result.unwrap();
        assert_eq!(
            pvt_status,
            PvtStatus::RaimPassed,
            "Return code should be raim passed. Saw: {:?}",
            pvt_status
        );
        assert_eq!(
            soln.signals_used(),
            (nms.len() - 2) as u8,
            "n_sigs_used should be {}. Saw: {}",
            nms.len() - 2,
            soln.signals_used()
        );
        assert_eq!(
            soln.sats_used(),
            (nms.len() - 2) as u8,
            "n_sats_used should be {}. Saw: {}",
            nms.len() - 2,
            soln.sats_used()
        );

        /* add outlier to one of the L2 measurements  */
        nm11_bias.set_pseudorange(nm11_bias.pseudorange().unwrap() + 1000.);
        let nms = [
            make_nm2(),
            make_nm3(),
            make_nm4(),
            make_nm5(),
            make_nm6(),
            make_nm7(),
            make_nm8(),
            nm10_bias,
            nm11_bias,
        ];

        let result = calc_pvt(&nms, make_tor(), settings);

        assert!(result.is_ok(), "PVT should succeed");
        let (pvt_status, soln, _, _sid_set) = result.unwrap();
        assert_eq!(
            pvt_status,
            PvtStatus::RaimPassed,
            "Return code should be repaired solution. Saw: {:?}",
            pvt_status
        );
        assert_eq!(
            soln.signals_used(),
            (nms.len() - 2) as u8,
            "n_sigs_used should be {}. Saw: {}",
            nms.len() - 2,
            soln.signals_used()
        );
        assert_eq!(
            soln.sats_used(),
            (nms.len() - 2) as u8,
            "n_sats_used should be {}. Saw: {}",
            nms.len() - 2,
            soln.sats_used()
        );
    }

    #[test]
    fn disable_pvt_raim() {
        let nms = [
            make_nm1(),
            make_nm2(),
            make_nm3(),
            make_nm4(),
            make_nm5(),
            make_nm6(),
        ];
        /* disable raim check */
        let settings = PvtSettings {
            strategy: ProcessingStrategy::AllConstellations,
            disable_raim: true,
            disable_velocity: true,
        };

        let result = calc_pvt(&nms, make_tor(), settings);

        assert!(result.is_ok(), "PVT should succeed");
        let (pvt_status, soln, _, _) = result.unwrap();
        assert_eq!(
            pvt_status,
            PvtStatus::RaimSkipped,
            "Return code should be raim not used. Saw: {:?}",
            pvt_status
        );
        assert!(soln.pos_valid(), "Solution should be valid!");
    }

    #[test]
    fn disable_pvt_velocity() {
        let nms = [
            make_nm1_no_doppler(),
            make_nm2(),
            make_nm3(),
            make_nm4(),
            make_nm5(),
            make_nm6(),
        ];
        let settings = PvtSettings {
            strategy: ProcessingStrategy::AllConstellations,
            disable_raim: false,
            disable_velocity: true,
        };

        let result = calc_pvt(&nms, make_tor(), settings);

        assert!(result.is_ok(), "PVT should succeed");
        let (_, soln, _, _) = result.unwrap();
        assert!(soln.pos_valid(), "Solution should be valid!");
        assert!(!soln.vel_valid(), "Velocity should not be valid!");
        assert!(soln.vel_ned().is_none(), "Velocity should not be valid!");
        assert!(soln.vel_ecef().is_none(), "Velocity should not be valid!");
    }

    #[test]
    fn count_sats() {
        let nms = [
            make_nm1(),
            make_nm2(),
            make_nm3(),
            make_nm4(),
            make_nm5(),
            make_nm6(),
            make_nm7(),
            make_nm8(),
            make_nm9(),
            make_nm10(),
        ];
        let settings = PvtSettings {
            strategy: ProcessingStrategy::AllConstellations,
            disable_raim: true,
            disable_velocity: false,
        };

        let result = calc_pvt(&nms, make_tor(), settings);

        assert!(result.is_ok(), "PVT should succeed");
        let (_, soln, _, _) = result.unwrap();
        assert!(soln.pos_valid(), "Solution should be valid!");
        assert_eq!(
            soln.signals_used(),
            10,
            "n_sigs_used should be 10. Saw: {}",
            soln.signals_used()
        );
        assert_eq!(
            soln.sats_used(),
            9,
            "n_sats_used should be 9. Saw: {}",
            soln.sats_used()
        );
    }

    #[test]
    fn count_sats_l1ca_only() {
        let nms = [
            make_nm1(),
            make_nm2(),
            make_nm3(),
            make_nm4(),
            make_nm5(),
            make_nm6(),
            make_nm7(),
            make_nm8(),
            make_nm9(),
            make_nm10(),
        ];
        let settings = PvtSettings {
            strategy: ProcessingStrategy::GpsL1caWhenPossible,
            disable_raim: true,
            disable_velocity: false,
        };

        let result = calc_pvt(&nms, make_tor(), settings);

        assert!(result.is_ok(), "PVT should succeed");
        let (_, soln, _, _) = result.unwrap();
        assert!(soln.pos_valid(), "Solution should be valid!");
        assert_eq!(
            soln.signals_used(),
            9,
            "n_sigs_used should be 9. Saw: {}",
            soln.signals_used()
        );
        assert_eq!(
            soln.sats_used(),
            9,
            "n_sats_used should be 9. Saw: {}",
            soln.sats_used()
        );
    }

    #[test]
    fn dops() {
        let truedops = Dops(swiftnav_sys::dops_t {
            pdop: 2.69955,
            gdop: 3.07696,
            tdop: 1.47652,
            hdop: 1.76157,
            vdop: 2.04559,
        });

        let dop_tol = 1e-3;

        let nms = [
            make_nm1(),
            make_nm2(),
            make_nm3(),
            make_nm4(),
            make_nm5(),
            make_nm6(),
        ];
        let settings = PvtSettings {
            strategy: ProcessingStrategy::AllConstellations,
            disable_raim: false,
            disable_velocity: true,
        };

        let result = calc_pvt(&nms, make_tor(), settings);

        assert!(result.is_ok(), "PVT should succeed");
        let (_, soln, dops, _) = result.unwrap();
        assert!(soln.pos_valid(), "Solution should be valid!");
        assert!(
            (dops.pdop() * dops.pdop() - (dops.vdop() * dops.vdop() + dops.hdop() * dops.hdop()))
                .abs()
                < dop_tol,
            "HDOP^2 + VDOP^2 != PDOP^2.  Saw: {}, {}, {}, {}, {}",
            dops.pdop(),
            dops.gdop(),
            dops.tdop(),
            dops.hdop(),
            dops.vdop()
        );
        let dop_err = (dops.pdop() - truedops.pdop()).abs()
            + (dops.gdop() - truedops.gdop()).abs()
            + (dops.tdop() - truedops.tdop()).abs()
            + (dops.hdop() - truedops.hdop()).abs()
            + (dops.vdop() - truedops.vdop()).abs();
        assert!(
            dop_err < dop_tol,
            "DOPs don't match hardcoded correct values. Saw: {}, {}, {}, {}, {}",
            dops.pdop(),
            dops.gdop(),
            dops.tdop(),
            dops.hdop(),
            dops.vdop()
        );
    }

    #[test]
    fn test_calc_pvt_exclude_gal() {
        // u8 n_used = 8;
        // u8 n_gps_l1ca = 6;
        // gnss_solution soln;
        // dops_t dops;
        // gnss_sid_set_t raim_removed_sids;

        let nms = [
            make_nm3(),
            make_gal_nm1(),
            make_gal_nm2(),
            make_nm5(),
            make_nm6(),
            make_nm7(),
            make_nm8(),
            make_nm9(),
        ];
        let settings = PvtSettings {
            strategy: ProcessingStrategy::GpsOnly,
            disable_raim: false,
            disable_velocity: false,
        };

        let result = calc_pvt(&nms, make_tor(), settings);

        assert!(result.is_ok(), "PVT should succeed");
        let (_, soln, _, _) = result.unwrap();
        assert_eq!(
            soln.sats_used(),
            6,
            "Only 6 sats should be used when performing GPS only"
        );
        assert_eq!(
            soln.signals_used(),
            6,
            "Only 6 signals should be used when performing GPS only"
        );
    }
}
