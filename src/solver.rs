//! Single epoch PVT solver
//!
//! Several [raw measurements](crate::navmeas::NavigationMeasurement) from the
//! same point in time can be processed to get an estimated PVT (position,
//! velocity, and time) solution.

use crate::c_bindings;
use crate::coords::{LLHRadians, ECEF, NED};
use crate::navmeas::NavigationMeasurement;
use crate::signal::GnssSignal;
use crate::time::GpsTime;
use std::borrow::Cow;
use std::ffi;
use std::fmt;

/// A position velocity and time solution
#[derive(Clone, Debug)]
pub struct GnssSolution(c_bindings::gnss_solution);

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
            Some(LLHRadians::from_array(&self.0.pos_llh))
        } else {
            None
        }
    }

    /// Gets the received position in earth centered earth fixed cartesian coordinates
    pub fn pos_ecef(&self) -> Option<ECEF> {
        if self.pos_valid() {
            Some(ECEF::from_array(&self.0.pos_ecef))
        } else {
            None
        }
    }

    /// Gets the receiver velocity in local north east down coordinates
    pub fn vel_ned(&self) -> Option<NED> {
        if self.vel_valid() {
            Some(NED::from_array(&self.0.vel_ned))
        } else {
            None
        }
    }

    /// Gets the receiver velocity in earth centered earth fixed cartesian coordinates
    pub fn vel_ecef(&self) -> Option<ECEF> {
        if self.vel_valid() {
            Some(ECEF::from_array(&self.0.vel_ecef))
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
    ///
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
#[derive(Clone, Debug)]
pub struct Dops(c_bindings::dops_t);

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
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum ProcessingStrategy {
    GpsOnly = c_bindings::processing_strategy_t_GPS_ONLY,
    AllConstellations = c_bindings::processing_strategy_t_ALL_CONSTELLATIONS,
    GpsL1caWhenPossible = c_bindings::processing_strategy_t_GPS_L1CA_WHEN_POSSIBLE,
    L1Only = c_bindings::processing_strategy_t_L1_ONLY,
}

/// Holds the settings to customize how the GNSS solution is calculated
#[derive(Copy, Clone, Debug)]
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
pub struct SidSet(c_bindings::gnss_sid_set_t);

impl SidSet {
    /// Makes an empty set
    pub fn new() -> SidSet {
        unsafe {
            let mut ss = std::mem::zeroed::<SidSet>();
            c_bindings::sid_set_init(&mut ss.0);
            ss
        }
    }

    /// Gets the number of satellites in the set
    pub fn get_sat_count(&self) -> u32 {
        unsafe { c_bindings::sid_set_get_sat_count(&self.0) }
    }

    /// Gets the number of signals in the set
    pub fn get_sig_count(&self) -> u32 {
        unsafe { c_bindings::sid_set_get_sig_count(&self.0) }
    }

    /// Checks to see if a signal is present within the set
    pub fn contains(&self, sid: GnssSignal) -> bool {
        unsafe { c_bindings::sid_set_contains(&self.0, sid.to_gnss_signal_t()) }
    }
}

impl Default for SidSet {
    fn default() -> SidSet {
        SidSet::new()
    }
}

/// Causes of a failed PVT solution
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(usize)]
pub enum PvtError {
    /// The PDOP of the solution was unacceptably high
    HighPdop = 0,
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
            let c_char_ptr = c_bindings::pvt_err_msg[index];
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(usize)]
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
    assert!(measurements.len() <= std::u8::MAX as usize);

    let mut solution = GnssSolution::new();
    let mut dops = Dops::new();
    let mut sidset = SidSet::new();

    let result = unsafe {
        let meas_ptr =
            measurements.as_ptr() as *const [c_bindings::navigation_measurement_t; 0usize];
        c_bindings::calc_PVT(
            measurements.len() as u8,
            meas_ptr,
            tor.c_ptr(),
            settings.disable_raim,
            settings.disable_velocity,
            settings.strategy as u32,
            &mut solution.0,
            &mut dops.0,
            &mut sidset.0,
        )
    };

    if result >= 0 {
        Ok((solution, dops, sidset))
    } else {
        Err(PvtError::from_i8(result))
    }
}
