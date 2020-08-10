//! Decoding and evaluation of satellite ephemeris
//!
//! GNSS satellites broadcast ephemeris, values used to calculate their position
//! in space over a period of time. Libswiftnav is able to decode the raw
//! ephemeris data and then evaluate the ephemeris.
//!
//! Broadcast ephemerides are only valid of a particular period of time, and the
//! constellations will update the ephemerides regularly to make sure they are
//! always valid when they need to be.

use crate::{
    c_bindings,
    coords::AzimuthElevation,
    signal::{Code, Constellation, GnssSignal},
    time::GpsTime,
    Vec3,
};
use std::fmt::{Display, Formatter};

/// Number of bytes in  the Galileo INAV message
// TODO(jbangelo) bindgen doesn't catch this variable on linux for some reason
pub const GAL_INAV_CONTENT_BYTE: usize = ((128 + 8 - 1) / 8);

/// An error indicating that the ephemeris is invalid
#[derive(Copy, Clone, Debug)]
pub struct InvalidEphemeris {}

impl Display for InvalidEphemeris {
    fn fmt(&self, f: &mut Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "Invalid Ephemeris")
    }
}

impl std::error::Error for InvalidEphemeris {}

type Result<T> = std::result::Result<T, InvalidEphemeris>;

/// Various statuses that an ephemeris can be in
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

/// Orbital terms of an ephemeris
pub enum EphemerisTerms {
    /// GPS, BDS, GAL, and QZSS all broadcast their terms as keplarian elements
    Kepler(c_bindings::ephemeris_kepler_t),
    /// SBAS systems broadcast their terms as simple XYZ terms
    Xyz(c_bindings::ephemeris_xyz_t),
    /// GLONASS broadcast their terms in a unique format and timeframe
    Glo(c_bindings::ephemeris_glo_t),
}

impl EphemerisTerms {
    /// Create new keplarian ephemeris terms from already decoded data
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
            toc: toc.to_gps_time_t(),
            iodc,
            iode,
        })
    }

    /// Create new XYZ ephemeris terms from already decoded data
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

    /// Create new GLONASS ephemeris terms from already decoded data
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

/// Representation of full ephemeris
pub struct Ephemeris(c_bindings::ephemeris_t);

impl Ephemeris {
    /// Create new ephemeris from already decoded data
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
            toe: toe.to_gps_time_t(),
            ura,
            fit_interval,
            valid,
            health_bits,
            source,
            __bindgen_anon_1: match terms {
                EphemerisTerms::Kepler(c_kepler) => {
                    assert!(matches!(
                        sid.to_constellation(),
                        Constellation::Gps | Constellation::Gal | Constellation::Bds
                    ));
                    c_bindings::ephemeris_t__bindgen_ty_1 { kepler: c_kepler }
                }
                EphemerisTerms::Xyz(c_xyz) => {
                    assert_eq!(sid.to_constellation(), Constellation::Sbas);
                    c_bindings::ephemeris_t__bindgen_ty_1 { xyz: c_xyz }
                }
                EphemerisTerms::Glo(c_glo) => {
                    assert_eq!(sid.to_constellation(), Constellation::Glo);
                    c_bindings::ephemeris_t__bindgen_ty_1 { glo: c_glo }
                }
            },
        })
    }

    /// Decode ephemeris from L1 C/A GPS navigation message frames.
    ///
    /// This function does not check for parity errors. You should check the
    /// subframes for parity errors before calling this function.
    ///
    /// `frame_words` is an array containing words 3 through 10 of subframes 1,
    /// 2 and 3. Word is in the 30 LSBs of the u32.
    ///
    /// `tot_tow` Is the time of transmission
    ///
    /// References:
    ///   -# IS-GPS-200D, Section 20.3.2 and Figure 20-1
    pub fn decode_gps(frame_words: &[[u32; 8]; 3], tot_tow: f64) -> Ephemeris {
        let mut e = Ephemeris::default();
        unsafe {
            c_bindings::decode_ephemeris(frame_words, e.mut_c_ptr(), tot_tow);
        }
        e
    }

    /// Decodes Beidou D1 ephemeris.
    /// `words` should contain subframes (FraID) 1,2,3.
    pub fn decode_bds(words: &[[u32; 10]; 3], sid: GnssSignal) -> Ephemeris {
        let mut e = Ephemeris::default();
        unsafe {
            c_bindings::decode_bds_d1_ephemeris(words, sid.to_gnss_signal_t(), e.mut_c_ptr());
        }
        e
    }

    /// Decodes GAL ephemeris.
    /// `page` should contain GAL pages 1-5. Page 5 is needed to extract Galileo
    /// system time (GST) and make corrections to TOE and TOC if needed.
    pub fn decode_gal(page: &[[u8; GAL_INAV_CONTENT_BYTE]; 5]) -> Ephemeris {
        let mut e = Ephemeris::default();
        unsafe {
            c_bindings::decode_gal_ephemeris(page, e.mut_c_ptr());
        }
        e
    }

    // TODO Add GLONASS decoding, needs UTC params though

    pub(crate) fn mut_c_ptr(&mut self) -> *mut c_bindings::ephemeris_t {
        &mut self.0
    }


    /// Calculate satellite position, velocity and clock offset from ephemeris.
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
                t.c_ptr(),
                sat.pos.as_mut_array_ref(),
                sat.vel.as_mut_array_ref(),
                sat.acc.as_mut_array_ref(),
                &mut sat.clock_err,
                &mut sat.clock_rate_err,
                &mut sat.iodc,
                &mut sat.iode,
            )
        };

        if result == 0 {
            Ok(sat)
        } else {
            Err(InvalidEphemeris {})
        }
    }

    /// Calculate the azimuth and elevation of a satellite from a reference
    /// position given the satellite ephemeris.
    pub fn calc_satellite_az_el(&self, t: &GpsTime, pos: &Vec3) -> Result<AzimuthElevation> {
        let mut sat = AzimuthElevation::default();

        let result = unsafe {
            c_bindings::calc_sat_az_el(
                &self.0,
                t.c_ptr(),
                pos.as_array_ref(),
                &mut sat.az,
                &mut sat.el,
                true,
            )
        };

        if result == 0 {
            Ok(sat)
        } else {
            Err(InvalidEphemeris {})
        }
    }

    /// Calculate the Doppler shift of a satellite as observed at a reference
    /// position given the satellite ephemeris.
    pub fn calc_satellite_doppler(&self, t: &GpsTime, pos: &Vec3, vel: &Vec3) -> Result<f64> {
        let mut doppler = 0.0;

        let result = unsafe {
            c_bindings::calc_sat_doppler(
                &self.0,
                t.c_ptr(),
                pos.as_array_ref(),
                vel.as_array_ref(),
                &mut doppler,
            )
        };

        if result == 0 {
            Ok(doppler)
        } else {
            Err(InvalidEphemeris {})
        }
    }

    /// Gets the status of an ephemeris - is the ephemeris invalid, unhealthy,
    /// or has some other condition which makes it unusable?
    pub fn get_status(&self) -> Status {
        Status::from_ephemeris_status_t(unsafe { c_bindings::get_ephemeris_status_t(&self.0) })
    }

    /// Is this ephemeris usable?
    pub fn is_valid_at_time(&self, t: &GpsTime) -> bool {
        let result = unsafe { c_bindings::ephemeris_valid(&self.0, t.c_ptr()) };
        result == 0
    }

    /// Check if this this ephemeris is healthy
    pub fn is_healthy(&self, code: &Code) -> bool {
        unsafe { c_bindings::ephemeris_healthy(&self.0, code.to_code_t()) }
    }

    /// Get the ephemeris iod. For BDS, returns a crc value uniquely identifying
    /// the satellite ephemeris set; for all other constellations, returns the IODE
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

impl Default for Ephemeris {
    fn default() -> Self {
        unsafe { std::mem::zeroed::<Ephemeris>() }
    }
}

/// Representation of a satellite state from evaluating its ephemeris at a
/// certain time.
pub struct SatelliteState {
    /// Calculated satellite position, in meters
    pub pos: Vec3,
    /// Calculated satellite velocity, in meters/second
    pub vel: Vec3,
    /// Calculated satellite acceleration, meters/second/second
    pub acc: Vec3,
    /// Calculated satellite clock error, in seconds
    pub clock_err: f64,
    /// Calculated satellite clock error rate, in seconds/second
    pub clock_rate_err: f64,
    /// Issue of data clock, unitless
    pub iodc: u16,
    /// Issue of data ephemeris, unitless
    pub iode: u8,
}

#[cfg(test)]
mod tests {
    use crate::ephemeris::{Ephemeris, EphemerisTerms};
    use crate::signal::{Code, Constellation, GnssSignal};
    use crate::time::GpsTime;
    use std::os::raw::c_int;

    #[test]
    fn bds_decode() {
        let expected_ephemeris = Ephemeris::new(
            GnssSignal::new(25, Code::Bds2B1),      // sid
            GpsTime::new_unchecked(2091, 460800.0), // toe
            2.0,                                    //ura
            0,                                      // fit_interval
            0,                                      // valid
            0,                                      // health_bits
            0,                                      // source
            EphemerisTerms::new_kepler(
                Constellation::Bds,
                [-2.99999997e-10, -2.99999997e-10],    // tgd
                167.140625,                            // crc
                -18.828125,                            // crs
                -9.0105459094047546e-07,               // cuc
                9.4850547611713409e-06,                // cus
                -4.0978193283081055e-08,               // cic
                1.0104849934577942e-07,                // cis
                3.9023054038264214e-09,                // dn
                0.39869951815527438,                   // m0
                0.00043709692545235157,                // ecc
                5282.6194686889648,                    // sqrta
                2.2431156200949509,                    // omega0
                -6.6892072037584707e-09,               // omegadot
                0.39590413040186828,                   // w
                0.95448398903792575,                   // inc
                -6.2716898124832475e-10,               // inc_dot
                -0.00050763087347149849,               // af0
                -1.3019807454384136e-11,               // af1
                0.000000,                              // af2
                GpsTime::new_unchecked(2091, 460800.), // toc
                160,                                   // iodc
                160,                                   // iode
            ),
        );

        let words: [[u32; 10]; 3] = [
            [
                0x38901714, 0x5F81035, 0x5BEE184, 0x3FDF95, 0x3D0B09CA, 0x3C47CDE6, 0x19AC7AD,
                0x24005E73, 0x2ED79F72, 0x38D7A13C,
            ],
            [
                0x38902716, 0x610AAF9, 0x2EFE1C86, 0x1103E979, 0x18E80030, 0x394A8A9E, 0x4F9109A,
                0x29C9FE18, 0x34BA516C, 0x13D2B18F,
            ],
            [
                0x38903719, 0x62B0869, 0x4DC786, 0x1087FF8F, 0x3D47FD49, 0x2DAE0084, 0x1B3C9264,
                0xB6C9161, 0x1B58811D, 0x2DC18C7,
            ],
        ];

        let sid = GnssSignal::new(25, Code::Bds2B1);

        let decoded_eph = Ephemeris::decode_bds(&words, sid);

        assert!(expected_ephemeris == decoded_eph);
    }

    #[test]
    fn gal_decode() {
        use super::GAL_INAV_CONTENT_BYTE;

        let expected_ephemeris = Ephemeris::new(
            GnssSignal::new(8, Code::GalE1b),      // sid
            GpsTime::new_unchecked(2090, 135000.), // toe
            3.120000,                              // ura
            14400,                                 // fit_interval
            1,                                     // valid
            0,                                     // health_bits
            0,                                     // source
            EphemerisTerms::new_kepler(
                Constellation::Gal,
                [0., 0.],                              // tgd
                62.375,                                // crs
                -54.0625,                              // crc
                -2.3748725652694702e-06,               // cuc
                1.2902542948722839e-05,                // cus
                7.4505805969238281e-09,                // cic
                4.6566128730773926e-08,                // cis
                2.9647663515616992e-09,                // dn
                1.1731263781996162,                    // m0
                0.00021702353842556477,                // ecc
                5440.6276874542236,                    // sqrta
                0.7101536200630526,                    // omega0
                -5.363080536688408e-09,                // omegadot
                0.39999676368790066,                   // w
                0.95957029480011957,                   // inc
                4.3751822439020375e-10,                // inc_dot
                0.0062288472545333198,                 // af0
                -5.4427573559223666e-12,               // af1
                0.,                                    // af2
                GpsTime::new_unchecked(2090, 135000.), // toc
                97,                                    // iode
                97,                                    // iodc
            ),
        );

        let words: [[u8; GAL_INAV_CONTENT_BYTE]; 5] = [
            [
                0x4, 0x61, 0x23, 0x28, 0xBF, 0x30, 0x9B, 0xA0, 0x0, 0x71, 0xC8, 0x6A, 0xA8, 0x14,
                0x16, 0x7,
            ],
            [
                0x8, 0x61, 0x1C, 0xEF, 0x2B, 0xC3, 0x27, 0x18, 0xAE, 0x65, 0x10, 0x4C, 0x1E, 0x1A,
                0x13, 0x25,
            ],
            [
                0xC, 0x61, 0xFF, 0xC5, 0x58, 0x20, 0x6D, 0xFB, 0x5, 0x1B, 0xF, 0x7, 0xCC, 0xF9,
                0x3E, 0x6B,
            ],
            [
                0x10, 0x61, 0x20, 0x0, 0x10, 0x0, 0x64, 0x8C, 0xA0, 0xCC, 0x1B, 0x5B, 0xBF, 0xFE,
                0x81, 0x1,
            ],
            [
                0x14, 0x50, 0x80, 0x20, 0x5, 0x81, 0xF4, 0x7C, 0x80, 0x21, 0x51, 0x9, 0xB6, 0xAA,
                0xAA, 0xAA,
            ],
        ];

        let mut decoded_eph = Ephemeris::decode_gal(&words);

        decoded_eph.0.sid.code = Code::GalE1b as c_int;
        decoded_eph.0.valid = 1;

        assert!(expected_ephemeris == decoded_eph);
    }
}
