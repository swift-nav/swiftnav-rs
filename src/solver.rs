use crate::c_bindings;
use crate::coords::{LLHRadians, ECEF, NED};
use crate::navmeas::NavigationMeasurement;
use crate::signal::GnssSignal;
use crate::time::GpsTime;
use std::borrow::Cow;
use std::ffi;
use std::fmt;

#[derive(Clone, Debug)]
pub struct GnssSolution(c_bindings::gnss_solution);

impl GnssSolution {
    pub fn pos_valid(&self) -> bool {
        self.0.valid == 1
    }

    pub fn vel_valid(&self) -> bool {
        self.0.velocity_valid == 1
    }

    pub fn pos_llh(&self) -> LLHRadians {
        LLHRadians::from_array(&self.0.pos_llh)
    }

    pub fn pos_ecef(&self) -> ECEF {
        ECEF::from_array(&self.0.pos_ecef)
    }

    pub fn vel_ned(&self) -> NED {
        NED::from_array(&self.0.vel_ned)
    }

    pub fn vel_ecef(&self) -> ECEF {
        ECEF::from_array(&self.0.vel_ecef)
    }

    pub fn err_cov(&self) -> &[f64; 7] {
        &self.0.err_cov
    }

    pub fn vel_cov(&self) -> &[f64; 7] {
        &self.0.vel_cov
    }

    pub fn clock_offset(&self) -> f64 {
        self.0.clock_offset
    }

    pub fn clock_offset_var(&self) -> f64 {
        self.0.clock_offset_var
    }

    pub fn clock_drift(&self) -> f64 {
        self.0.clock_drift
    }

    pub fn clock_drift_var(&self) -> f64 {
        self.0.clock_drift_var
    }

    pub fn time(&self) -> GpsTime {
        GpsTime::new_unchecked(self.0.time.wn, self.0.time.tow)
    }

    pub fn sats_used(&self) -> u8 {
        self.0.n_sats_used
    }

    pub fn signals_used(&self) -> u8 {
        self.0.n_sigs_used
    }
}

impl Default for GnssSolution {
    fn default() -> GnssSolution {
        unsafe { std::mem::zeroed::<GnssSolution>() }
    }
}

#[derive(Clone, Debug)]
pub struct Dops(c_bindings::dops_t);

impl Dops {
    pub fn pdop(&self) -> f64 {
        self.0.pdop
    }
    pub fn gdop(&self) -> f64 {
        self.0.gdop
    }
    pub fn tdop(&self) -> f64 {
        self.0.tdop
    }
    pub fn hdop(&self) -> f64 {
        self.0.hdop
    }
    pub fn vdop(&self) -> f64 {
        self.0.vdop
    }
}

impl Default for Dops {
    fn default() -> Dops {
        unsafe { std::mem::zeroed::<Dops>() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum ProcessingStrategy {
    GpsOnly = c_bindings::processing_strategy_t_GPS_ONLY,
    AllConstellations = c_bindings::processing_strategy_t_ALL_CONSTELLATIONS,
    GpsL1caWhenPossible = c_bindings::processing_strategy_t_GPS_L1CA_WHEN_POSSIBLE,
    L1Only = c_bindings::processing_strategy_t_L1_ONLY,
}

#[derive(Copy, Clone, Debug)]
pub struct PvtSettings {
    strategy: ProcessingStrategy,
    disable_raim: bool,
    disable_velocity: bool,
}

impl PvtSettings {
    pub fn set_strategy(self, strategy: ProcessingStrategy) -> PvtSettings {
        PvtSettings {
            strategy,
            disable_raim: self.disable_raim,
            disable_velocity: self.disable_velocity,
        }
    }

    pub fn enable_raim(self) -> PvtSettings {
        PvtSettings {
            strategy: self.strategy,
            disable_raim: false,
            disable_velocity: self.disable_velocity,
        }
    }

    pub fn disable_raim(self) -> PvtSettings {
        PvtSettings {
            strategy: self.strategy,
            disable_raim: true,
            disable_velocity: self.disable_velocity,
        }
    }

    pub fn enable_velocity(self) -> PvtSettings {
        PvtSettings {
            strategy: self.strategy,
            disable_raim: self.disable_raim,
            disable_velocity: false,
        }
    }

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
        PvtSettings {
            strategy: ProcessingStrategy::AllConstellations,
            disable_raim: false,
            disable_velocity: false,
        }
    }
}

#[derive(Clone)]
pub struct SidSet(c_bindings::gnss_sid_set_t);

impl SidSet {
    pub fn new() -> SidSet {
        unsafe {
            let mut ss = std::mem::zeroed::<SidSet>();
            c_bindings::sid_set_init(&mut ss.0);
            ss
        }
    }

    pub fn get_sat_count(&self) -> u32 {
        unsafe { c_bindings::sid_set_get_sat_count(&self.0) }
    }

    pub fn get_sig_count(&self) -> u32 {
        unsafe { c_bindings::sid_set_get_sig_count(&self.0) }
    }

    pub fn contains(&self, sid: GnssSignal) -> bool {
        unsafe { c_bindings::sid_set_contains(&self.0, sid.to_gnss_signal_t()) }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(usize)]
pub enum PvtError {
    HighPdop = 0,
    UnreasonableAltitude,
    HighVelocity,
    RaimRepairFailed,
    RaimRepairImpossible,
    FailedToConverge,
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

pub fn calc_pvt(
    measurements: &[NavigationMeasurement],
    tor: GpsTime,
    settings: PvtSettings,
) -> Result<(GnssSolution, Dops, SidSet), PvtError> {
    assert!(measurements.len() <= std::u8::MAX as usize);

    let mut solution = GnssSolution::default();
    let mut dops = Dops::default();
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
