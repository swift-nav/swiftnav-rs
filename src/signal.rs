//! Signal identifiers
//!
//! Signals are specific to a satellite and code combination. A satellite is
//! identified by it's assigned number and the constellation it belongs to. Each
//! satellite can send out multiple signals.

use crate::c_bindings;
use std::borrow::Cow;
use std::ffi;

/// GNSS satellite constellations
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
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
    fn from_constellation_t(value: c_bindings::constellation_t) -> Option<Constellation> {
        match value {
            c_bindings::constellation_e_CONSTELLATION_GPS => Some(Constellation::Gps),
            c_bindings::constellation_e_CONSTELLATION_SBAS => Some(Constellation::Sbas),
            c_bindings::constellation_e_CONSTELLATION_GLO => Some(Constellation::Glo),
            c_bindings::constellation_e_CONSTELLATION_BDS => Some(Constellation::Bds),
            c_bindings::constellation_e_CONSTELLATION_QZS => Some(Constellation::Qzs),
            c_bindings::constellation_e_CONSTELLATION_GAL => Some(Constellation::Gal),
            c_bindings::constellation_e_CONSTELLATION_INVALID
            | c_bindings::constellation_e_CONSTELLATION_COUNT
            | _ => None,
        }
    }

    pub(crate) fn to_constellation_t(&self) -> c_bindings::constellation_t {
        match *self {
            Constellation::Gps => c_bindings::constellation_e_CONSTELLATION_GPS,
            Constellation::Sbas => c_bindings::constellation_e_CONSTELLATION_SBAS,
            Constellation::Glo => c_bindings::constellation_e_CONSTELLATION_GLO,
            Constellation::Bds => c_bindings::constellation_e_CONSTELLATION_BDS,
            Constellation::Qzs => c_bindings::constellation_e_CONSTELLATION_QZS,
            Constellation::Gal => c_bindings::constellation_e_CONSTELLATION_GAL,
        }
    }

    /// Gets the specified maximum number of active satellites for the constellation
    pub fn sat_count(&self) -> u16 {
        unsafe { c_bindings::constellation_to_sat_count(*self as c_bindings::constellation_t) }
    }

    pub fn to_str(&self) -> Cow<'static, str> {
        let c_str = unsafe {
            ffi::CStr::from_ptr(c_bindings::constellation_to_string(
                self.to_constellation_t(),
            ))
        };
        c_str.to_string_lossy()
    }
}

/// Code identifiers
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
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

impl Code {
    pub(crate) fn from_code_t(value: c_bindings::code_t) -> Option<Code> {
        match value {
            c_bindings::code_e_CODE_GPS_L1CA => Some(Code::GpsL1ca),
            c_bindings::code_e_CODE_GPS_L2CM => Some(Code::GpsL2cm),
            c_bindings::code_e_CODE_SBAS_L1CA => Some(Code::SbasL1ca),
            c_bindings::code_e_CODE_GLO_L1OF => Some(Code::GloL1of),
            c_bindings::code_e_CODE_GLO_L2OF => Some(Code::GloL2of),
            c_bindings::code_e_CODE_GPS_L1P => Some(Code::GpsL1p),
            c_bindings::code_e_CODE_GPS_L2P => Some(Code::GpsL2p),
            c_bindings::code_e_CODE_GPS_L2CL => Some(Code::GpsL2cl),
            c_bindings::code_e_CODE_GPS_L2CX => Some(Code::GpsL2cx),
            c_bindings::code_e_CODE_GPS_L5I => Some(Code::GpsL5i),
            c_bindings::code_e_CODE_GPS_L5Q => Some(Code::GpsL5q),
            c_bindings::code_e_CODE_GPS_L5X => Some(Code::GpsL5x),
            c_bindings::code_e_CODE_BDS2_B1 => Some(Code::Bds2B1),
            c_bindings::code_e_CODE_BDS2_B2 => Some(Code::Bds2B2),
            c_bindings::code_e_CODE_GAL_E1B => Some(Code::GalE1b),
            c_bindings::code_e_CODE_GAL_E1C => Some(Code::GalE1c),
            c_bindings::code_e_CODE_GAL_E1X => Some(Code::GalE1x),
            c_bindings::code_e_CODE_GAL_E6B => Some(Code::GalE6b),
            c_bindings::code_e_CODE_GAL_E6C => Some(Code::GalE6c),
            c_bindings::code_e_CODE_GAL_E6X => Some(Code::GalE6x),
            c_bindings::code_e_CODE_GAL_E7I => Some(Code::GalE7i),
            c_bindings::code_e_CODE_GAL_E7Q => Some(Code::GalE7q),
            c_bindings::code_e_CODE_GAL_E7X => Some(Code::GalE7x),
            c_bindings::code_e_CODE_GAL_E8I => Some(Code::GalE8i),
            c_bindings::code_e_CODE_GAL_E8Q => Some(Code::GalE8q),
            c_bindings::code_e_CODE_GAL_E8X => Some(Code::GalE8x),
            c_bindings::code_e_CODE_GAL_E5I => Some(Code::GalE5i),
            c_bindings::code_e_CODE_GAL_E5Q => Some(Code::GalE5q),
            c_bindings::code_e_CODE_GAL_E5X => Some(Code::GalE5x),
            c_bindings::code_e_CODE_GLO_L1P => Some(Code::GloL1p),
            c_bindings::code_e_CODE_GLO_L2P => Some(Code::GloL2p),
            c_bindings::code_e_CODE_QZS_L1CA => Some(Code::QzsL1ca),
            c_bindings::code_e_CODE_QZS_L1CI => Some(Code::QzsL1ci),
            c_bindings::code_e_CODE_QZS_L1CQ => Some(Code::QzsL1cq),
            c_bindings::code_e_CODE_QZS_L1CX => Some(Code::QzsL1cx),
            c_bindings::code_e_CODE_QZS_L2CM => Some(Code::QzsL2cm),
            c_bindings::code_e_CODE_QZS_L2CL => Some(Code::QzsL2cl),
            c_bindings::code_e_CODE_QZS_L2CX => Some(Code::QzsL2cx),
            c_bindings::code_e_CODE_QZS_L5I => Some(Code::QzsL5i),
            c_bindings::code_e_CODE_QZS_L5Q => Some(Code::QzsL5q),
            c_bindings::code_e_CODE_QZS_L5X => Some(Code::QzsL5x),
            c_bindings::code_e_CODE_SBAS_L5I => Some(Code::SbasL5i),
            c_bindings::code_e_CODE_SBAS_L5Q => Some(Code::SbasL5q),
            c_bindings::code_e_CODE_SBAS_L5X => Some(Code::SbasL5x),
            c_bindings::code_e_CODE_BDS3_B1CI => Some(Code::Bds3B1ci),
            c_bindings::code_e_CODE_BDS3_B1CQ => Some(Code::Bds3B1cq),
            c_bindings::code_e_CODE_BDS3_B1CX => Some(Code::Bds3B1cx),
            c_bindings::code_e_CODE_BDS3_B5I => Some(Code::Bds3B5i),
            c_bindings::code_e_CODE_BDS3_B5Q => Some(Code::Bds3B5q),
            c_bindings::code_e_CODE_BDS3_B5X => Some(Code::Bds3B5x),
            c_bindings::code_e_CODE_BDS3_B7I => Some(Code::Bds3B7i),
            c_bindings::code_e_CODE_BDS3_B7Q => Some(Code::Bds3B7q),
            c_bindings::code_e_CODE_BDS3_B7X => Some(Code::Bds3B7x),
            c_bindings::code_e_CODE_BDS3_B3I => Some(Code::Bds3B3i),
            c_bindings::code_e_CODE_BDS3_B3Q => Some(Code::Bds3B3q),
            c_bindings::code_e_CODE_BDS3_B3X => Some(Code::Bds3B3x),
            c_bindings::code_e_CODE_GPS_L1CI => Some(Code::GpsL1ci),
            c_bindings::code_e_CODE_GPS_L1CQ => Some(Code::GpsL1cq),
            c_bindings::code_e_CODE_GPS_L1CX => Some(Code::GpsL1cx),
            c_bindings::code_e_CODE_AUX_GPS => Some(Code::AuxGps),
            c_bindings::code_e_CODE_AUX_SBAS => Some(Code::AuxSbas),
            c_bindings::code_e_CODE_AUX_GAL => Some(Code::AuxGal),
            c_bindings::code_e_CODE_AUX_QZS => Some(Code::AuxQzs),
            c_bindings::code_e_CODE_AUX_BDS => Some(Code::AuxBds),
            c_bindings::code_e_CODE_INVALID | c_bindings::code_e_CODE_COUNT | _ => None,
        }
    }

    pub(crate) fn to_code_t(&self) -> c_bindings::code_t {
        match *self {
            Code::GpsL1ca => c_bindings::code_e_CODE_GPS_L1CA,
            Code::GpsL2cm => c_bindings::code_e_CODE_GPS_L2CM,
            Code::SbasL1ca => c_bindings::code_e_CODE_SBAS_L1CA,
            Code::GloL1of => c_bindings::code_e_CODE_GLO_L1OF,
            Code::GloL2of => c_bindings::code_e_CODE_GLO_L2OF,
            Code::GpsL1p => c_bindings::code_e_CODE_GPS_L1P,
            Code::GpsL2p => c_bindings::code_e_CODE_GPS_L2P,
            Code::GpsL2cl => c_bindings::code_e_CODE_GPS_L2CL,
            Code::GpsL2cx => c_bindings::code_e_CODE_GPS_L2CX,
            Code::GpsL5i => c_bindings::code_e_CODE_GPS_L5I,
            Code::GpsL5q => c_bindings::code_e_CODE_GPS_L5Q,
            Code::GpsL5x => c_bindings::code_e_CODE_GPS_L5X,
            Code::Bds2B1 => c_bindings::code_e_CODE_BDS2_B1,
            Code::Bds2B2 => c_bindings::code_e_CODE_BDS2_B2,
            Code::GalE1b => c_bindings::code_e_CODE_GAL_E1B,
            Code::GalE1c => c_bindings::code_e_CODE_GAL_E1C,
            Code::GalE1x => c_bindings::code_e_CODE_GAL_E1X,
            Code::GalE6b => c_bindings::code_e_CODE_GAL_E6B,
            Code::GalE6c => c_bindings::code_e_CODE_GAL_E6C,
            Code::GalE6x => c_bindings::code_e_CODE_GAL_E6X,
            Code::GalE7i => c_bindings::code_e_CODE_GAL_E7I,
            Code::GalE7q => c_bindings::code_e_CODE_GAL_E7Q,
            Code::GalE7x => c_bindings::code_e_CODE_GAL_E7X,
            Code::GalE8i => c_bindings::code_e_CODE_GAL_E8I,
            Code::GalE8q => c_bindings::code_e_CODE_GAL_E8Q,
            Code::GalE8x => c_bindings::code_e_CODE_GAL_E8X,
            Code::GalE5i => c_bindings::code_e_CODE_GAL_E5I,
            Code::GalE5q => c_bindings::code_e_CODE_GAL_E5Q,
            Code::GalE5x => c_bindings::code_e_CODE_GAL_E5X,
            Code::GloL1p => c_bindings::code_e_CODE_GLO_L1P,
            Code::GloL2p => c_bindings::code_e_CODE_GLO_L2P,
            Code::QzsL1ca => c_bindings::code_e_CODE_QZS_L1CA,
            Code::QzsL1ci => c_bindings::code_e_CODE_QZS_L1CI,
            Code::QzsL1cq => c_bindings::code_e_CODE_QZS_L1CQ,
            Code::QzsL1cx => c_bindings::code_e_CODE_QZS_L1CX,
            Code::QzsL2cm => c_bindings::code_e_CODE_QZS_L2CM,
            Code::QzsL2cl => c_bindings::code_e_CODE_QZS_L2CL,
            Code::QzsL2cx => c_bindings::code_e_CODE_QZS_L2CX,
            Code::QzsL5i => c_bindings::code_e_CODE_QZS_L5I,
            Code::QzsL5q => c_bindings::code_e_CODE_QZS_L5Q,
            Code::QzsL5x => c_bindings::code_e_CODE_QZS_L5X,
            Code::SbasL5i => c_bindings::code_e_CODE_SBAS_L5I,
            Code::SbasL5q => c_bindings::code_e_CODE_SBAS_L5Q,
            Code::SbasL5x => c_bindings::code_e_CODE_SBAS_L5X,
            Code::Bds3B1ci => c_bindings::code_e_CODE_BDS3_B1CI,
            Code::Bds3B1cq => c_bindings::code_e_CODE_BDS3_B1CQ,
            Code::Bds3B1cx => c_bindings::code_e_CODE_BDS3_B1CX,
            Code::Bds3B5i => c_bindings::code_e_CODE_BDS3_B5I,
            Code::Bds3B5q => c_bindings::code_e_CODE_BDS3_B5Q,
            Code::Bds3B5x => c_bindings::code_e_CODE_BDS3_B5X,
            Code::Bds3B7i => c_bindings::code_e_CODE_BDS3_B7I,
            Code::Bds3B7q => c_bindings::code_e_CODE_BDS3_B7Q,
            Code::Bds3B7x => c_bindings::code_e_CODE_BDS3_B7X,
            Code::Bds3B3i => c_bindings::code_e_CODE_BDS3_B3I,
            Code::Bds3B3q => c_bindings::code_e_CODE_BDS3_B3Q,
            Code::Bds3B3x => c_bindings::code_e_CODE_BDS3_B3X,
            Code::GpsL1ci => c_bindings::code_e_CODE_GPS_L1CI,
            Code::GpsL1cq => c_bindings::code_e_CODE_GPS_L1CQ,
            Code::GpsL1cx => c_bindings::code_e_CODE_GPS_L1CX,
            Code::AuxGps => c_bindings::code_e_CODE_AUX_GPS,
            Code::AuxSbas => c_bindings::code_e_CODE_AUX_SBAS,
            Code::AuxGal => c_bindings::code_e_CODE_AUX_GAL,
            Code::AuxQzs => c_bindings::code_e_CODE_AUX_QZS,
            Code::AuxBds => c_bindings::code_e_CODE_AUX_BDS,
        }
    }

    /// Attempts to make a `Code` from a string
    pub fn from_str(s: &ffi::CStr) -> Option<Code> {
        Self::from_code_t(unsafe { c_bindings::code_string_to_enum(s.as_ptr()) })
    }

    pub fn to_str(&self) -> Cow<'static, str> {
        let c_str = unsafe { ffi::CStr::from_ptr(c_bindings::code_to_string(self.to_code_t())) };
        c_str.to_string_lossy()
    }

    /// Gets  the corresponding `Constellation`
    pub fn to_constellation(&self) -> Constellation {
        Constellation::from_constellation_t(unsafe {
            c_bindings::code_to_constellation(self.to_code_t())
        })
        .unwrap()
    }

    /// Get the number of signals for a code
    pub fn sig_count(&self) -> u16 {
        unsafe { c_bindings::code_to_sig_count(self.to_code_t()) }
    }

    /// Get the chips count of a code
    pub fn chip_count(&self) -> u32 {
        unsafe { c_bindings::code_to_chip_count(self.to_code_t()) }
    }

    /// Get the chips rate of a code
    pub fn chip_rate(&self) -> f64 {
        unsafe { c_bindings::code_to_chip_rate(self.to_code_t()) }
    }

    pub fn is_gps(&self) -> bool {
        unsafe { c_bindings::is_gps(self.to_code_t()) }
    }

    pub fn is_sbas(&self) -> bool {
        unsafe { c_bindings::is_sbas(self.to_code_t()) }
    }

    pub fn is_glo(&self) -> bool {
        unsafe { c_bindings::is_glo(self.to_code_t()) }
    }

    pub fn is_bds2(&self) -> bool {
        unsafe { c_bindings::is_bds2(self.to_code_t()) }
    }

    pub fn is_gal(&self) -> bool {
        unsafe { c_bindings::is_gal(self.to_code_t()) }
    }

    pub fn is_qzss(&self) -> bool {
        unsafe { c_bindings::is_qzss(self.to_code_t()) }
    }
}

/// GNSS Signal identifier
#[derive(Copy, Clone)]
pub struct GnssSignal(c_bindings::gnss_signal_t);

impl GnssSignal {
    pub fn new(sat: u16, code: Code) -> GnssSignal {
        let code = code as c_bindings::code_t;
        GnssSignal(c_bindings::gnss_signal_t { sat, code })
    }

    pub(crate) fn from_gnss_signal_t(sid: c_bindings::gnss_signal_t) -> Option<GnssSignal> {
        Some(GnssSignal::new(sid.sat, Code::from_code_t(sid.code)?))
    }

    pub(crate) fn to_gnss_signal_t(&self) -> c_bindings::gnss_signal_t {
        self.0
    }

    pub fn get_sat(&self) -> u16 {
        self.0.sat
    }

    pub fn get_code(&self) -> Code {
        Code::from_code_t(self.0.code).unwrap()
    }

    /// Get the constellation of the signal
    pub fn to_constellation(&self) -> Constellation {
        Constellation::from_constellation_t(unsafe { c_bindings::sid_to_constellation(self.0) })
            .unwrap()
    }

    /// Get the carrier frequency of the signal
    pub fn carrier_frequency(&self) -> f64 {
        unsafe { c_bindings::sid_to_carr_freq(self.0) }
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
            GnssSignal::new(0, Code::GpsL1ca).to_constellation(),
            Constellation::Gps
        );
        assert_eq!(
            GnssSignal::new(0, Code::GpsL2cm).to_constellation(),
            Constellation::Gps
        );
        assert_eq!(
            GnssSignal::new(0, Code::SbasL1ca).to_constellation(),
            Constellation::Sbas
        );
        assert_eq!(
            GnssSignal::new(0, Code::GloL1of).to_constellation(),
            Constellation::Glo
        );
        assert_eq!(
            GnssSignal::new(0, Code::GloL2of).to_constellation(),
            Constellation::Glo
        );
        assert_eq!(
            GnssSignal::new(0, Code::GpsL1p).to_constellation(),
            Constellation::Gps
        );
        assert_eq!(
            GnssSignal::new(0, Code::GpsL2p).to_constellation(),
            Constellation::Gps
        );
        assert_eq!(
            GnssSignal::new(0, Code::GpsL2cl).to_constellation(),
            Constellation::Gps
        );
        assert_eq!(
            GnssSignal::new(0, Code::GpsL2cx).to_constellation(),
            Constellation::Gps
        );
        assert_eq!(
            GnssSignal::new(0, Code::GpsL5i).to_constellation(),
            Constellation::Gps
        );
        assert_eq!(
            GnssSignal::new(0, Code::GpsL5q).to_constellation(),
            Constellation::Gps
        );
        assert_eq!(
            GnssSignal::new(0, Code::GpsL5x).to_constellation(),
            Constellation::Gps
        );
        assert_eq!(
            GnssSignal::new(0, Code::Bds2B1).to_constellation(),
            Constellation::Bds
        );
        assert_eq!(
            GnssSignal::new(0, Code::Bds2B2).to_constellation(),
            Constellation::Bds
        );
        assert_eq!(
            GnssSignal::new(0, Code::GalE1b).to_constellation(),
            Constellation::Gal
        );
        assert_eq!(
            GnssSignal::new(0, Code::GalE1c).to_constellation(),
            Constellation::Gal
        );
        assert_eq!(
            GnssSignal::new(0, Code::GalE1x).to_constellation(),
            Constellation::Gal
        );
        assert_eq!(
            GnssSignal::new(0, Code::GalE6b).to_constellation(),
            Constellation::Gal
        );
        assert_eq!(
            GnssSignal::new(0, Code::GalE6c).to_constellation(),
            Constellation::Gal
        );
        assert_eq!(
            GnssSignal::new(0, Code::GalE6x).to_constellation(),
            Constellation::Gal
        );
        assert_eq!(
            GnssSignal::new(0, Code::GalE7i).to_constellation(),
            Constellation::Gal
        );
        assert_eq!(
            GnssSignal::new(0, Code::GalE7q).to_constellation(),
            Constellation::Gal
        );
        assert_eq!(
            GnssSignal::new(0, Code::GalE7x).to_constellation(),
            Constellation::Gal
        );
        assert_eq!(
            GnssSignal::new(0, Code::GalE8i).to_constellation(),
            Constellation::Gal
        );
        assert_eq!(
            GnssSignal::new(0, Code::GalE8q).to_constellation(),
            Constellation::Gal
        );
        assert_eq!(
            GnssSignal::new(0, Code::GalE8x).to_constellation(),
            Constellation::Gal
        );
        assert_eq!(
            GnssSignal::new(0, Code::GalE5i).to_constellation(),
            Constellation::Gal
        );
        assert_eq!(
            GnssSignal::new(0, Code::GalE5q).to_constellation(),
            Constellation::Gal
        );
        assert_eq!(
            GnssSignal::new(0, Code::GalE5x).to_constellation(),
            Constellation::Gal
        );
        assert_eq!(
            GnssSignal::new(0, Code::GloL1p).to_constellation(),
            Constellation::Glo
        );
        assert_eq!(
            GnssSignal::new(0, Code::GloL2p).to_constellation(),
            Constellation::Glo
        );
        assert_eq!(
            GnssSignal::new(0, Code::QzsL1ca).to_constellation(),
            Constellation::Qzs
        );
        assert_eq!(
            GnssSignal::new(0, Code::QzsL1ci).to_constellation(),
            Constellation::Qzs
        );
        assert_eq!(
            GnssSignal::new(0, Code::QzsL1cq).to_constellation(),
            Constellation::Qzs
        );
        assert_eq!(
            GnssSignal::new(0, Code::QzsL1cx).to_constellation(),
            Constellation::Qzs
        );
        assert_eq!(
            GnssSignal::new(0, Code::QzsL2cm).to_constellation(),
            Constellation::Qzs
        );
        assert_eq!(
            GnssSignal::new(0, Code::QzsL2cl).to_constellation(),
            Constellation::Qzs
        );
        assert_eq!(
            GnssSignal::new(0, Code::QzsL2cx).to_constellation(),
            Constellation::Qzs
        );
        assert_eq!(
            GnssSignal::new(0, Code::QzsL5i).to_constellation(),
            Constellation::Qzs
        );
        assert_eq!(
            GnssSignal::new(0, Code::QzsL5q).to_constellation(),
            Constellation::Qzs
        );
        assert_eq!(
            GnssSignal::new(0, Code::QzsL5x).to_constellation(),
            Constellation::Qzs
        );
        assert_eq!(
            GnssSignal::new(0, Code::SbasL5i).to_constellation(),
            Constellation::Sbas
        );
        assert_eq!(
            GnssSignal::new(0, Code::SbasL5q).to_constellation(),
            Constellation::Sbas
        );
        assert_eq!(
            GnssSignal::new(0, Code::SbasL5x).to_constellation(),
            Constellation::Sbas
        );
        assert_eq!(
            GnssSignal::new(0, Code::Bds3B1ci).to_constellation(),
            Constellation::Bds
        );
        assert_eq!(
            GnssSignal::new(0, Code::Bds3B1cq).to_constellation(),
            Constellation::Bds
        );
        assert_eq!(
            GnssSignal::new(0, Code::Bds3B1cx).to_constellation(),
            Constellation::Bds
        );
        assert_eq!(
            GnssSignal::new(0, Code::Bds3B5i).to_constellation(),
            Constellation::Bds
        );
        assert_eq!(
            GnssSignal::new(0, Code::Bds3B5q).to_constellation(),
            Constellation::Bds
        );
        assert_eq!(
            GnssSignal::new(0, Code::Bds3B5x).to_constellation(),
            Constellation::Bds
        );
        assert_eq!(
            GnssSignal::new(0, Code::Bds3B7i).to_constellation(),
            Constellation::Bds
        );
        assert_eq!(
            GnssSignal::new(0, Code::Bds3B7q).to_constellation(),
            Constellation::Bds
        );
        assert_eq!(
            GnssSignal::new(0, Code::Bds3B7x).to_constellation(),
            Constellation::Bds
        );
        assert_eq!(
            GnssSignal::new(0, Code::Bds3B3i).to_constellation(),
            Constellation::Bds
        );
        assert_eq!(
            GnssSignal::new(0, Code::Bds3B3q).to_constellation(),
            Constellation::Bds
        );
        assert_eq!(
            GnssSignal::new(0, Code::Bds3B3x).to_constellation(),
            Constellation::Bds
        );
        assert_eq!(
            GnssSignal::new(0, Code::GpsL1ci).to_constellation(),
            Constellation::Gps
        );
        assert_eq!(
            GnssSignal::new(0, Code::GpsL1cq).to_constellation(),
            Constellation::Gps
        );
        assert_eq!(
            GnssSignal::new(0, Code::GpsL1cx).to_constellation(),
            Constellation::Gps
        );
        assert_eq!(
            GnssSignal::new(0, Code::AuxGps).to_constellation(),
            Constellation::Gps
        );
        assert_eq!(
            GnssSignal::new(0, Code::AuxSbas).to_constellation(),
            Constellation::Sbas
        );
        assert_eq!(
            GnssSignal::new(0, Code::AuxGal).to_constellation(),
            Constellation::Gal
        );
        assert_eq!(
            GnssSignal::new(0, Code::AuxQzs).to_constellation(),
            Constellation::Qzs
        );
        assert_eq!(
            GnssSignal::new(0, Code::AuxBds).to_constellation(),
            Constellation::Bds
        );
    }
}
