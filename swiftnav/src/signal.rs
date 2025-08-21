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
use std::fmt;

pub mod consts {
    use crate::math::compile_time_max_u16;

    /// Number of satellites in the GPS constellation.
    pub const NUM_SATS_GPS: u16 = 32;
    /// Number of satellites in the SBAS constellation.
    pub const NUM_SATS_SBAS: u16 = 19;
    /// Number of satellites in the GLONASS constellation.
    /// refer to <https://igscb.jpl.nasa.gov/pipermail/igsmail/2012/007771.html> and
    /// <https://igscb.jpl.nasa.gov/pipermail/igsmail/2015/008391.html>
    pub const NUM_SATS_GLO: u16 = 28;
    /// Number of satellites in the BeiDou constellation.
    pub const NUM_SATS_BDS: u16 = 64;
    /// Number of satellites in the Galileo constellation.
    pub const NUM_SATS_GAL: u16 = 36;
    /// Number of satellites in the QZSS constellation.
    pub const NUM_SATS_QZS: u16 = 10;

    pub const NUM_SATS: u16 =
        NUM_SATS_GPS + NUM_SATS_SBAS + NUM_SATS_GLO + NUM_SATS_BDS + NUM_SATS_QZS + NUM_SATS_GAL;

    pub const MAX_NUM_SATS: u16 = compile_time_max_u16(
        NUM_SATS_GPS,
        compile_time_max_u16(
            NUM_SATS_SBAS,
            compile_time_max_u16(
                NUM_SATS_GLO,
                compile_time_max_u16(NUM_SATS_BDS, compile_time_max_u16(NUM_SATS_QZS, NUM_SATS_GAL)),
            ),
        ),
    );

    /* Number of codes in each constellation. */
    pub const NUM_CODES_GPS: u16 = 13;
    pub const NUM_CODES_SBAS: u16 = 5;
    pub const NUM_CODES_GLO: u16 = 4;
    pub const NUM_CODES_BDS: u16 = 15;
    pub const NUM_CODES_QZS: u16 = 11;
    pub const NUM_CODES_GAL: u16 = 16;

    pub const NUM_CODES: u16 =
        NUM_CODES_GPS + NUM_CODES_SBAS + NUM_CODES_GLO + NUM_CODES_BDS + NUM_CODES_GAL + NUM_CODES_QZS;

    /// Max number of GLO frequency slot, correspond to frequency slot 6
    pub const GLO_MAX_FCN: u16 = 14;

    // /// Min number of GLO frequency slot, correspond to frequency slot -7
    // const GLO_MIN_FCN: u16 = 1;

    // /// Frequency of GLO channel is unknown */
    // const GLO_FCN_UNKNOWN: u16 = 0;

    // /// Used to produce an unshifted GLO frequency slot out of GLO slots in
    // /// GLO_MIN_FCN .. GLO_MAX_FCN range
    // const GLO_FCN_OFFSET: u16 = 8;

    // /// GLO Orbital slot is unknown
    // const GLO_ORBIT_SLOT_UNKNOWN: u16 = 0;

    /* Number of signals in each code. */
    pub const NUM_SIGNALS_GPS_L1CA: u16 = NUM_SATS_GPS;
    pub const NUM_SIGNALS_GPS_L2C: u16 = NUM_SATS_GPS;
    pub const NUM_SIGNALS_GPS_L5: u16 = NUM_SATS_GPS;
    pub const NUM_SIGNALS_GPS_L1P: u16 = NUM_SATS_GPS;
    pub const NUM_SIGNALS_GPS_L2P: u16 = NUM_SATS_GPS;
    pub const NUM_SIGNALS_GPS_L1C: u16 = NUM_SATS_GPS;

    pub const NUM_SIGNALS_SBAS_L1CA: u16 = NUM_SATS_SBAS;
    pub const NUM_SIGNALS_SBAS_L5: u16 = NUM_SATS_SBAS;

    pub const NUM_SIGNALS_GLO_L1OF: u16 = NUM_SATS_GLO;
    pub const NUM_SIGNALS_GLO_L2OF: u16 = NUM_SATS_GLO;
    pub const NUM_SIGNALS_GLO_L1P: u16 = NUM_SATS_GLO;
    pub const NUM_SIGNALS_GLO_L2P: u16 = NUM_SATS_GLO;

    pub const NUM_SIGNALS_BDS2_B1: u16 = NUM_SATS_BDS;
    pub const NUM_SIGNALS_BDS2_B2: u16 = NUM_SATS_BDS;
    pub const NUM_SIGNALS_BDS3_B1C: u16 = NUM_SATS_BDS;
    pub const NUM_SIGNALS_BDS3_B5: u16 = NUM_SATS_BDS;
    pub const NUM_SIGNALS_BDS3_B7: u16 = NUM_SATS_BDS;
    pub const NUM_SIGNALS_BDS3_B3: u16 = NUM_SATS_BDS;

    pub const NUM_SIGNALS_GAL_E1: u16 = NUM_SATS_GAL;
    pub const NUM_SIGNALS_GAL_E6: u16 = NUM_SATS_GAL;
    pub const NUM_SIGNALS_GAL_E7: u16 = NUM_SATS_GAL;
    pub const NUM_SIGNALS_GAL_E8: u16 = NUM_SATS_GAL;
    pub const NUM_SIGNALS_GAL_E5: u16 = NUM_SATS_GAL;

    pub const NUM_SIGNALS_QZS_L1: u16 = NUM_SATS_QZS;
    pub const NUM_SIGNALS_QZS_L1C: u16 = NUM_SATS_QZS;
    pub const NUM_SIGNALS_QZS_L2C: u16 = NUM_SATS_QZS;
    pub const NUM_SIGNALS_QZS_L5: u16 = NUM_SATS_QZS;

    /* Number of frequencies in GLO. */
    pub const NUM_FREQ_GLO_L1OF: u16 = GLO_MAX_FCN;
    pub const NUM_FREQ_GLO_L2OF: u16 = GLO_MAX_FCN;

    /* Number of signals in each constellation. */
    pub const NUM_SIGNALS_GPS: u16 = 2 * NUM_SIGNALS_GPS_L1CA
        + 3 * NUM_SIGNALS_GPS_L2C
        + NUM_SIGNALS_GPS_L1P
        + NUM_SIGNALS_GPS_L2P
        + 3 * NUM_SIGNALS_GPS_L5
        + 3 * NUM_SIGNALS_GPS_L1C;
    pub const NUM_SIGNALS_SBAS: u16 = 2 * NUM_SIGNALS_SBAS_L1CA + 3 * NUM_SIGNALS_SBAS_L5;
    pub const NUM_SIGNALS_GLO: u16 =
        NUM_SIGNALS_GLO_L1OF + NUM_SIGNALS_GLO_L2OF + NUM_SIGNALS_GLO_L1P + NUM_SIGNALS_GLO_L2P;
    pub const NUM_SIGNALS_BDS: u16 = 2 * NUM_SIGNALS_BDS2_B1
        + NUM_SIGNALS_BDS2_B2
        + 3 * NUM_SIGNALS_BDS3_B1C
        + 3 * NUM_SIGNALS_BDS3_B5
        + 3 * NUM_SIGNALS_BDS3_B7
        + 3 * NUM_SIGNALS_BDS3_B3;
    pub const NUM_SIGNALS_GAL: u16 = 4 * NUM_SIGNALS_GAL_E1
        + 3 * NUM_SIGNALS_GAL_E6
        + 3 * NUM_SIGNALS_GAL_E7
        + 3 * NUM_SIGNALS_GAL_E8
        + 3 * NUM_SIGNALS_GAL_E5;
    pub const NUM_SIGNALS_QZS: u16 = 2 * NUM_SIGNALS_QZS_L1
        + 3 * NUM_SIGNALS_QZS_L1C
        + 3 * NUM_SIGNALS_QZS_L2C
        + 3 * NUM_SIGNALS_QZS_L5;
    pub const NUM_SIGNALS: u16 = NUM_SIGNALS_GPS
        + NUM_SIGNALS_SBAS
        + NUM_SIGNALS_GLO
        + NUM_SIGNALS_BDS
        + NUM_SIGNALS_GAL
        + NUM_SIGNALS_QZS;

    pub const GPS_FIRST_PRN: u16 = 1;
    pub const SBAS_FIRST_PRN: u16 = 120;
    pub const GLO_FIRST_PRN: u16 = 1;
    pub const BDS_FIRST_PRN: u16 = 1;
    pub const GAL_FIRST_PRN: u16 = 1;
    pub const QZS_FIRST_PRN: u16 = 193;
}

/// GNSS satellite constellations
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, strum::Display, strum::EnumString, strum::IntoStaticStr)]
#[strum(serialize_all = "UPPERCASE")]
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

impl Constellation {
    /// Gets the specified maximum number of active satellites for the constellation
    pub fn sat_count(&self) -> u16 {
        match &self {
            Constellation::Gps => consts::NUM_SATS_GPS,
            Constellation::Sbas => consts::NUM_SATS_SBAS,
            Constellation::Glo => consts::NUM_SATS_GLO,
            Constellation::Bds => consts::NUM_SATS_BDS,
            Constellation::Gal => consts::NUM_SATS_GAL,
            Constellation::Qzs => consts::NUM_SATS_QZS,
        }
    }

    pub fn first_prn(&self) -> u16 {
        match &self {
            Constellation::Gps => consts::GPS_FIRST_PRN,
            Constellation::Sbas => consts::SBAS_FIRST_PRN,
            Constellation::Glo => consts::GLO_FIRST_PRN,
            Constellation::Bds => consts::BDS_FIRST_PRN,
            Constellation::Gal => consts::GAL_FIRST_PRN,
            Constellation::Qzs => consts::QZS_FIRST_PRN,
        }
    }

    /// Get the human readable name of the constellation.
    pub fn to_str(&self) -> Cow<'static, str> {
        let s: &'static str = self.into();
        s.into()
    }
}

/// An error encountered when converting an integer into a [`Constellation`]
/// and no constellation is associated with the given value
#[derive(thiserror::Error, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[error("Invalid integer for GNSS Constellation ({0})")]
pub struct InvalidConstellationInt(u8);

impl std::convert::TryFrom<u8> for Constellation {
    type Error = InvalidConstellationInt;
    fn try_from(value: u8) -> Result<Constellation, Self::Error> {
        match value {
            0 => Ok(Constellation::Gps),
            1 => Ok(Constellation::Sbas),
            2 => Ok(Constellation::Glo),
            3 => Ok(Constellation::Bds),
            4 => Ok(Constellation::Qzs),
            5 => Ok(Constellation::Gal),
            _ => Err(InvalidConstellationInt(value)),
        }
    }
}

/// Code identifiers
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, strum::Display, strum::EnumString, strum::IntoStaticStr)]
pub enum Code {
    #[strum(to_string = "GPS L1CA")]
    /// GPS L1CA: BPSK(1)
    GpsL1ca,
    #[strum(to_string = "GPS L2CM")]
    /// GPS L2C: 2 x BPSK(0.5)
    GpsL2cm,
    #[strum(to_string = "SBAS L1")]
    /// SBAS L1: BPSK(1)
    SbasL1ca,
    #[strum(to_string = "GLO L1OF")]
    /// GLONASS L1OF: FDMA BPSK(0.5)
    GloL1of,
    #[strum(to_string = "GLO L2OF")]
    /// GLONASS L2OF: FDMA BPSK(0.5)
    GloL2of,
    #[strum(to_string = "GPS L1P")]
    /// GPS L1P(Y): encrypted BPSK(10)
    GpsL1p,
    #[strum(to_string = "GPS L2P")]
    /// GPS L2P(Y): encrypted BPSK(10)
    GpsL2p,
    #[strum(to_string = "GPS L2CL")]
    GpsL2cl,
    #[strum(to_string = "GPS L2C")]
    GpsL2cx,
    #[strum(to_string = "GPS L5I")]
    /// GPS L5: QPSK(10) at 1150*f0
    GpsL5i,
    #[strum(to_string = "GPS L5Q")]
    GpsL5q,
    #[strum(to_string = "GPS L5")]
    GpsL5x,
    #[strum(to_string = "BDS B1")]
    /// BDS2 B1I: BPSK(2) at 1526*f0
    Bds2B1,
    #[strum(to_string = "BDS B2")]
    /// BDS2 B2I: BPSK(2) at 1180*f0
    Bds2B2,
    #[strum(to_string = "GAL E1B")]
    /// Galileo E1: CASM CBOC(1,1) at 1540*f0
    GalE1b,
    #[strum(to_string = "GAL E1C")]
    GalE1c,
    #[strum(to_string = "GAL E1")]
    GalE1x,
    #[strum(to_string = "GAL E6B")]
    /// Galileo E6: CASM BPSK(5) at 1250*f0
    GalE6b,
    #[strum(to_string = "GAL E6C")]
    GalE6c,
    #[strum(to_string = "GAL E6")]
    GalE6x,
    #[strum(to_string = "GAL E5bI")]
    /// Galileo E5b: QPSK(10) at 1180*f0
    GalE7i,
    #[strum(to_string = "GAL E5bQ")]
    GalE7q,
    #[strum(to_string = "GAL E5b")]
    GalE7x,
    #[strum(to_string = "GAL E8I")]
    /// Galileo E5AltBOC(15,10) at 1165*f0
    GalE8i,
    #[strum(to_string = "GAL E8Q")]
    GalE8q,
    #[strum(to_string = "GAL E8")]
    GalE8x,
    #[strum(to_string = "GAL E5aI")]
    /// Galileo E5a: QPSK(10) at 1150*f0
    GalE5i,
    #[strum(to_string = "GAL E5aQ")]
    GalE5q,
    #[strum(to_string = "GAL E5a")]
    GalE5x,
    #[strum(to_string = "GLO L1P")]
    /// GLONASS L1P: encrypted
    GloL1p,
    #[strum(to_string = "GLO L2P")]
    /// GLONASS L2P: encrypted
    GloL2p,
    #[strum(to_string = "QZS L1CA")]
    /// QZSS L1CA: BPSK(1) at 1540*f0
    QzsL1ca,
    #[strum(to_string = "QZS L1CI")]
    /// QZSS L1C: TM-BOC at 1540*f0
    QzsL1ci,
    #[strum(to_string = "QZS L1CQ")]
    QzsL1cq,
    #[strum(to_string = "QZS L1CX")]
    QzsL1cx,
    #[strum(to_string = "QZS L2CM")]
    /// QZSS L2C: 2 x BPSK(0.5) at 1200*f0
    QzsL2cm,
    #[strum(to_string = "QZS L2CL")]
    QzsL2cl,
    #[strum(to_string = "QZS L2C")]
    QzsL2cx,
    #[strum(to_string = "QZS L5I")]
    /// QZSS L5: QPSK(10) at 1150*f0
    QzsL5i,
    #[strum(to_string = "QZS L5Q")]
    QzsL5q,
    #[strum(to_string = "QZS L5")]
    QzsL5x,
    #[strum(to_string = "SBAS L5I")]
    /// SBAS L5: ? at 1150*f0
    SbasL5i,
    #[strum(to_string = "SBAS L5Q")]
    SbasL5q,
    #[strum(to_string = "SBAS L5")]
    SbasL5x,
    #[strum(to_string = "BDS3 B1CI")]
    /// BDS3 B1C: TM-BOC at 1540*f0
    Bds3B1ci,
    #[strum(to_string = "BDS3 B1CQ")]
    Bds3B1cq,
    #[strum(to_string = "BDS3 B1C")]
    Bds3B1cx,
    #[strum(to_string = "BDS3 B5I")]
    /// BDS3 B2a: QPSK(10) at 1150*f0
    Bds3B5i,
    #[strum(to_string = "BDS3 B5Q")]
    Bds3B5q,
    #[strum(to_string = "BDS3 B5")]
    Bds3B5x,
    #[strum(to_string = "BDS3 B7I")]
    /// BDS3 B2b: QPSK(10) at 1180*f0
    Bds3B7i,
    #[strum(to_string = "BDS3 B7Q")]
    Bds3B7q,
    #[strum(to_string = "BDS3 B7")]
    Bds3B7x,
    #[strum(to_string = "BDS3 B3I")]
    /// BDS3 B3I: QPSK(10) at 1240*f0
    Bds3B3i,
    #[strum(to_string = "BDS3 B3Q")]
    Bds3B3q,
    #[strum(to_string = "BDS3 B3")]
    Bds3B3x,
    #[strum(to_string = "GPS L1CI")]
    /// GPS L1C: TM-BOC at 1540*f0
    GpsL1ci,
    #[strum(to_string = "GPS L1CQ")]
    GpsL1cq,
    #[strum(to_string = "GPS L1C")]
    GpsL1cx,
    #[strum(to_string = "GPS AUX")]
    /// Auxiliary GPS antenna signals
    AuxGps,
    #[strum(to_string = "SBAS AUX")]
    /// Auxiliary SBAS antenna signals
    AuxSbas,
    #[strum(to_string = "GAL AUX")]
    /// Auxiliary GAL antenna signals
    AuxGal,
    #[strum(to_string = "QZS AUX")]
    /// Auxiliary QZSS antenna signals
    AuxQzs,
    #[strum(to_string = "BDS AUX")]
    /// Auxiliary BDS antenna signals
    AuxBds,
}

impl Code {
    /// Get the human readable name of the code.
    pub fn to_str(&self) -> Cow<'static, str> {
        let s: &'static str = self.into();
        s.into()
    }

    /// Gets the corresponding [`Constellation`]
    pub fn to_constellation(&self) -> Constellation {
        match &self {
            Code::GpsL1ca
            | Code::GpsL2cm
            | Code::GpsL1p
            | Code::GpsL2p
            | Code::GpsL2cl
            | Code::GpsL2cx
            | Code::GpsL5i
            | Code::GpsL5q
            | Code::GpsL5x
            | Code::GpsL1ci
            | Code::GpsL1cq
            | Code::GpsL1cx
            | Code::AuxGps => Constellation::Gps,
            Code::SbasL1ca | Code::SbasL5i | Code::SbasL5q | Code::SbasL5x | Code::AuxSbas => {
                Constellation::Sbas
            }
            Code::GloL1of | Code::GloL2of | Code::GloL1p | Code::GloL2p => Constellation::Glo,
            Code::Bds2B1
            | Code::Bds2B2
            | Code::Bds3B1ci
            | Code::Bds3B1cq
            | Code::Bds3B1cx
            | Code::Bds3B5i
            | Code::Bds3B5q
            | Code::Bds3B5x
            | Code::Bds3B7i
            | Code::Bds3B7q
            | Code::Bds3B7x
            | Code::Bds3B3i
            | Code::Bds3B3q
            | Code::Bds3B3x
            | Code::AuxBds => Constellation::Bds,
            Code::GalE1b
            | Code::GalE1c
            | Code::GalE1x
            | Code::GalE6b
            | Code::GalE6c
            | Code::GalE6x
            | Code::GalE7i
            | Code::GalE7q
            | Code::GalE7x
            | Code::GalE8i
            | Code::GalE8q
            | Code::GalE8x
            | Code::GalE5i
            | Code::GalE5q
            | Code::GalE5x
            | Code::AuxGal => Constellation::Gal,
            Code::QzsL1ca
            | Code::QzsL1ci
            | Code::QzsL1cq
            | Code::QzsL1cx
            | Code::QzsL2cm
            | Code::QzsL2cl
            | Code::QzsL2cx
            | Code::QzsL5i
            | Code::QzsL5q
            | Code::QzsL5x
            | Code::AuxQzs => Constellation::Qzs,
        }
    }

    /// Checks if this ia a GPS code
    pub fn is_gps(&self) -> bool {
        self.to_constellation() == Constellation::Gps
    }

    pub fn is_sbas(&self) -> bool {
        self.to_constellation() == Constellation::Sbas
    }

    pub fn is_glo(&self) -> bool {
        self.to_constellation() == Constellation::Glo
    }

    pub fn is_bds2(&self) -> bool {
        self.to_constellation() == Constellation::Bds
    }

    pub fn is_gal(&self) -> bool {
        self.to_constellation() == Constellation::Gal
    }

    pub fn is_qzss(&self) -> bool {
        self.to_constellation() == Constellation::Qzs
    }
}

/// An error encountered when converting an integer into a [`Code`]
/// and no code is associated with the given value
#[derive(thiserror::Error, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[error("Invalid integer for GNSS Code ({0})")]
pub struct InvalidCodeInt(u8);

impl std::convert::TryFrom<u8> for Code {
    type Error = InvalidCodeInt;
    fn try_from(value: u8) -> Result<Code, Self::Error> {
        match value {
            0 => Ok(Code::GpsL1ca),
            1 => Ok(Code::GpsL2cm),
            2 => Ok(Code::SbasL1ca),
            3 => Ok(Code::GloL1of),
            4 => Ok(Code::GloL2of),
            5 => Ok(Code::GpsL1p),
            6 => Ok(Code::GpsL2p),
            7 => Ok(Code::GpsL2cl),
            8 => Ok(Code::GpsL2cx),
            9 => Ok(Code::GpsL5i),
            10 => Ok(Code::GpsL5q),
            11 => Ok(Code::GpsL5x),
            12 => Ok(Code::Bds2B1),
            13 => Ok(Code::Bds2B2),
            14 => Ok(Code::GalE1b),
            15 => Ok(Code::GalE1c),
            16 => Ok(Code::GalE1x),
            17 => Ok(Code::GalE6b),
            18 => Ok(Code::GalE6c),
            19 => Ok(Code::GalE6x),
            20 => Ok(Code::GalE7i),
            21 => Ok(Code::GalE7q),
            22 => Ok(Code::GalE7x),
            23 => Ok(Code::GalE8i),
            24 => Ok(Code::GalE8q),
            25 => Ok(Code::GalE8x),
            26 => Ok(Code::GalE5i),
            27 => Ok(Code::GalE5q),
            28 => Ok(Code::GalE5x),
            29 => Ok(Code::GloL1p),
            30 => Ok(Code::GloL2p),
            31 => Ok(Code::QzsL1ca),
            32 => Ok(Code::QzsL1ci),
            33 => Ok(Code::QzsL1cq),
            34 => Ok(Code::QzsL1cx),
            35 => Ok(Code::QzsL2cm),
            36 => Ok(Code::QzsL2cl),
            37 => Ok(Code::QzsL2cx),
            38 => Ok(Code::QzsL5i),
            39 => Ok(Code::QzsL5q),
            40 => Ok(Code::QzsL5x),
            41 => Ok(Code::SbasL5i),
            42 => Ok(Code::SbasL5q),
            43 => Ok(Code::SbasL5x),
            44 => Ok(Code::Bds3B1ci),
            45 => Ok(Code::Bds3B1cq),
            46 => Ok(Code::Bds3B1cx),
            47 => Ok(Code::Bds3B5i),
            48 => Ok(Code::Bds3B5q),
            49 => Ok(Code::Bds3B5x),
            50 => Ok(Code::Bds3B7i),
            51 => Ok(Code::Bds3B7q),
            52 => Ok(Code::Bds3B7x),
            53 => Ok(Code::Bds3B3i),
            54 => Ok(Code::Bds3B3q),
            55 => Ok(Code::Bds3B3x),
            56 => Ok(Code::GpsL1ci),
            57 => Ok(Code::GpsL1cq),
            58 => Ok(Code::GpsL1cx),
            59 => Ok(Code::AuxGps),
            60 => Ok(Code::AuxSbas),
            61 => Ok(Code::AuxGal),
            62 => Ok(Code::AuxQzs),
            63 => Ok(Code::AuxBds),
            _ => Err(InvalidCodeInt(value))
        }
    }
}

/// GNSS Signal identifier
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct GnssSignal{
    code: Code,
    sat: u16,
}

/// An error encountered when converting an integer into a [`GnssSignal`]
/// and satellite number is not in the valid range for the associated constellation
#[derive(thiserror::Error, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[error("The satellite number is not valid for the associated constellation ({0})")]
pub struct InvalidSatellite(u16);

impl GnssSignal {
    pub fn new(sat: u16, code: Code) -> Result<GnssSignal, InvalidSatellite> {
        let constellation = code.to_constellation();
        if sat < constellation.first_prn() || sat >= (constellation.first_prn() + constellation.sat_count()) {
            Err(InvalidSatellite(sat))
        } else {
            Ok(GnssSignal{code, sat})
        }
    }

    /// Get the satellite PRN of the signal
    pub fn sat(&self) -> u16 {
        self.sat
    }

    /// Get the [`Code`] of the signal
    pub fn code(&self) -> Code {
        self.code
    }

    /// Get the [`Constellation`] of the signal
    pub fn to_constellation(&self) -> Constellation {
        self.code.to_constellation()
    }

    /// Makes the human readable signal name
    pub fn to_str(&self) -> String {
        format!("{} {}", self.code.to_str(), self.sat)
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
        let first = consts::GPS_FIRST_PRN;
        let last = consts::GPS_FIRST_PRN + consts::NUM_SATS_GPS;
        for sat in (first - 1)..(last + 2) {
            let result = GnssSignal::new(sat, Code::GpsL1ca);
            if sat < first || sat >= last {
                assert!(result.is_err());
                assert_eq!(
                    result.unwrap_err(),
                    InvalidSatellite(sat)
                );
            } else {
                assert!(result.is_ok());
            }
        }

        let first = consts::SBAS_FIRST_PRN;
        let last = consts::SBAS_FIRST_PRN + consts::NUM_SATS_SBAS;
        for sat in (first - 1)..(last + 2) {
            let result = GnssSignal::new(sat, Code::SbasL1ca);
            if sat < first || sat >= last {
                assert!(result.is_err());
                assert_eq!(
                    result.unwrap_err(),
                    InvalidSatellite(sat)
                );
            } else {
                assert!(result.is_ok());
            }
        }

        let first = consts::GLO_FIRST_PRN;
        let last = consts::GLO_FIRST_PRN + consts::NUM_SATS_GLO;
        for sat in (first - 1)..(last + 2) {
            let result = GnssSignal::new(sat, Code::GloL1of);
            if sat < first || sat >= last {
                assert!(result.is_err());
                assert_eq!(
                    result.unwrap_err(),
                    InvalidSatellite(sat)
                );
            } else {
                assert!(result.is_ok());
            }
        }

        let first = consts::BDS_FIRST_PRN;
        let last = consts::BDS_FIRST_PRN + consts::NUM_SATS_BDS;
        for sat in (first - 1)..(last + 2) {
            let result = GnssSignal::new(sat, Code::Bds2B1);
            if sat < first || sat >= last {
                assert!(result.is_err());
                assert_eq!(
                    result.unwrap_err(),
                    InvalidSatellite(sat)
                );
            } else {
                assert!(result.is_ok());
            }
        }

        let first = consts::GAL_FIRST_PRN;
        let last = consts::GAL_FIRST_PRN + consts::NUM_SATS_GAL;
        for sat in (first - 1)..(last + 2) {
            let result = GnssSignal::new(sat, Code::GalE1b);
            if sat < first || sat >= last {
                assert!(result.is_err());
                assert_eq!(
                    result.unwrap_err(),
                    InvalidSatellite(sat)
                );
            } else {
                assert!(result.is_ok());
            }
        }

        let first = consts::QZS_FIRST_PRN;
        let last = consts::QZS_FIRST_PRN + consts::NUM_SATS_QZS;
        for sat in (first - 1)..(last + 2) {
            let result = GnssSignal::new(sat, Code::QzsL1ca);
            if sat < first || sat >= last {
                assert!(result.is_err());
                assert_eq!(
                    result.unwrap_err(),
                    InvalidSatellite(sat)
                );
            } else {
                assert!(result.is_ok());
            }
        }
    }

    #[test]
    fn constellation_strings() {
        use std::str::FromStr;

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
        }
        {
            let result = Constellation::from_str("Nul\0String");
            assert!(result.is_err());
        }
        {
            let result = Constellation::from_str("💩💩💩💩");
            assert!(result.is_err());
        }
    }

    #[test]
    fn code_strings() {
        use std::str::FromStr;

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
        }
        {
            let result = Code::from_str("Nul\0String");
            assert!(result.is_err());
        }
        {
            let result = Code::from_str("💩💩💩💩");
            assert!(result.is_err());
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
