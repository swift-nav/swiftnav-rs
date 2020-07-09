use crate::{
    c_bindings,
    signal::{Code, Constellation},
    time::GpsTime,
    AzEl, Vec3,
};
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug)]
pub enum Error {
    InvalidEphemeris,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "Invalid Ephemeris")
    }
}

impl std::error::Error for Error {}

type Result<T> = std::result::Result<T, Error>;

#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum Status {
    Null = c_bindings::ephemeris_status_t_EPH_NULL,
    Invalid = c_bindings::ephemeris_status_t_EPH_INVALID,
    WnEqualsZero = c_bindings::ephemeris_status_t_EPH_WN_EQ_0,
    FitIntervalEqualsZero = c_bindings::ephemeris_status_t_EPH_FIT_INTERVAL_EQ_0,
    Unhealthy = c_bindings::ephemeris_status_t_EPH_UNHEALTHY,
    TooOld = c_bindings::ephemeris_status_t_EPH_TOO_OLD,
    Valid = c_bindings::ephemeris_status_t_EPH_VALID,
}

impl Status {
    fn from_ephemeris_status_t(value: c_bindings::ephemeris_status_t) -> Status {
        match value {
            c_bindings::ephemeris_status_t_EPH_NULL => Status::Null,
            c_bindings::ephemeris_status_t_EPH_INVALID => Status::Invalid,
            c_bindings::ephemeris_status_t_EPH_WN_EQ_0 => Status::WnEqualsZero,
            c_bindings::ephemeris_status_t_EPH_FIT_INTERVAL_EQ_0 => Status::FitIntervalEqualsZero,
            c_bindings::ephemeris_status_t_EPH_UNHEALTHY => Status::Unhealthy,
            c_bindings::ephemeris_status_t_EPH_TOO_OLD => Status::TooOld,
            c_bindings::ephemeris_status_t_EPH_VALID => Status::Valid,
            _ => panic!("Invalid ephemeris_status_t value: {}", value),
        }
    }
}

pub enum EphemerisTerms {
    Kepler(c_bindings::ephemeris_kepler_t),
    Xyz(c_bindings::ephemeris_xyz_t),
    Glo(c_bindings::ephemeris_glo_t),
}

impl EphemerisTerms {
    pub fn new_kepler(
        constellation: Constellation,
        tgd: [f32; 2],
        crc: f64,
        crs: f64,
        cuc: f64,
        cus: f64,
        cic: f64,
        cis: f64,
        dn: f64,
        m0: f64,
        ecc: f64,
        sqrta: f64,
        omega0: f64,
        omegadot: f64,
        w: f64,
        inc: f64,
        inc_dot: f64,
        af0: f64,
        af1: f64,
        af2: f64,
        toc: GpsTime,
        iodc: u16,
        iode: u16,
    ) -> EphemerisTerms {
        EphemerisTerms::Kepler(c_bindings::ephemeris_kepler_t {
            tgd: match constellation {
                Constellation::Gps => c_bindings::ephemeris_kepler_t__bindgen_ty_1 { gps_s: tgd },
                Constellation::Qzs => c_bindings::ephemeris_kepler_t__bindgen_ty_1 { qzss_s: tgd },
                Constellation::Bds => c_bindings::ephemeris_kepler_t__bindgen_ty_1 { bds_s: tgd },
                Constellation::Gal => c_bindings::ephemeris_kepler_t__bindgen_ty_1 { gal_s: tgd },
                _ => panic!("Invalid constellation for a Kepler ephemeris"),
            },
            crc,
            crs,
            cuc,
            cus,
            cic,
            cis,
            dn,
            m0,
            ecc,
            sqrta,
            omega0,
            omegadot,
            w,
            inc,
            inc_dot,
            af0,
            af1,
            af2,
            toc: toc.to_gnss_time_t(),
            iodc,
            iode,
        })
    }

    pub fn new_xyz(
        pos: [f64; 3],
        vel: [f64; 3],
        acc: [f64; 3],
        a_gf0: f64,
        a_gf1: f64,
    ) -> EphemerisTerms {
        EphemerisTerms::Xyz(c_bindings::ephemeris_xyz_t {
            pos,
            vel,
            acc,
            a_gf0,
            a_gf1,
        })
    }

    pub fn new_glo(
        gamma: f64,
        tau: f64,
        d_tau: f64,
        pos: [f64; 3],
        vel: [f64; 3],
        acc: [f64; 3],
        fcn: u16,
        iod: u8,
    ) -> EphemerisTerms {
        EphemerisTerms::Glo(c_bindings::ephemeris_glo_t {
            gamma,
            tau,
            d_tau,
            pos,
            vel,
            acc,
            fcn,
            iod,
        })
    }
}

pub struct Ephemeris(c_bindings::ephemeris_t);

impl Ephemeris {
    pub fn new(
        sid: crate::signal::GnssSignal,
        toe: crate::time::GpsTime,
        ura: f32,
        fit_interval: u32,
        valid: u8,
        health_bits: u8,
        source: u8,
        terms: EphemerisTerms,
    ) -> Ephemeris {
         Ephemeris(c_bindings::ephemeris_t {
            sid: sid.to_gnss_signal_t(),
            toe: toe.to_gnss_time_t(),
            ura,
            fit_interval,
            valid,
            health_bits,
            source,
            __bindgen_anon_1: match terms {
                EphemerisTerms::Kepler(c_kepler) => c_bindings::ephemeris_t__bindgen_ty_1{ kepler: c_kepler },
                EphemerisTerms::Xyz(c_xyz) => c_bindings::ephemeris_t__bindgen_ty_1{ xyz: c_xyz },
                EphemerisTerms::Glo(c_glo) => c_bindings::ephemeris_t__bindgen_ty_1{ glo: c_glo },
            },
        })
    }

    pub fn calc_satellite_state(&self, t: &GpsTime) -> Result<SatelliteState> {
        let mut sat = SatelliteState {
            pos: Vec3::default(),
            vel: Vec3::default(),
            acc: Vec3::default(),
            clock_err: 0.0,
            clock_rate_err: 0.0,
            iodc: 0,
            iode: 0,
        };

        let result = unsafe {
            c_bindings::calc_sat_state(
                &self.0,
                t.to_gnss_time_ptr(),
                sat.pos.as_mut_ptr(),
                sat.vel.as_mut_ptr(),
                sat.acc.as_mut_ptr(),
                &mut sat.clock_err,
                &mut sat.clock_rate_err,
                &mut sat.iodc,
                &mut sat.iode,
            )
        };

        if result == 0 {
            Ok(sat)
        } else {
            Err(Error::InvalidEphemeris)
        }
    }

    pub fn calc_satellite_az_el(&self, t: &GpsTime, pos: &Vec3) -> Result<AzEl> {
        let mut sat = AzEl { az: 0.0, el: 0.0 };

        let result = unsafe {
            c_bindings::calc_sat_az_el(
                &self.0,
                t.to_gnss_time_ptr(),
                pos.as_ptr(),
                &mut sat.az,
                &mut sat.el,
                true,
            )
        };

        if result == 0 {
            Ok(sat)
        } else {
            Err(Error::InvalidEphemeris)
        }
    }

    pub fn calc_satellite_doppler(&self, t: &GpsTime, pos: &Vec3, vel: &Vec3) -> Result<f64> {
        let mut doppler = 0.0;

        let result = unsafe {
            c_bindings::calc_sat_doppler(
                &self.0,
                t.to_gnss_time_ptr(),
                pos.as_ptr(),
                vel.as_ptr(),
                &mut doppler,
            )
        };

        if result == 0 {
            Ok(doppler)
        } else {
            Err(Error::InvalidEphemeris)
        }
    }

    pub fn get_status(&self) -> Status {
        Status::from_ephemeris_status_t(unsafe { c_bindings::get_ephemeris_status_t(&self.0) })
    }

    pub fn get_status_at_time(&self, t: &GpsTime) -> Status {
        Status::from_ephemeris_status_t(unsafe {
            c_bindings::ephemeris_valid_detailed(&self.0, t.to_gnss_time_ptr())
        })
    }

    pub fn is_valid_at_time(&self, t: &GpsTime) -> bool {
        let result = unsafe { c_bindings::ephemeris_valid(&self.0, t.to_gnss_time_ptr()) };
        result == 0
    }

    pub fn is_healthy(&self, code: &Code) -> bool {
        unsafe { c_bindings::ephemeris_healthy(&self.0, code.to_code_t()) }
    }

    pub fn get_iod_or_iodcrc(&self) -> u32 {
        unsafe { c_bindings::get_ephemeris_iod_or_iodcrc(&self.0) }
    }
}

impl PartialEq for Ephemeris {
    fn eq(&self, other: &Self) -> bool {
        unsafe { c_bindings::ephemeris_equal(&self.0, &other.0) }
    }
}

impl Eq for Ephemeris {}

pub struct SatelliteState {
    pub pos: Vec3,
    pub vel: Vec3,
    pub acc: Vec3,
    pub clock_err: f64,
    pub clock_rate_err: f64,
    pub iodc: u16,
    pub iode: u8,
}
