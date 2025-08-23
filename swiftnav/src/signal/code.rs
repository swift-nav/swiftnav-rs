// Copyright (c) 2025 Swift Navigation Inc.
// Contact: Swift Navigation <dev@swiftnav.com>
//
// This source is subject to the license found in the file 'LICENSE' which must
// be be distributed together with this source. All other rights reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND,
// EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A PARTICULAR PURPOSE.

use super::{consts, Constellation};

/// Code identifiers
#[derive(
    Debug,
    Copy,
    Clone,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    Hash,
    strum::AsRefStr,
    strum::Display,
    strum::EnumIter,
    strum::EnumString,
    strum::FromRepr,
    strum::IntoStaticStr,
)]
#[repr(u8)]
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
    /// Gets the corresponding [`Constellation`]
    #[must_use]
    pub fn to_constellation(self) -> Constellation {
        match self {
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

    /// Checks if this is a GPS code
    #[must_use]
    pub fn is_gps(self) -> bool {
        self.to_constellation() == Constellation::Gps
    }

    /// Checks if this is a SBAS code
    #[must_use]
    pub fn is_sbas(self) -> bool {
        self.to_constellation() == Constellation::Sbas
    }

    /// Checks if this is a GLONASS code
    #[must_use]
    pub fn is_glo(self) -> bool {
        self.to_constellation() == Constellation::Glo
    }

    /// Checks if this is a BeiDou code
    #[must_use]
    pub fn is_bds(self) -> bool {
        self.to_constellation() == Constellation::Bds
    }

    /// Checks if this is a Galileo code
    #[must_use]
    pub fn is_gal(self) -> bool {
        self.to_constellation() == Constellation::Gal
    }

    /// Checks if this is a QZSS code
    #[must_use]
    pub fn is_qzss(self) -> bool {
        self.to_constellation() == Constellation::Qzs
    }

    #[must_use]
    pub(crate) fn to_code_t(self) -> i32 {
        self as i32
    }

    /// Get the carrier frequency of the given code
    ///
    /// # Note
    ///
    /// GLONASS FDMA codes return the center frequency. To get the channel
    /// frequency use [`Code::get_glo_channel_frequency()`] instead
    #[must_use]
    pub fn get_carrier_frequency(&self) -> f64 {
        match &self {
            Code::GpsL1ca
            | Code::GpsL1p
            | Code::GpsL1ci
            | Code::GpsL1cq
            | Code::GpsL1cx
            | Code::AuxGps => consts::GPS_L1_HZ,
            Code::GpsL2cm | Code::GpsL2p | Code::GpsL2cl | Code::GpsL2cx => consts::GPS_L2_HZ,
            Code::GpsL5i | Code::GpsL5q | Code::GpsL5x => consts::GPS_L5_HZ,

            Code::SbasL1ca | Code::AuxSbas => consts::SBAS_L1_HZ,
            Code::SbasL5i | Code::SbasL5q | Code::SbasL5x => consts::SBAS_L5_HZ,
            Code::GloL1of | Code::GloL1p => consts::GLO_L1_HZ,
            Code::GloL2of | Code::GloL2p => consts::GLO_L2_HZ,
            Code::Bds2B1 | Code::AuxBds => consts::BDS2_B1I_HZ,
            Code::Bds3B1ci | Code::Bds3B1cq | Code::Bds3B1cx => consts::BDS3_B1C_HZ,
            Code::Bds2B2 => consts::BDS2_B2_HZ,
            Code::Bds3B3i | Code::Bds3B3q | Code::Bds3B3x => consts::BDS3_B3_HZ,
            Code::Bds3B5i | Code::Bds3B5q | Code::Bds3B5x => consts::BDS3_B5_HZ,
            Code::Bds3B7i | Code::Bds3B7q | Code::Bds3B7x => consts::BDS3_B7_HZ,
            Code::GalE1b | Code::GalE1c | Code::GalE1x | Code::AuxGal => consts::GAL_E1_HZ,
            Code::GalE5i | Code::GalE5q | Code::GalE5x => consts::GAL_E5_HZ,
            Code::GalE6b | Code::GalE6c | Code::GalE6x => consts::GAL_E6_HZ,
            Code::GalE7i | Code::GalE7q | Code::GalE7x => consts::GAL_E7_HZ,
            Code::GalE8i | Code::GalE8q | Code::GalE8x => consts::GAL_E8_HZ,
            Code::QzsL1ca | Code::QzsL1ci | Code::QzsL1cq | Code::QzsL1cx | Code::AuxQzs => {
                consts::QZS_L1_HZ
            }
            Code::QzsL2cm | Code::QzsL2cl | Code::QzsL2cx => consts::QZS_L2_HZ,
            Code::QzsL5i | Code::QzsL5q | Code::QzsL5x => consts::QZS_L5_HZ,
        }
    }

    /// Get the channel frequency for the given GLONASS FDMA code and channel slot
    ///
    /// The code must be either GLO L1OF or L2OF. The slot number must be between -7 and +6
    ///
    /// # Panics
    ///
    /// This function will panic if the code is not a GLONASS FDMA code, or if the channel slot
    /// is invalid.
    #[must_use]
    pub fn get_glo_channel_frequency(&self, slot: i16) -> f64 {
        assert!(*self == Code::GloL1of || *self == Code::GloL2of);

        let fcn = slot + consts::GLO_FCN_OFFSET;
        assert!((consts::GLO_MIN_FCN..=consts::GLO_MAX_FCN).contains(&fcn));

        match &self {
            Code::GloL1of => {
                consts::GLO_L1_HZ + (fcn - consts::GLO_FCN_OFFSET) as f64 * consts::GLO_L1_DELTA_HZ
            }
            Code::GloL2of => {
                consts::GLO_L2_HZ + (fcn - consts::GLO_FCN_OFFSET) as f64 * consts::GLO_L2_DELTA_HZ
            }
            _ => panic!("You can't call get_glo_channel_frequency() on a non-GLONASS FDMA code!"),
        }
    }

    /// Get an iterator through the codes
    pub fn iter() -> CodeIter {
        <Self as strum::IntoEnumIterator>::iter()
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
        Code::from_repr(value).ok_or(InvalidCodeInt(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert!(Code::Bds2B1.is_bds());
        assert!(Code::Bds2B2.is_bds());
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
        assert!(Code::Bds3B1ci.is_bds());
        assert!(Code::Bds3B1cq.is_bds());
        assert!(Code::Bds3B1cx.is_bds());
        assert!(Code::Bds3B5i.is_bds());
        assert!(Code::Bds3B5q.is_bds());
        assert!(Code::Bds3B5x.is_bds());
        assert!(Code::Bds3B7i.is_bds());
        assert!(Code::Bds3B7q.is_bds());
        assert!(Code::Bds3B7x.is_bds());
        assert!(Code::Bds3B3i.is_bds());
        assert!(Code::Bds3B3q.is_bds());
        assert!(Code::Bds3B3x.is_bds());
        assert!(Code::GpsL1ci.is_gps());
        assert!(Code::GpsL1cq.is_gps());
        assert!(Code::GpsL1cx.is_gps());
        assert!(Code::AuxGps.is_gps());
        assert!(Code::AuxSbas.is_sbas());
        assert!(Code::AuxGal.is_gal());
        assert!(Code::AuxQzs.is_qzss());
        assert!(Code::AuxBds.is_bds());
    }

    #[test]
    fn code_strings() {
        use std::str::FromStr;

        assert_eq!(Code::GpsL1ca.to_string(), "GPS L1CA");
        assert_eq!(Code::GpsL2cm.to_string(), "GPS L2CM");
        assert_eq!(Code::SbasL1ca.to_string(), "SBAS L1");
        assert_eq!(Code::GloL1of.to_string(), "GLO L1OF");
        assert_eq!(Code::GloL2of.to_string(), "GLO L2OF");
        assert_eq!(Code::GpsL1p.to_string(), "GPS L1P");
        assert_eq!(Code::GpsL2p.to_string(), "GPS L2P");
        assert_eq!(Code::GpsL2cl.to_string(), "GPS L2CL");
        assert_eq!(Code::GpsL2cx.to_string(), "GPS L2C");
        assert_eq!(Code::GpsL5i.to_string(), "GPS L5I");
        assert_eq!(Code::GpsL5q.to_string(), "GPS L5Q");
        assert_eq!(Code::GpsL5x.to_string(), "GPS L5");
        assert_eq!(Code::Bds2B1.to_string(), "BDS B1");
        assert_eq!(Code::Bds2B2.to_string(), "BDS B2");
        assert_eq!(Code::GalE1b.to_string(), "GAL E1B");
        assert_eq!(Code::GalE1c.to_string(), "GAL E1C");
        assert_eq!(Code::GalE1x.to_string(), "GAL E1");
        assert_eq!(Code::GalE6b.to_string(), "GAL E6B");
        assert_eq!(Code::GalE6c.to_string(), "GAL E6C");
        assert_eq!(Code::GalE6x.to_string(), "GAL E6");
        assert_eq!(Code::GalE7i.to_string(), "GAL E5bI");
        assert_eq!(Code::GalE7q.to_string(), "GAL E5bQ");
        assert_eq!(Code::GalE7x.to_string(), "GAL E5b");
        assert_eq!(Code::GalE8i.to_string(), "GAL E8I");
        assert_eq!(Code::GalE8q.to_string(), "GAL E8Q");
        assert_eq!(Code::GalE8x.to_string(), "GAL E8");
        assert_eq!(Code::GalE5i.to_string(), "GAL E5aI");
        assert_eq!(Code::GalE5q.to_string(), "GAL E5aQ");
        assert_eq!(Code::GalE5x.to_string(), "GAL E5a");
        assert_eq!(Code::GloL1p.to_string(), "GLO L1P");
        assert_eq!(Code::GloL2p.to_string(), "GLO L2P");
        assert_eq!(Code::QzsL1ca.to_string(), "QZS L1CA");
        assert_eq!(Code::QzsL1ci.to_string(), "QZS L1CI");
        assert_eq!(Code::QzsL1cq.to_string(), "QZS L1CQ");
        assert_eq!(Code::QzsL1cx.to_string(), "QZS L1CX");
        assert_eq!(Code::QzsL2cm.to_string(), "QZS L2CM");
        assert_eq!(Code::QzsL2cl.to_string(), "QZS L2CL");
        assert_eq!(Code::QzsL2cx.to_string(), "QZS L2C");
        assert_eq!(Code::QzsL5i.to_string(), "QZS L5I");
        assert_eq!(Code::QzsL5q.to_string(), "QZS L5Q");
        assert_eq!(Code::QzsL5x.to_string(), "QZS L5");
        assert_eq!(Code::SbasL5i.to_string(), "SBAS L5I");
        assert_eq!(Code::SbasL5q.to_string(), "SBAS L5Q");
        assert_eq!(Code::SbasL5x.to_string(), "SBAS L5");
        assert_eq!(Code::Bds3B1ci.to_string(), "BDS3 B1CI");
        assert_eq!(Code::Bds3B1cq.to_string(), "BDS3 B1CQ");
        assert_eq!(Code::Bds3B1cx.to_string(), "BDS3 B1C");
        assert_eq!(Code::Bds3B5i.to_string(), "BDS3 B5I");
        assert_eq!(Code::Bds3B5q.to_string(), "BDS3 B5Q");
        assert_eq!(Code::Bds3B5x.to_string(), "BDS3 B5");
        assert_eq!(Code::Bds3B7i.to_string(), "BDS3 B7I");
        assert_eq!(Code::Bds3B7q.to_string(), "BDS3 B7Q");
        assert_eq!(Code::Bds3B7x.to_string(), "BDS3 B7");
        assert_eq!(Code::Bds3B3i.to_string(), "BDS3 B3I");
        assert_eq!(Code::Bds3B3q.to_string(), "BDS3 B3Q");
        assert_eq!(Code::Bds3B3x.to_string(), "BDS3 B3");
        assert_eq!(Code::GpsL1ci.to_string(), "GPS L1CI");
        assert_eq!(Code::GpsL1cq.to_string(), "GPS L1CQ");
        assert_eq!(Code::GpsL1cx.to_string(), "GPS L1C");
        assert_eq!(Code::AuxGps.to_string(), "GPS AUX");
        assert_eq!(Code::AuxSbas.to_string(), "SBAS AUX");
        assert_eq!(Code::AuxGal.to_string(), "GAL AUX");
        assert_eq!(Code::AuxQzs.to_string(), "QZS AUX");
        assert_eq!(Code::AuxBds.to_string(), "BDS AUX");

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
    fn swiftnav_sys_int_values() {
        for (i, e) in ((swiftnav_sys::code_e_CODE_INVALID + 1)..(swiftnav_sys::code_e_CODE_COUNT))
            .zip(Code::iter())
        {
            assert_eq!(i, e as i32);
        }
    }
}
