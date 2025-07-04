// Copyright (c) 2020-2021 Swift Navigation Inc.
// Contact: Swift Navigation <dev@swiftnav.com>
//
// This source is subject to the license found in the file 'LICENSE' which must
// be be distributed together with this source. All other rights reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND,
// EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A PARTICULAR PURPOSE.
//! Signal identifiers
//!
//! Signals are specific to a satellite and code combination. A satellite is
//! identified by it's assigned number and the constellation it belongs to. Each
//! satellite can send out multiple signals.

use std::borrow::Cow;
use std::error::Error;
use std::ffi;
use std::fmt;
use std::str::FromStr;

/// GNSS satellite constellations
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Constellation {
    /// GPS
    Gps,
    /// SBAS - Space based  augmentation systems
    Sbas,
    /// GLONASS
    Glo,
    /// Beidou
    Bds,
    /// QZSS
    Qzs,
    /// Galileo
    Gal,
}

/// Invalid constellation integer value
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct InvalidConstellation(swiftnav_sys::constellation_t);

impl fmt::Display for InvalidConstellation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid constellation integer value: {}", self.0)
    }
}

impl Error for InvalidConstellation {}

impl Constellation {
    fn from_constellation_t(
        value: swiftnav_sys::constellation_t,
    ) -> Result<Constellation, InvalidConstellation> {
        match value {
            swiftnav_sys::constellation_e_CONSTELLATION_GPS => Ok(Constellation::Gps),
            swiftnav_sys::constellation_e_CONSTELLATION_SBAS => Ok(Constellation::Sbas),
            swiftnav_sys::constellation_e_CONSTELLATION_GLO => Ok(Constellation::Glo),
            swiftnav_sys::constellation_e_CONSTELLATION_BDS => Ok(Constellation::Bds),
            swiftnav_sys::constellation_e_CONSTELLATION_QZS => Ok(Constellation::Qzs),
            swiftnav_sys::constellation_e_CONSTELLATION_GAL => Ok(Constellation::Gal),
            _ => Err(InvalidConstellation(value)),
        }
    }

    pub(crate) fn to_constellation_t(self) -> swiftnav_sys::constellation_t {
        match self {
            Constellation::Gps => swiftnav_sys::constellation_e_CONSTELLATION_GPS,
            Constellation::Sbas => swiftnav_sys::constellation_e_CONSTELLATION_SBAS,
            Constellation::Glo => swiftnav_sys::constellation_e_CONSTELLATION_GLO,
            Constellation::Bds => swiftnav_sys::constellation_e_CONSTELLATION_BDS,
            Constellation::Qzs => swiftnav_sys::constellation_e_CONSTELLATION_QZS,
            Constellation::Gal => swiftnav_sys::constellation_e_CONSTELLATION_GAL,
        }
    }

    /// Gets the specified maximum number of active satellites for the constellation
    pub fn sat_count(&self) -> u16 {
        unsafe { swiftnav_sys::constellation_to_sat_count(*self as swiftnav_sys::constellation_t) }
    }

    /// Get the human readable name of the constellation.
    pub fn to_str(&self) -> Cow<'static, str> {
        let c_str = unsafe {
            ffi::CStr::from_ptr(swiftnav_sys::constellation_to_string(
                self.to_constellation_t(),
            ))
        };
        c_str.to_string_lossy()
    }
}

impl FromStr for Constellation {
    type Err = InvalidConstellation;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c_str = ffi::CString::new(s).map_err(|_| InvalidConstellation(-1))?;
        let constellation = unsafe { swiftnav_sys::constellation_string_to_enum(c_str.as_ptr()) };

        Self::from_constellation_t(constellation)
    }
}

impl fmt::Display for Constellation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl std::convert::TryFrom<u8> for Constellation {
    type Error = InvalidConstellation;
    fn try_from(value: u8) -> Result<Constellation, InvalidConstellation> {
        Self::from_constellation_t(value as swiftnav_sys::constellation_t)
    }
}

/// Code identifiers
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Code {
    /// GPS L1CA: BPSK(1)
    GpsL1ca,
    /// GPS L2C: 2 x BPSK(0.5)
    GpsL2cm,
    /// SBAS L1: BPSK(1)
    SbasL1ca,
    /// GLONASS L1OF: FDMA BPSK(0.5)
    GloL1of,
    /// GLONASS L2OF: FDMA BPSK(0.5)
    GloL2of,
    /// GPS L1P(Y): encrypted BPSK(10)
    GpsL1p,
    /// GPS L2P(Y): encrypted BPSK(10)
    GpsL2p,
    GpsL2cl,
    GpsL2cx,
    /// GPS L5: QPSK(10) at 1150*f0
    GpsL5i,
    GpsL5q,
    GpsL5x,
    /// BDS2 B1I: BPSK(2) at 1526*f0
    Bds2B1,
    /// BDS2 B2I: BPSK(2) at 1180*f0
    Bds2B2,
    /// Galileo E1: CASM CBOC(1,1) at 1540*f0
    GalE1b,
    GalE1c,
    GalE1x,
    /// Galileo E6: CASM BPSK(5) at 1250*f0
    GalE6b,
    GalE6c,
    GalE6x,
    /// Galileo E5b: QPSK(10) at 1180*f0
    GalE7i,
    GalE7q,
    GalE7x,
    /// Galileo E5AltBOC(15,10) at 1165*f0
    GalE8i,
    GalE8q,
    GalE8x,
    /// Galileo E5a: QPSK(10) at 1150*f0
    GalE5i,
    GalE5q,
    GalE5x,
    /// GLONASS L1P: encrypted
    GloL1p,
    /// GLONASS L2P: encrypted
    GloL2p,
    /// QZSS L1CA: BPSK(1) at 1540*f0
    QzsL1ca,
    /// QZSS L1C: TM-BOC at 1540*f0
    QzsL1ci,
    QzsL1cq,
    QzsL1cx,
    /// QZSS L2C: 2 x BPSK(0.5) at 1200*f0
    QzsL2cm,
    QzsL2cl,
    QzsL2cx,
    /// QZSS L5: QPSK(10) at 1150*f0
    QzsL5i,
    QzsL5q,
    QzsL5x,
    /// SBAS L5: ? at 1150*f0
    SbasL5i,
    SbasL5q,
    SbasL5x,
    /// BDS3 B1C: TM-BOC at 1540*f0
    Bds3B1ci,
    Bds3B1cq,
    Bds3B1cx,
    /// BDS3 B2a: QPSK(10) at 1150*f0
    Bds3B5i,
    Bds3B5q,
    Bds3B5x,
    /// BDS3 B2b: QPSK(10) at 1180*f0
    Bds3B7i,
    Bds3B7q,
    Bds3B7x,
    /// BDS3 B3I: QPSK(10) at 1240*f0
    Bds3B3i,
    Bds3B3q,
    Bds3B3x,
    /// GPS L1C: TM-BOC at 1540*f0
    GpsL1ci,
    GpsL1cq,
    GpsL1cx,
    /// Auxiliary GPS antenna signals
    AuxGps,
    /// Auxiliary SBAS antenna signals
    AuxSbas,
    /// Auxiliary GAL antenna signals
    AuxGal,
    /// Auxiliary QZSS antenna signals
    AuxQzs,
    /// Auxiliary BDS antenna signals
    AuxBds,
}

/// Invalid code integer value
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct InvalidCode(swiftnav_sys::code_t);

impl fmt::Display for InvalidCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid code integer value: {}", self.0)
    }
}

impl Error for InvalidCode {}

impl Code {
    pub(crate) fn from_code_t(value: swiftnav_sys::code_t) -> Result<Code, InvalidCode> {
        match value {
            swiftnav_sys::code_e_CODE_GPS_L1CA => Ok(Code::GpsL1ca),
            swiftnav_sys::code_e_CODE_GPS_L2CM => Ok(Code::GpsL2cm),
            swiftnav_sys::code_e_CODE_SBAS_L1CA => Ok(Code::SbasL1ca),
            swiftnav_sys::code_e_CODE_GLO_L1OF => Ok(Code::GloL1of),
            swiftnav_sys::code_e_CODE_GLO_L2OF => Ok(Code::GloL2of),
            swiftnav_sys::code_e_CODE_GPS_L1P => Ok(Code::GpsL1p),
            swiftnav_sys::code_e_CODE_GPS_L2P => Ok(Code::GpsL2p),
            swiftnav_sys::code_e_CODE_GPS_L2CL => Ok(Code::GpsL2cl),
            swiftnav_sys::code_e_CODE_GPS_L2CX => Ok(Code::GpsL2cx),
            swiftnav_sys::code_e_CODE_GPS_L5I => Ok(Code::GpsL5i),
            swiftnav_sys::code_e_CODE_GPS_L5Q => Ok(Code::GpsL5q),
            swiftnav_sys::code_e_CODE_GPS_L5X => Ok(Code::GpsL5x),
            swiftnav_sys::code_e_CODE_BDS2_B1 => Ok(Code::Bds2B1),
            swiftnav_sys::code_e_CODE_BDS2_B2 => Ok(Code::Bds2B2),
            swiftnav_sys::code_e_CODE_GAL_E1B => Ok(Code::GalE1b),
            swiftnav_sys::code_e_CODE_GAL_E1C => Ok(Code::GalE1c),
            swiftnav_sys::code_e_CODE_GAL_E1X => Ok(Code::GalE1x),
            swiftnav_sys::code_e_CODE_GAL_E6B => Ok(Code::GalE6b),
            swiftnav_sys::code_e_CODE_GAL_E6C => Ok(Code::GalE6c),
            swiftnav_sys::code_e_CODE_GAL_E6X => Ok(Code::GalE6x),
            swiftnav_sys::code_e_CODE_GAL_E7I => Ok(Code::GalE7i),
            swiftnav_sys::code_e_CODE_GAL_E7Q => Ok(Code::GalE7q),
            swiftnav_sys::code_e_CODE_GAL_E7X => Ok(Code::GalE7x),
            swiftnav_sys::code_e_CODE_GAL_E8I => Ok(Code::GalE8i),
            swiftnav_sys::code_e_CODE_GAL_E8Q => Ok(Code::GalE8q),
            swiftnav_sys::code_e_CODE_GAL_E8X => Ok(Code::GalE8x),
            swiftnav_sys::code_e_CODE_GAL_E5I => Ok(Code::GalE5i),
            swiftnav_sys::code_e_CODE_GAL_E5Q => Ok(Code::GalE5q),
            swiftnav_sys::code_e_CODE_GAL_E5X => Ok(Code::GalE5x),
            swiftnav_sys::code_e_CODE_GLO_L1P => Ok(Code::GloL1p),
            swiftnav_sys::code_e_CODE_GLO_L2P => Ok(Code::GloL2p),
            swiftnav_sys::code_e_CODE_QZS_L1CA => Ok(Code::QzsL1ca),
            swiftnav_sys::code_e_CODE_QZS_L1CI => Ok(Code::QzsL1ci),
            swiftnav_sys::code_e_CODE_QZS_L1CQ => Ok(Code::QzsL1cq),
            swiftnav_sys::code_e_CODE_QZS_L1CX => Ok(Code::QzsL1cx),
            swiftnav_sys::code_e_CODE_QZS_L2CM => Ok(Code::QzsL2cm),
            swiftnav_sys::code_e_CODE_QZS_L2CL => Ok(Code::QzsL2cl),
            swiftnav_sys::code_e_CODE_QZS_L2CX => Ok(Code::QzsL2cx),
            swiftnav_sys::code_e_CODE_QZS_L5I => Ok(Code::QzsL5i),
            swiftnav_sys::code_e_CODE_QZS_L5Q => Ok(Code::QzsL5q),
            swiftnav_sys::code_e_CODE_QZS_L5X => Ok(Code::QzsL5x),
            swiftnav_sys::code_e_CODE_SBAS_L5I => Ok(Code::SbasL5i),
            swiftnav_sys::code_e_CODE_SBAS_L5Q => Ok(Code::SbasL5q),
            swiftnav_sys::code_e_CODE_SBAS_L5X => Ok(Code::SbasL5x),
            swiftnav_sys::code_e_CODE_BDS3_B1CI => Ok(Code::Bds3B1ci),
            swiftnav_sys::code_e_CODE_BDS3_B1CQ => Ok(Code::Bds3B1cq),
            swiftnav_sys::code_e_CODE_BDS3_B1CX => Ok(Code::Bds3B1cx),
            swiftnav_sys::code_e_CODE_BDS3_B5I => Ok(Code::Bds3B5i),
            swiftnav_sys::code_e_CODE_BDS3_B5Q => Ok(Code::Bds3B5q),
            swiftnav_sys::code_e_CODE_BDS3_B5X => Ok(Code::Bds3B5x),
            swiftnav_sys::code_e_CODE_BDS3_B7I => Ok(Code::Bds3B7i),
            swiftnav_sys::code_e_CODE_BDS3_B7Q => Ok(Code::Bds3B7q),
            swiftnav_sys::code_e_CODE_BDS3_B7X => Ok(Code::Bds3B7x),
            swiftnav_sys::code_e_CODE_BDS3_B3I => Ok(Code::Bds3B3i),
            swiftnav_sys::code_e_CODE_BDS3_B3Q => Ok(Code::Bds3B3q),
            swiftnav_sys::code_e_CODE_BDS3_B3X => Ok(Code::Bds3B3x),
            swiftnav_sys::code_e_CODE_GPS_L1CI => Ok(Code::GpsL1ci),
            swiftnav_sys::code_e_CODE_GPS_L1CQ => Ok(Code::GpsL1cq),
            swiftnav_sys::code_e_CODE_GPS_L1CX => Ok(Code::GpsL1cx),
            swiftnav_sys::code_e_CODE_AUX_GPS => Ok(Code::AuxGps),
            swiftnav_sys::code_e_CODE_AUX_SBAS => Ok(Code::AuxSbas),
            swiftnav_sys::code_e_CODE_AUX_GAL => Ok(Code::AuxGal),
            swiftnav_sys::code_e_CODE_AUX_QZS => Ok(Code::AuxQzs),
            swiftnav_sys::code_e_CODE_AUX_BDS => Ok(Code::AuxBds),
            _ => Err(InvalidCode(value)),
        }
    }

    pub(crate) fn to_code_t(self) -> swiftnav_sys::code_t {
        match self {
            Code::GpsL1ca => swiftnav_sys::code_e_CODE_GPS_L1CA,
            Code::GpsL2cm => swiftnav_sys::code_e_CODE_GPS_L2CM,
            Code::SbasL1ca => swiftnav_sys::code_e_CODE_SBAS_L1CA,
            Code::GloL1of => swiftnav_sys::code_e_CODE_GLO_L1OF,
            Code::GloL2of => swiftnav_sys::code_e_CODE_GLO_L2OF,
            Code::GpsL1p => swiftnav_sys::code_e_CODE_GPS_L1P,
            Code::GpsL2p => swiftnav_sys::code_e_CODE_GPS_L2P,
            Code::GpsL2cl => swiftnav_sys::code_e_CODE_GPS_L2CL,
            Code::GpsL2cx => swiftnav_sys::code_e_CODE_GPS_L2CX,
            Code::GpsL5i => swiftnav_sys::code_e_CODE_GPS_L5I,
            Code::GpsL5q => swiftnav_sys::code_e_CODE_GPS_L5Q,
            Code::GpsL5x => swiftnav_sys::code_e_CODE_GPS_L5X,
            Code::Bds2B1 => swiftnav_sys::code_e_CODE_BDS2_B1,
            Code::Bds2B2 => swiftnav_sys::code_e_CODE_BDS2_B2,
            Code::GalE1b => swiftnav_sys::code_e_CODE_GAL_E1B,
            Code::GalE1c => swiftnav_sys::code_e_CODE_GAL_E1C,
            Code::GalE1x => swiftnav_sys::code_e_CODE_GAL_E1X,
            Code::GalE6b => swiftnav_sys::code_e_CODE_GAL_E6B,
            Code::GalE6c => swiftnav_sys::code_e_CODE_GAL_E6C,
            Code::GalE6x => swiftnav_sys::code_e_CODE_GAL_E6X,
            Code::GalE7i => swiftnav_sys::code_e_CODE_GAL_E7I,
            Code::GalE7q => swiftnav_sys::code_e_CODE_GAL_E7Q,
            Code::GalE7x => swiftnav_sys::code_e_CODE_GAL_E7X,
            Code::GalE8i => swiftnav_sys::code_e_CODE_GAL_E8I,
            Code::GalE8q => swiftnav_sys::code_e_CODE_GAL_E8Q,
            Code::GalE8x => swiftnav_sys::code_e_CODE_GAL_E8X,
            Code::GalE5i => swiftnav_sys::code_e_CODE_GAL_E5I,
            Code::GalE5q => swiftnav_sys::code_e_CODE_GAL_E5Q,
            Code::GalE5x => swiftnav_sys::code_e_CODE_GAL_E5X,
            Code::GloL1p => swiftnav_sys::code_e_CODE_GLO_L1P,
            Code::GloL2p => swiftnav_sys::code_e_CODE_GLO_L2P,
            Code::QzsL1ca => swiftnav_sys::code_e_CODE_QZS_L1CA,
            Code::QzsL1ci => swiftnav_sys::code_e_CODE_QZS_L1CI,
            Code::QzsL1cq => swiftnav_sys::code_e_CODE_QZS_L1CQ,
            Code::QzsL1cx => swiftnav_sys::code_e_CODE_QZS_L1CX,
            Code::QzsL2cm => swiftnav_sys::code_e_CODE_QZS_L2CM,
            Code::QzsL2cl => swiftnav_sys::code_e_CODE_QZS_L2CL,
            Code::QzsL2cx => swiftnav_sys::code_e_CODE_QZS_L2CX,
            Code::QzsL5i => swiftnav_sys::code_e_CODE_QZS_L5I,
            Code::QzsL5q => swiftnav_sys::code_e_CODE_QZS_L5Q,
            Code::QzsL5x => swiftnav_sys::code_e_CODE_QZS_L5X,
            Code::SbasL5i => swiftnav_sys::code_e_CODE_SBAS_L5I,
            Code::SbasL5q => swiftnav_sys::code_e_CODE_SBAS_L5Q,
            Code::SbasL5x => swiftnav_sys::code_e_CODE_SBAS_L5X,
            Code::Bds3B1ci => swiftnav_sys::code_e_CODE_BDS3_B1CI,
            Code::Bds3B1cq => swiftnav_sys::code_e_CODE_BDS3_B1CQ,
            Code::Bds3B1cx => swiftnav_sys::code_e_CODE_BDS3_B1CX,
            Code::Bds3B5i => swiftnav_sys::code_e_CODE_BDS3_B5I,
            Code::Bds3B5q => swiftnav_sys::code_e_CODE_BDS3_B5Q,
            Code::Bds3B5x => swiftnav_sys::code_e_CODE_BDS3_B5X,
            Code::Bds3B7i => swiftnav_sys::code_e_CODE_BDS3_B7I,
            Code::Bds3B7q => swiftnav_sys::code_e_CODE_BDS3_B7Q,
            Code::Bds3B7x => swiftnav_sys::code_e_CODE_BDS3_B7X,
            Code::Bds3B3i => swiftnav_sys::code_e_CODE_BDS3_B3I,
            Code::Bds3B3q => swiftnav_sys::code_e_CODE_BDS3_B3Q,
            Code::Bds3B3x => swiftnav_sys::code_e_CODE_BDS3_B3X,
            Code::GpsL1ci => swiftnav_sys::code_e_CODE_GPS_L1CI,
            Code::GpsL1cq => swiftnav_sys::code_e_CODE_GPS_L1CQ,
            Code::GpsL1cx => swiftnav_sys::code_e_CODE_GPS_L1CX,
            Code::AuxGps => swiftnav_sys::code_e_CODE_AUX_GPS,
            Code::AuxSbas => swiftnav_sys::code_e_CODE_AUX_SBAS,
            Code::AuxGal => swiftnav_sys::code_e_CODE_AUX_GAL,
            Code::AuxQzs => swiftnav_sys::code_e_CODE_AUX_QZS,
            Code::AuxBds => swiftnav_sys::code_e_CODE_AUX_BDS,
        }
    }

    /// Get the human readable name of the code.
    pub fn to_str(&self) -> Cow<'static, str> {
        let c_str = unsafe { ffi::CStr::from_ptr(swiftnav_sys::code_to_string(self.to_code_t())) };
        c_str.to_string_lossy()
    }

    /// Gets the corresponding [`Constellation`]
    pub fn to_constellation(&self) -> Constellation {
        Constellation::from_constellation_t(unsafe {
            swiftnav_sys::code_to_constellation(self.to_code_t())
        })
        .unwrap()
    }

    /// Get the number of signals for a code
    pub fn sig_count(&self) -> u16 {
        unsafe { swiftnav_sys::code_to_sig_count(self.to_code_t()) }
    }

    /// Get the chips count of a code
    pub fn chip_count(&self) -> u32 {
        unsafe { swiftnav_sys::code_to_chip_count(self.to_code_t()) }
    }

    /// Get the chips rate of a code
    pub fn chip_rate(&self) -> f64 {
        unsafe { swiftnav_sys::code_to_chip_rate(self.to_code_t()) }
    }

    pub fn is_gps(&self) -> bool {
        unsafe { swiftnav_sys::is_gps(self.to_code_t()) }
    }

    pub fn is_sbas(&self) -> bool {
        unsafe { swiftnav_sys::is_sbas(self.to_code_t()) }
    }

    pub fn is_glo(&self) -> bool {
        unsafe { swiftnav_sys::is_glo(self.to_code_t()) }
    }

    pub fn is_bds2(&self) -> bool {
        unsafe { swiftnav_sys::is_bds2(self.to_code_t()) }
    }

    pub fn is_gal(&self) -> bool {
        unsafe { swiftnav_sys::is_gal(self.to_code_t()) }
    }

    pub fn is_qzss(&self) -> bool {
        unsafe { swiftnav_sys::is_qzss(self.to_code_t()) }
    }
}

impl FromStr for Code {
    type Err = InvalidCode;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c_str = ffi::CString::new(s).map_err(|_| InvalidCode(-1))?;
        let code = unsafe { swiftnav_sys::code_string_to_enum(c_str.as_ptr()) };

        Self::from_code_t(code)
    }
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl std::convert::TryFrom<u8> for Code {
    type Error = InvalidCode;
    fn try_from(value: u8) -> Result<Code, InvalidCode> {
        Self::from_code_t(value as swiftnav_sys::code_t)
    }
}

/// GNSS Signal identifier
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct GnssSignal(swiftnav_sys::gnss_signal_t);

/// Invalid values when creating a [`GnssSignal`] object
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum InvalidGnssSignal {
    /// The code integer value was invalid
    InvalidCode(InvalidCode),
    /// The satellite number is not in the valid range for the associated constellation
    InvalidSatellite(u16),
}

impl fmt::Display for InvalidGnssSignal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InvalidGnssSignal::InvalidCode(code) => code.fmt(f),
            InvalidGnssSignal::InvalidSatellite(sat) => {
                write!(f, "Invalid satellite number: {sat}")
            }
        }
    }
}

impl Error for InvalidGnssSignal {}

impl From<InvalidCode> for InvalidGnssSignal {
    fn from(other: InvalidCode) -> InvalidGnssSignal {
        InvalidGnssSignal::InvalidCode(other)
    }
}

impl GnssSignal {
    pub fn new(sat: u16, code: Code) -> Result<GnssSignal, InvalidGnssSignal> {
        let code = code.to_code_t();
        let sid = swiftnav_sys::gnss_signal_t { sat, code };
        let sid_is_valid = unsafe { swiftnav_sys::sid_valid(sid) };
        if sid_is_valid {
            Ok(GnssSignal(sid))
        } else {
            Err(InvalidGnssSignal::InvalidSatellite(sat))
        }
    }

    pub(crate) fn from_gnss_signal_t(
        sid: swiftnav_sys::gnss_signal_t,
    ) -> Result<GnssSignal, InvalidGnssSignal> {
        GnssSignal::new(sid.sat, Code::from_code_t(sid.code)?)
    }

    pub(crate) fn to_gnss_signal_t(self) -> swiftnav_sys::gnss_signal_t {
        self.0
    }

    pub fn sat(&self) -> u16 {
        self.0.sat
    }

    pub fn code(&self) -> Code {
        Code::from_code_t(self.0.code).unwrap()
    }

    /// Get the constellation of the signal
    pub fn to_constellation(&self) -> Constellation {
        Constellation::from_constellation_t(unsafe { swiftnav_sys::sid_to_constellation(self.0) })
            .unwrap()
    }

    /// Get the carrier frequency of the signal
    pub fn carrier_frequency(&self) -> f64 {
        unsafe { swiftnav_sys::sid_to_carr_freq(self.0) }
    }

    /// Makes the human readable signal name
    pub fn to_str(&self) -> String {
        let mut raw_str = [0; swiftnav_sys::SID_STR_LEN_MAX as usize + 1];

        unsafe {
            let n_bytes = swiftnav_sys::sid_to_string(
                raw_str.as_mut_ptr(),
                raw_str.len() as i32 - 1,
                self.to_gnss_signal_t(),
            );
            raw_str[n_bytes as usize] = 0;

            let str = ffi::CStr::from_ptr(raw_str.as_ptr());

            str.to_string_lossy().to_string()
        }
    }
}

impl fmt::Display for GnssSignal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sat_count() {
        assert_eq!(Constellation::Gps.sat_count(), 32);
        assert_eq!(Constellation::Sbas.sat_count(), 19);
        assert_eq!(Constellation::Glo.sat_count(), 28);
        assert_eq!(Constellation::Bds.sat_count(), 64);
        assert_eq!(Constellation::Gal.sat_count(), 36);
        assert_eq!(Constellation::Qzs.sat_count(), 10);
    }

    #[test]
    fn code_to_constellation() {
        assert!(Code::GpsL1ca.is_gps());
        assert!(Code::GpsL2cm.is_gps());
        assert!(Code::SbasL1ca.is_sbas());
        assert!(Code::GloL1of.is_glo());
        assert!(Code::GloL2of.is_glo());
        assert!(Code::GpsL1p.is_gps());
        assert!(Code::GpsL2p.is_gps());
        assert!(Code::GpsL2cl.is_gps());
        assert!(Code::GpsL2cx.is_gps());
        assert!(Code::GpsL5i.is_gps());
        assert!(Code::GpsL5q.is_gps());
        assert!(Code::GpsL5x.is_gps());
        assert!(Code::Bds2B1.is_bds2());
        assert!(Code::Bds2B2.is_bds2());
        assert!(Code::GalE1b.is_gal());
        assert!(Code::GalE1c.is_gal());
        assert!(Code::GalE1x.is_gal());
        assert!(Code::GalE6b.is_gal());
        assert!(Code::GalE6c.is_gal());
        assert!(Code::GalE6x.is_gal());
        assert!(Code::GalE7i.is_gal());
        assert!(Code::GalE7q.is_gal());
        assert!(Code::GalE7x.is_gal());
        assert!(Code::GalE8i.is_gal());
        assert!(Code::GalE8q.is_gal());
        assert!(Code::GalE8x.is_gal());
        assert!(Code::GalE5i.is_gal());
        assert!(Code::GalE5q.is_gal());
        assert!(Code::GalE5x.is_gal());
        assert!(Code::GloL1p.is_glo());
        assert!(Code::GloL2p.is_glo());
        assert!(Code::QzsL1ca.is_qzss());
        assert!(Code::QzsL1ci.is_qzss());
        assert!(Code::QzsL1cq.is_qzss());
        assert!(Code::QzsL1cx.is_qzss());
        assert!(Code::QzsL2cm.is_qzss());
        assert!(Code::QzsL2cl.is_qzss());
        assert!(Code::QzsL2cx.is_qzss());
        assert!(Code::QzsL5i.is_qzss());
        assert!(Code::QzsL5q.is_qzss());
        assert!(Code::QzsL5x.is_qzss());
        assert!(Code::SbasL5i.is_sbas());
        assert!(Code::SbasL5q.is_sbas());
        assert!(Code::SbasL5x.is_sbas());
        assert!(Code::Bds3B1ci.is_bds2());
        assert!(Code::Bds3B1cq.is_bds2());
        assert!(Code::Bds3B1cx.is_bds2());
        assert!(Code::Bds3B5i.is_bds2());
        assert!(Code::Bds3B5q.is_bds2());
        assert!(Code::Bds3B5x.is_bds2());
        assert!(Code::Bds3B7i.is_bds2());
        assert!(Code::Bds3B7q.is_bds2());
        assert!(Code::Bds3B7x.is_bds2());
        assert!(Code::Bds3B3i.is_bds2());
        assert!(Code::Bds3B3q.is_bds2());
        assert!(Code::Bds3B3x.is_bds2());
        assert!(Code::GpsL1ci.is_gps());
        assert!(Code::GpsL1cq.is_gps());
        assert!(Code::GpsL1cx.is_gps());
        assert!(Code::AuxGps.is_gps());
        assert!(Code::AuxSbas.is_sbas());
        assert!(Code::AuxGal.is_gal());
        assert!(Code::AuxQzs.is_qzss());
        assert!(Code::AuxBds.is_bds2());
    }

    #[test]
    fn signal_to_constellation() {
        assert_eq!(
            GnssSignal::new(1, Code::GpsL1ca)
                .unwrap()
                .to_constellation(),
            Constellation::Gps
        );
        assert_eq!(
            GnssSignal::new(1, Code::GpsL2cm)
                .unwrap()
                .to_constellation(),
            Constellation::Gps
        );
        assert_eq!(
            GnssSignal::new(120, Code::SbasL1ca)
                .unwrap()
                .to_constellation(),
            Constellation::Sbas
        );
        assert_eq!(
            GnssSignal::new(1, Code::GloL1of)
                .unwrap()
                .to_constellation(),
            Constellation::Glo
        );
        assert_eq!(
            GnssSignal::new(1, Code::GloL2of)
                .unwrap()
                .to_constellation(),
            Constellation::Glo
        );
        assert_eq!(
            GnssSignal::new(1, Code::GpsL1p).unwrap().to_constellation(),
            Constellation::Gps
        );
        assert_eq!(
            GnssSignal::new(1, Code::GpsL2p).unwrap().to_constellation(),
            Constellation::Gps
        );
        assert_eq!(
            GnssSignal::new(1, Code::GpsL2cl)
                .unwrap()
                .to_constellation(),
            Constellation::Gps
        );
        assert_eq!(
            GnssSignal::new(1, Code::GpsL2cx)
                .unwrap()
                .to_constellation(),
            Constellation::Gps
        );
        assert_eq!(
            GnssSignal::new(1, Code::GpsL5i).unwrap().to_constellation(),
            Constellation::Gps
        );
        assert_eq!(
            GnssSignal::new(1, Code::GpsL5q).unwrap().to_constellation(),
            Constellation::Gps
        );
        assert_eq!(
            GnssSignal::new(1, Code::GpsL5x).unwrap().to_constellation(),
            Constellation::Gps
        );
        assert_eq!(
            GnssSignal::new(1, Code::Bds2B1).unwrap().to_constellation(),
            Constellation::Bds
        );
        assert_eq!(
            GnssSignal::new(1, Code::Bds2B2).unwrap().to_constellation(),
            Constellation::Bds
        );
        assert_eq!(
            GnssSignal::new(1, Code::GalE1b).unwrap().to_constellation(),
            Constellation::Gal
        );
        assert_eq!(
            GnssSignal::new(1, Code::GalE1c).unwrap().to_constellation(),
            Constellation::Gal
        );
        assert_eq!(
            GnssSignal::new(1, Code::GalE1x).unwrap().to_constellation(),
            Constellation::Gal
        );
        assert_eq!(
            GnssSignal::new(1, Code::GalE6b).unwrap().to_constellation(),
            Constellation::Gal
        );
        assert_eq!(
            GnssSignal::new(1, Code::GalE6c).unwrap().to_constellation(),
            Constellation::Gal
        );
        assert_eq!(
            GnssSignal::new(1, Code::GalE6x).unwrap().to_constellation(),
            Constellation::Gal
        );
        assert_eq!(
            GnssSignal::new(1, Code::GalE7i).unwrap().to_constellation(),
            Constellation::Gal
        );
        assert_eq!(
            GnssSignal::new(1, Code::GalE7q).unwrap().to_constellation(),
            Constellation::Gal
        );
        assert_eq!(
            GnssSignal::new(1, Code::GalE7x).unwrap().to_constellation(),
            Constellation::Gal
        );
        assert_eq!(
            GnssSignal::new(1, Code::GalE8i).unwrap().to_constellation(),
            Constellation::Gal
        );
        assert_eq!(
            GnssSignal::new(1, Code::GalE8q).unwrap().to_constellation(),
            Constellation::Gal
        );
        assert_eq!(
            GnssSignal::new(1, Code::GalE8x).unwrap().to_constellation(),
            Constellation::Gal
        );
        assert_eq!(
            GnssSignal::new(1, Code::GalE5i).unwrap().to_constellation(),
            Constellation::Gal
        );
        assert_eq!(
            GnssSignal::new(1, Code::GalE5q).unwrap().to_constellation(),
            Constellation::Gal
        );
        assert_eq!(
            GnssSignal::new(1, Code::GalE5x).unwrap().to_constellation(),
            Constellation::Gal
        );
        assert_eq!(
            GnssSignal::new(1, Code::GloL1p).unwrap().to_constellation(),
            Constellation::Glo
        );
        assert_eq!(
            GnssSignal::new(1, Code::GloL2p).unwrap().to_constellation(),
            Constellation::Glo
        );
        assert_eq!(
            GnssSignal::new(193, Code::QzsL1ca)
                .unwrap()
                .to_constellation(),
            Constellation::Qzs
        );
        assert_eq!(
            GnssSignal::new(193, Code::QzsL1ci)
                .unwrap()
                .to_constellation(),
            Constellation::Qzs
        );
        assert_eq!(
            GnssSignal::new(193, Code::QzsL1cq)
                .unwrap()
                .to_constellation(),
            Constellation::Qzs
        );
        assert_eq!(
            GnssSignal::new(193, Code::QzsL1cx)
                .unwrap()
                .to_constellation(),
            Constellation::Qzs
        );
        assert_eq!(
            GnssSignal::new(193, Code::QzsL2cm)
                .unwrap()
                .to_constellation(),
            Constellation::Qzs
        );
        assert_eq!(
            GnssSignal::new(193, Code::QzsL2cl)
                .unwrap()
                .to_constellation(),
            Constellation::Qzs
        );
        assert_eq!(
            GnssSignal::new(193, Code::QzsL2cx)
                .unwrap()
                .to_constellation(),
            Constellation::Qzs
        );
        assert_eq!(
            GnssSignal::new(193, Code::QzsL5i)
                .unwrap()
                .to_constellation(),
            Constellation::Qzs
        );
        assert_eq!(
            GnssSignal::new(193, Code::QzsL5q)
                .unwrap()
                .to_constellation(),
            Constellation::Qzs
        );
        assert_eq!(
            GnssSignal::new(193, Code::QzsL5x)
                .unwrap()
                .to_constellation(),
            Constellation::Qzs
        );
        assert_eq!(
            GnssSignal::new(120, Code::SbasL5i)
                .unwrap()
                .to_constellation(),
            Constellation::Sbas
        );
        assert_eq!(
            GnssSignal::new(120, Code::SbasL5q)
                .unwrap()
                .to_constellation(),
            Constellation::Sbas
        );
        assert_eq!(
            GnssSignal::new(120, Code::SbasL5x)
                .unwrap()
                .to_constellation(),
            Constellation::Sbas
        );
        assert_eq!(
            GnssSignal::new(1, Code::Bds3B1ci)
                .unwrap()
                .to_constellation(),
            Constellation::Bds
        );
        assert_eq!(
            GnssSignal::new(1, Code::Bds3B1cq)
                .unwrap()
                .to_constellation(),
            Constellation::Bds
        );
        assert_eq!(
            GnssSignal::new(1, Code::Bds3B1cx)
                .unwrap()
                .to_constellation(),
            Constellation::Bds
        );
        assert_eq!(
            GnssSignal::new(1, Code::Bds3B5i)
                .unwrap()
                .to_constellation(),
            Constellation::Bds
        );
        assert_eq!(
            GnssSignal::new(1, Code::Bds3B5q)
                .unwrap()
                .to_constellation(),
            Constellation::Bds
        );
        assert_eq!(
            GnssSignal::new(1, Code::Bds3B5x)
                .unwrap()
                .to_constellation(),
            Constellation::Bds
        );
        assert_eq!(
            GnssSignal::new(1, Code::Bds3B7i)
                .unwrap()
                .to_constellation(),
            Constellation::Bds
        );
        assert_eq!(
            GnssSignal::new(1, Code::Bds3B7q)
                .unwrap()
                .to_constellation(),
            Constellation::Bds
        );
        assert_eq!(
            GnssSignal::new(1, Code::Bds3B7x)
                .unwrap()
                .to_constellation(),
            Constellation::Bds
        );
        assert_eq!(
            GnssSignal::new(1, Code::Bds3B3i)
                .unwrap()
                .to_constellation(),
            Constellation::Bds
        );
        assert_eq!(
            GnssSignal::new(1, Code::Bds3B3q)
                .unwrap()
                .to_constellation(),
            Constellation::Bds
        );
        assert_eq!(
            GnssSignal::new(1, Code::Bds3B3x)
                .unwrap()
                .to_constellation(),
            Constellation::Bds
        );
        assert_eq!(
            GnssSignal::new(1, Code::GpsL1ci)
                .unwrap()
                .to_constellation(),
            Constellation::Gps
        );
        assert_eq!(
            GnssSignal::new(1, Code::GpsL1cq)
                .unwrap()
                .to_constellation(),
            Constellation::Gps
        );
        assert_eq!(
            GnssSignal::new(1, Code::GpsL1cx)
                .unwrap()
                .to_constellation(),
            Constellation::Gps
        );
        assert_eq!(
            GnssSignal::new(1, Code::AuxGps).unwrap().to_constellation(),
            Constellation::Gps
        );
        assert_eq!(
            GnssSignal::new(120, Code::AuxSbas)
                .unwrap()
                .to_constellation(),
            Constellation::Sbas
        );
        assert_eq!(
            GnssSignal::new(1, Code::AuxGal).unwrap().to_constellation(),
            Constellation::Gal
        );
        assert_eq!(
            GnssSignal::new(193, Code::AuxQzs)
                .unwrap()
                .to_constellation(),
            Constellation::Qzs
        );
        assert_eq!(
            GnssSignal::new(1, Code::AuxBds).unwrap().to_constellation(),
            Constellation::Bds
        );
    }

    #[test]
    fn invalid_sats() {
        let first = swiftnav_sys::GPS_FIRST_PRN;
        let last = swiftnav_sys::GPS_FIRST_PRN + swiftnav_sys::NUM_SATS_GPS;
        for sat in (first - 1)..(last + 2) {
            let result = GnssSignal::new(sat as u16, Code::GpsL1ca);
            if sat < first || sat >= last {
                assert!(result.is_err());
                assert_eq!(
                    result.unwrap_err(),
                    InvalidGnssSignal::InvalidSatellite(sat as u16)
                );
            } else {
                assert!(result.is_ok());
            }
        }

        let first = swiftnav_sys::SBAS_FIRST_PRN;
        let last = swiftnav_sys::SBAS_FIRST_PRN + swiftnav_sys::NUM_SATS_SBAS;
        for sat in (first - 1)..(last + 2) {
            let result = GnssSignal::new(sat as u16, Code::SbasL1ca);
            if sat < first || sat >= last {
                assert!(result.is_err());
                assert_eq!(
                    result.unwrap_err(),
                    InvalidGnssSignal::InvalidSatellite(sat as u16)
                );
            } else {
                assert!(result.is_ok());
            }
        }

        let first = swiftnav_sys::GLO_FIRST_PRN;
        let last = swiftnav_sys::GLO_FIRST_PRN + swiftnav_sys::NUM_SATS_GLO;
        for sat in (first - 1)..(last + 2) {
            let result = GnssSignal::new(sat as u16, Code::GloL1of);
            if sat < first || sat >= last {
                assert!(result.is_err());
                assert_eq!(
                    result.unwrap_err(),
                    InvalidGnssSignal::InvalidSatellite(sat as u16)
                );
            } else {
                assert!(result.is_ok());
            }
        }

        let first = swiftnav_sys::BDS_FIRST_PRN;
        let last = swiftnav_sys::BDS_FIRST_PRN + swiftnav_sys::NUM_SATS_BDS;
        for sat in (first - 1)..(last + 2) {
            let result = GnssSignal::new(sat as u16, Code::Bds2B1);
            if sat < first || sat >= last {
                assert!(result.is_err());
                assert_eq!(
                    result.unwrap_err(),
                    InvalidGnssSignal::InvalidSatellite(sat as u16)
                );
            } else {
                assert!(result.is_ok());
            }
        }

        let first = swiftnav_sys::GAL_FIRST_PRN;
        let last = swiftnav_sys::GAL_FIRST_PRN + swiftnav_sys::NUM_SATS_GAL;
        for sat in (first - 1)..(last + 2) {
            let result = GnssSignal::new(sat as u16, Code::GalE1b);
            if sat < first || sat >= last {
                assert!(result.is_err());
                assert_eq!(
                    result.unwrap_err(),
                    InvalidGnssSignal::InvalidSatellite(sat as u16)
                );
            } else {
                assert!(result.is_ok());
            }
        }

        let first = swiftnav_sys::QZS_FIRST_PRN;
        let last = swiftnav_sys::QZS_FIRST_PRN + swiftnav_sys::NUM_SATS_QZS;
        for sat in (first - 1)..(last + 2) {
            let result = GnssSignal::new(sat as u16, Code::QzsL1ca);
            if sat < first || sat >= last {
                assert!(result.is_err());
                assert_eq!(
                    result.unwrap_err(),
                    InvalidGnssSignal::InvalidSatellite(sat as u16)
                );
            } else {
                assert!(result.is_ok());
            }
        }
    }

    #[test]
    fn constellation_strings() {
        assert_eq!(Constellation::Gps.to_str(), "GPS");
        assert_eq!(Constellation::Sbas.to_str(), "SBAS");
        assert_eq!(Constellation::Glo.to_str(), "GLO");
        assert_eq!(Constellation::Bds.to_str(), "BDS");
        assert_eq!(Constellation::Qzs.to_str(), "QZS");
        assert_eq!(Constellation::Gal.to_str(), "GAL");

        assert_eq!(Constellation::from_str("GPS").unwrap(), Constellation::Gps);
        assert_eq!(
            Constellation::from_str("SBAS").unwrap(),
            Constellation::Sbas
        );
        assert_eq!(Constellation::from_str("GLO").unwrap(), Constellation::Glo);
        assert_eq!(Constellation::from_str("BDS").unwrap(), Constellation::Bds);
        assert_eq!(Constellation::from_str("QZS").unwrap(), Constellation::Qzs);
        assert_eq!(Constellation::from_str("GAL").unwrap(), Constellation::Gal);

        {
            let result = Constellation::from_str("Bad String");
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), InvalidConstellation(-1));
        }
        {
            let result = Constellation::from_str("Nul\0String");
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), InvalidConstellation(-1));
        }
        {
            let result = Constellation::from_str("💩💩💩💩");
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), InvalidConstellation(-1));
        }
    }

    #[test]
    fn code_strings() {
        assert_eq!(Code::GpsL1ca.to_str(), "GPS L1CA");
        assert_eq!(Code::GpsL2cm.to_str(), "GPS L2CM");
        assert_eq!(Code::SbasL1ca.to_str(), "SBAS L1");
        assert_eq!(Code::GloL1of.to_str(), "GLO L1OF");
        assert_eq!(Code::GloL2of.to_str(), "GLO L2OF");
        assert_eq!(Code::GpsL1p.to_str(), "GPS L1P");
        assert_eq!(Code::GpsL2p.to_str(), "GPS L2P");
        assert_eq!(Code::GpsL2cl.to_str(), "GPS L2CL");
        assert_eq!(Code::GpsL2cx.to_str(), "GPS L2C");
        assert_eq!(Code::GpsL5i.to_str(), "GPS L5I");
        assert_eq!(Code::GpsL5q.to_str(), "GPS L5Q");
        assert_eq!(Code::GpsL5x.to_str(), "GPS L5");
        assert_eq!(Code::Bds2B1.to_str(), "BDS B1");
        assert_eq!(Code::Bds2B2.to_str(), "BDS B2");
        assert_eq!(Code::GalE1b.to_str(), "GAL E1B");
        assert_eq!(Code::GalE1c.to_str(), "GAL E1C");
        assert_eq!(Code::GalE1x.to_str(), "GAL E1");
        assert_eq!(Code::GalE6b.to_str(), "GAL E6B");
        assert_eq!(Code::GalE6c.to_str(), "GAL E6C");
        assert_eq!(Code::GalE6x.to_str(), "GAL E6");
        assert_eq!(Code::GalE7i.to_str(), "GAL E5bI");
        assert_eq!(Code::GalE7q.to_str(), "GAL E5bQ");
        assert_eq!(Code::GalE7x.to_str(), "GAL E5b");
        assert_eq!(Code::GalE8i.to_str(), "GAL E8I");
        assert_eq!(Code::GalE8q.to_str(), "GAL E8Q");
        assert_eq!(Code::GalE8x.to_str(), "GAL E8");
        assert_eq!(Code::GalE5i.to_str(), "GAL E5aI");
        assert_eq!(Code::GalE5q.to_str(), "GAL E5aQ");
        assert_eq!(Code::GalE5x.to_str(), "GAL E5a");
        assert_eq!(Code::GloL1p.to_str(), "GLO L1P");
        assert_eq!(Code::GloL2p.to_str(), "GLO L2P");
        assert_eq!(Code::QzsL1ca.to_str(), "QZS L1CA");
        assert_eq!(Code::QzsL1ci.to_str(), "QZS L1CI");
        assert_eq!(Code::QzsL1cq.to_str(), "QZS L1CQ");
        assert_eq!(Code::QzsL1cx.to_str(), "QZS L1CX");
        assert_eq!(Code::QzsL2cm.to_str(), "QZS L2CM");
        assert_eq!(Code::QzsL2cl.to_str(), "QZS L2CL");
        assert_eq!(Code::QzsL2cx.to_str(), "QZS L2C");
        assert_eq!(Code::QzsL5i.to_str(), "QZS L5I");
        assert_eq!(Code::QzsL5q.to_str(), "QZS L5Q");
        assert_eq!(Code::QzsL5x.to_str(), "QZS L5");
        assert_eq!(Code::SbasL5i.to_str(), "SBAS L5I");
        assert_eq!(Code::SbasL5q.to_str(), "SBAS L5Q");
        assert_eq!(Code::SbasL5x.to_str(), "SBAS L5");
        assert_eq!(Code::Bds3B1ci.to_str(), "BDS3 B1CI");
        assert_eq!(Code::Bds3B1cq.to_str(), "BDS3 B1CQ");
        assert_eq!(Code::Bds3B1cx.to_str(), "BDS3 B1C");
        assert_eq!(Code::Bds3B5i.to_str(), "BDS3 B5I");
        assert_eq!(Code::Bds3B5q.to_str(), "BDS3 B5Q");
        assert_eq!(Code::Bds3B5x.to_str(), "BDS3 B5");
        assert_eq!(Code::Bds3B7i.to_str(), "BDS3 B7I");
        assert_eq!(Code::Bds3B7q.to_str(), "BDS3 B7Q");
        assert_eq!(Code::Bds3B7x.to_str(), "BDS3 B7");
        assert_eq!(Code::Bds3B3i.to_str(), "BDS3 B3I");
        assert_eq!(Code::Bds3B3q.to_str(), "BDS3 B3Q");
        assert_eq!(Code::Bds3B3x.to_str(), "BDS3 B3");
        assert_eq!(Code::GpsL1ci.to_str(), "GPS L1CI");
        assert_eq!(Code::GpsL1cq.to_str(), "GPS L1CQ");
        assert_eq!(Code::GpsL1cx.to_str(), "GPS L1C");
        assert_eq!(Code::AuxGps.to_str(), "GPS AUX");
        assert_eq!(Code::AuxSbas.to_str(), "SBAS AUX");
        assert_eq!(Code::AuxGal.to_str(), "GAL AUX");
        assert_eq!(Code::AuxQzs.to_str(), "QZS AUX");
        assert_eq!(Code::AuxBds.to_str(), "BDS AUX");

        assert_eq!(Code::from_str("GPS L1CA").unwrap(), Code::GpsL1ca);
        assert_eq!(Code::from_str("GPS L2CM").unwrap(), Code::GpsL2cm);
        assert_eq!(Code::from_str("SBAS L1").unwrap(), Code::SbasL1ca);
        assert_eq!(Code::from_str("GLO L1OF").unwrap(), Code::GloL1of);
        assert_eq!(Code::from_str("GLO L2OF").unwrap(), Code::GloL2of);
        assert_eq!(Code::from_str("GPS L1P").unwrap(), Code::GpsL1p);
        assert_eq!(Code::from_str("GPS L2P").unwrap(), Code::GpsL2p);
        assert_eq!(Code::from_str("GPS L2CL").unwrap(), Code::GpsL2cl);
        assert_eq!(Code::from_str("GPS L2C").unwrap(), Code::GpsL2cx);
        assert_eq!(Code::from_str("GPS L5I").unwrap(), Code::GpsL5i);
        assert_eq!(Code::from_str("GPS L5Q").unwrap(), Code::GpsL5q);
        assert_eq!(Code::from_str("GPS L5").unwrap(), Code::GpsL5x);
        assert_eq!(Code::from_str("BDS B1").unwrap(), Code::Bds2B1);
        assert_eq!(Code::from_str("BDS B2").unwrap(), Code::Bds2B2);
        assert_eq!(Code::from_str("GAL E1B").unwrap(), Code::GalE1b);
        assert_eq!(Code::from_str("GAL E1C").unwrap(), Code::GalE1c);
        assert_eq!(Code::from_str("GAL E1").unwrap(), Code::GalE1x);
        assert_eq!(Code::from_str("GAL E6B").unwrap(), Code::GalE6b);
        assert_eq!(Code::from_str("GAL E6C").unwrap(), Code::GalE6c);
        assert_eq!(Code::from_str("GAL E6").unwrap(), Code::GalE6x);
        assert_eq!(Code::from_str("GAL E5bI").unwrap(), Code::GalE7i);
        assert_eq!(Code::from_str("GAL E5bQ").unwrap(), Code::GalE7q);
        assert_eq!(Code::from_str("GAL E5b").unwrap(), Code::GalE7x);
        assert_eq!(Code::from_str("GAL E8I").unwrap(), Code::GalE8i);
        assert_eq!(Code::from_str("GAL E8Q").unwrap(), Code::GalE8q);
        assert_eq!(Code::from_str("GAL E8").unwrap(), Code::GalE8x);
        assert_eq!(Code::from_str("GAL E5aI").unwrap(), Code::GalE5i);
        assert_eq!(Code::from_str("GAL E5aQ").unwrap(), Code::GalE5q);
        assert_eq!(Code::from_str("GAL E5a").unwrap(), Code::GalE5x);
        assert_eq!(Code::from_str("GLO L1P").unwrap(), Code::GloL1p);
        assert_eq!(Code::from_str("GLO L2P").unwrap(), Code::GloL2p);
        assert_eq!(Code::from_str("QZS L1CA").unwrap(), Code::QzsL1ca);
        assert_eq!(Code::from_str("QZS L1CI").unwrap(), Code::QzsL1ci);
        assert_eq!(Code::from_str("QZS L1CQ").unwrap(), Code::QzsL1cq);
        assert_eq!(Code::from_str("QZS L1CX").unwrap(), Code::QzsL1cx);
        assert_eq!(Code::from_str("QZS L2CM").unwrap(), Code::QzsL2cm);
        assert_eq!(Code::from_str("QZS L2CL").unwrap(), Code::QzsL2cl);
        assert_eq!(Code::from_str("QZS L2C").unwrap(), Code::QzsL2cx);
        assert_eq!(Code::from_str("QZS L5I").unwrap(), Code::QzsL5i);
        assert_eq!(Code::from_str("QZS L5Q").unwrap(), Code::QzsL5q);
        assert_eq!(Code::from_str("QZS L5").unwrap(), Code::QzsL5x);
        assert_eq!(Code::from_str("SBAS L5I").unwrap(), Code::SbasL5i);
        assert_eq!(Code::from_str("SBAS L5Q").unwrap(), Code::SbasL5q);
        assert_eq!(Code::from_str("SBAS L5").unwrap(), Code::SbasL5x);
        assert_eq!(Code::from_str("BDS3 B1CI").unwrap(), Code::Bds3B1ci);
        assert_eq!(Code::from_str("BDS3 B1CQ").unwrap(), Code::Bds3B1cq);
        assert_eq!(Code::from_str("BDS3 B1C").unwrap(), Code::Bds3B1cx);
        assert_eq!(Code::from_str("BDS3 B5I").unwrap(), Code::Bds3B5i);
        assert_eq!(Code::from_str("BDS3 B5Q").unwrap(), Code::Bds3B5q);
        assert_eq!(Code::from_str("BDS3 B5").unwrap(), Code::Bds3B5x);
        assert_eq!(Code::from_str("BDS3 B7I").unwrap(), Code::Bds3B7i);
        assert_eq!(Code::from_str("BDS3 B7Q").unwrap(), Code::Bds3B7q);
        assert_eq!(Code::from_str("BDS3 B7").unwrap(), Code::Bds3B7x);
        assert_eq!(Code::from_str("BDS3 B3I").unwrap(), Code::Bds3B3i);
        assert_eq!(Code::from_str("BDS3 B3Q").unwrap(), Code::Bds3B3q);
        assert_eq!(Code::from_str("BDS3 B3").unwrap(), Code::Bds3B3x);
        assert_eq!(Code::from_str("GPS L1CI").unwrap(), Code::GpsL1ci);
        assert_eq!(Code::from_str("GPS L1CQ").unwrap(), Code::GpsL1cq);
        assert_eq!(Code::from_str("GPS L1C").unwrap(), Code::GpsL1cx);
        assert_eq!(Code::from_str("GPS AUX").unwrap(), Code::AuxGps);
        assert_eq!(Code::from_str("SBAS AUX").unwrap(), Code::AuxSbas);
        assert_eq!(Code::from_str("GAL AUX").unwrap(), Code::AuxGal);
        assert_eq!(Code::from_str("QZS AUX").unwrap(), Code::AuxQzs);
        assert_eq!(Code::from_str("BDS AUX").unwrap(), Code::AuxBds);

        {
            let result = Code::from_str("Bad String");
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), InvalidCode(-1));
        }
        {
            let result = Code::from_str("Nul\0String");
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), InvalidCode(-1));
        }
        {
            let result = Code::from_str("💩💩💩💩");
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), InvalidCode(-1));
        }
    }

    #[test]
    fn signal_strings() {
        assert_eq!(
            GnssSignal::new(1, Code::GpsL1ca).unwrap().to_str(),
            "GPS L1CA 1"
        );
        assert_eq!(
            GnssSignal::new(32, Code::GpsL1ca).unwrap().to_str(),
            "GPS L1CA 32"
        );
        assert_eq!(
            GnssSignal::new(1, Code::GalE5x).unwrap().to_str(),
            "GAL E5a 1"
        );
        assert_eq!(
            GnssSignal::new(32, Code::GalE5x).unwrap().to_str(),
            "GAL E5a 32"
        );
        assert_eq!(
            GnssSignal::new(1, Code::Bds2B1).unwrap().to_str(),
            "BDS B1 1"
        );
        assert_eq!(
            GnssSignal::new(32, Code::Bds2B1).unwrap().to_str(),
            "BDS B1 32"
        );
    }
}
