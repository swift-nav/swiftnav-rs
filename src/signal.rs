use crate::c_bindings::signal as signal_c;
use std::ffi;
use std::str::Utf8Error;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(i32)]
pub enum Constellation {
    Gps = signal_c::constellation_e_CONSTELLATION_GPS,
    Sbas = signal_c::constellation_e_CONSTELLATION_SBAS,
    Glo = signal_c::constellation_e_CONSTELLATION_GLO,
    Bds = signal_c::constellation_e_CONSTELLATION_BDS,
    Qzs = signal_c::constellation_e_CONSTELLATION_QZS,
    Gal = signal_c::constellation_e_CONSTELLATION_GAL,
}

impl Constellation {
    fn from_constellation_t(value: signal_c::constellation_t) -> Option<Constellation> {
        match value {
            signal_c::constellation_e_CONSTELLATION_GPS => Some(Constellation::Gps),
            signal_c::constellation_e_CONSTELLATION_SBAS => Some(Constellation::Sbas),
            signal_c::constellation_e_CONSTELLATION_GLO => Some(Constellation::Glo),
            signal_c::constellation_e_CONSTELLATION_BDS => Some(Constellation::Bds),
            signal_c::constellation_e_CONSTELLATION_QZS => Some(Constellation::Qzs),
            signal_c::constellation_e_CONSTELLATION_GAL => Some(Constellation::Gal),
            signal_c::constellation_e_CONSTELLATION_INVALID
            | signal_c::constellation_e_CONSTELLATION_COUNT
            | _ => None,
        }
    }

    pub fn sat_count(&self) -> u16 {
        unsafe { signal_c::constellation_to_sat_count(*self as signal_c::constellation_t) }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(i32)]
pub enum Code {
    GpsL1ca = signal_c::code_e_CODE_GPS_L1CA, /* GPS L1CA: BPSK(1) */
    GpsL2cm = signal_c::code_e_CODE_GPS_L2CM, /* GPS L2C: 2 x BPSK(0.5) */
    SbasL1ca = signal_c::code_e_CODE_SBAS_L1CA, /* SBAS L1: BPSK(1) */
    GloL1of = signal_c::code_e_CODE_GLO_L1OF, /* GLONASS L1OF: FDMA BPSK(0.5) */
    GloL2of = signal_c::code_e_CODE_GLO_L2OF, /* GLONASS L2OF: FDMA BPSK(0.5) */
    GpsL1p = signal_c::code_e_CODE_GPS_L1P,   /* GPS L1P(Y): encrypted BPSK(10) */
    GpsL2p = signal_c::code_e_CODE_GPS_L2P,   /* GPS L2P(Y): encrypted BPSK(10) */
    GpsL2cl = signal_c::code_e_CODE_GPS_L2CL,
    GpsL2cx = signal_c::code_e_CODE_GPS_L2CX,
    GpsL5i = signal_c::code_e_CODE_GPS_L5I, /* GPS L5: QPSK(10) at 1150*f0 */
    GpsL5q = signal_c::code_e_CODE_GPS_L5Q,
    GpsL5x = signal_c::code_e_CODE_GPS_L5X,
    Bds2B1 = signal_c::code_e_CODE_BDS2_B1, /* BDS2 B1I: BPSK(2) at 1526*f0 */
    Bds2B2 = signal_c::code_e_CODE_BDS2_B2, /* BDS2 B2I: BPSK(2) at 1180*f0 */
    GalE1b = signal_c::code_e_CODE_GAL_E1B, /* Galileo E1: CASM CBOC(1,1) at 1540*f0 */
    GalE1c = signal_c::code_e_CODE_GAL_E1C,
    GalE1x = signal_c::code_e_CODE_GAL_E1X,
    GalE6b = signal_c::code_e_CODE_GAL_E6B, /* Galileo E6: CASM BPSK(5) at 1250*f0 */
    GalE6c = signal_c::code_e_CODE_GAL_E6C,
    GalE6x = signal_c::code_e_CODE_GAL_E6X,
    GalE7i = signal_c::code_e_CODE_GAL_E7I, /* Galileo E5b: QPSK(10) at 1180*f0 */
    GalE7q = signal_c::code_e_CODE_GAL_E7Q,
    GalE7x = signal_c::code_e_CODE_GAL_E7X,
    GalE8i = signal_c::code_e_CODE_GAL_E8I, /* Galileo E5AltBOC(15,10) at 1165*f0 */
    GalE8q = signal_c::code_e_CODE_GAL_E8Q,
    GalE8x = signal_c::code_e_CODE_GAL_E8X,
    GalE5i = signal_c::code_e_CODE_GAL_E5I, /* Galileo E5a: QPSK(10) at 1150*f0 */
    GalE5q = signal_c::code_e_CODE_GAL_E5Q,
    GalE5x = signal_c::code_e_CODE_GAL_E5X,
    GloL1p = signal_c::code_e_CODE_GLO_L1P, /* GLONASS L1P: encrypted */
    GloL2p = signal_c::code_e_CODE_GLO_L2P, /* GLONASS L2P: encrypted */
    QzsL1ca = signal_c::code_e_CODE_QZS_L1CA, /* QZSS L1CA: BPSK(1) at 1540*f0 */
    QzsL1ci = signal_c::code_e_CODE_QZS_L1CI, /* QZSS L1C: TM-BOC at 1540*f0 */
    QzsL1cq = signal_c::code_e_CODE_QZS_L1CQ,
    QzsL1cx = signal_c::code_e_CODE_QZS_L1CX,
    QzsL2cm = signal_c::code_e_CODE_QZS_L2CM, /* QZSS L2C: 2 x BPSK(0.5) at 1200*f0 */
    QzsL2cl = signal_c::code_e_CODE_QZS_L2CL,
    QzsL2cx = signal_c::code_e_CODE_QZS_L2CX,
    QzsL5i = signal_c::code_e_CODE_QZS_L5I, /* QZSS L5: QPSK(10) at 1150*f0 */
    QzsL5q = signal_c::code_e_CODE_QZS_L5Q,
    QzsL5x = signal_c::code_e_CODE_QZS_L5X,
    SbasL5i = signal_c::code_e_CODE_SBAS_L5I, /* SBAS L5: ? at 1150*f0 */
    SbasL5q = signal_c::code_e_CODE_SBAS_L5Q,
    SbasL5x = signal_c::code_e_CODE_SBAS_L5X,
    Bds3B1ci = signal_c::code_e_CODE_BDS3_B1CI, /* BDS3 B1C: TM-BOC at 1540*f0 */
    Bds3B1cq = signal_c::code_e_CODE_BDS3_B1CQ,
    Bds3B1cx = signal_c::code_e_CODE_BDS3_B1CX,
    Bds3B5i = signal_c::code_e_CODE_BDS3_B5I, /* BDS3 B2a: QPSK(10) at 1150*f0 */
    Bds3B5q = signal_c::code_e_CODE_BDS3_B5Q,
    Bds3B5x = signal_c::code_e_CODE_BDS3_B5X,
    Bds3B7i = signal_c::code_e_CODE_BDS3_B7I, /* BDS3 B2b: QPSK(10) at 1180*f0 */
    Bds3B7q = signal_c::code_e_CODE_BDS3_B7Q,
    Bds3B7x = signal_c::code_e_CODE_BDS3_B7X,
    Bds3B3i = signal_c::code_e_CODE_BDS3_B3I, /* BDS3 B3I: QPSK(10) at 1240*f0 */
    Bds3B3q = signal_c::code_e_CODE_BDS3_B3Q,
    Bds3B3x = signal_c::code_e_CODE_BDS3_B3X,
    GpsL1ci = signal_c::code_e_CODE_GPS_L1CI, /* GPS L1C: TM-BOC at 1540*f0 */
    GpsL1cq = signal_c::code_e_CODE_GPS_L1CQ,
    GpsL1cx = signal_c::code_e_CODE_GPS_L1CX,
    AuxGps = signal_c::code_e_CODE_AUX_GPS, /* Auxiliary antenna signals */
    AuxSbas = signal_c::code_e_CODE_AUX_SBAS,
    AuxGal = signal_c::code_e_CODE_AUX_GAL,
    AuxQzs = signal_c::code_e_CODE_AUX_QZS,
    AuxBds = signal_c::code_e_CODE_AUX_BDS,
}

impl Code {
    fn from_code_t(value: signal_c::code_t) -> Option<Code> {
        match value {
            signal_c::code_e_CODE_GPS_L1CA => Some(Code::GpsL1ca),
            signal_c::code_e_CODE_GPS_L2CM => Some(Code::GpsL2cm),
            signal_c::code_e_CODE_SBAS_L1CA => Some(Code::SbasL1ca),
            signal_c::code_e_CODE_GLO_L1OF => Some(Code::GloL1of),
            signal_c::code_e_CODE_GLO_L2OF => Some(Code::GloL2of),
            signal_c::code_e_CODE_GPS_L1P => Some(Code::GpsL1p),
            signal_c::code_e_CODE_GPS_L2P => Some(Code::GpsL2p),
            signal_c::code_e_CODE_GPS_L2CL => Some(Code::GpsL2cl),
            signal_c::code_e_CODE_GPS_L2CX => Some(Code::GpsL2cx),
            signal_c::code_e_CODE_GPS_L5I => Some(Code::GpsL5i),
            signal_c::code_e_CODE_GPS_L5Q => Some(Code::GpsL5q),
            signal_c::code_e_CODE_GPS_L5X => Some(Code::GpsL5x),
            signal_c::code_e_CODE_BDS2_B1 => Some(Code::Bds2B1),
            signal_c::code_e_CODE_BDS2_B2 => Some(Code::Bds2B2),
            signal_c::code_e_CODE_GAL_E1B => Some(Code::GalE1b),
            signal_c::code_e_CODE_GAL_E1C => Some(Code::GalE1c),
            signal_c::code_e_CODE_GAL_E1X => Some(Code::GalE1x),
            signal_c::code_e_CODE_GAL_E6B => Some(Code::GalE6b),
            signal_c::code_e_CODE_GAL_E6C => Some(Code::GalE6c),
            signal_c::code_e_CODE_GAL_E6X => Some(Code::GalE6x),
            signal_c::code_e_CODE_GAL_E7I => Some(Code::GalE7i),
            signal_c::code_e_CODE_GAL_E7Q => Some(Code::GalE7q),
            signal_c::code_e_CODE_GAL_E7X => Some(Code::GalE7x),
            signal_c::code_e_CODE_GAL_E8I => Some(Code::GalE8i),
            signal_c::code_e_CODE_GAL_E8Q => Some(Code::GalE8q),
            signal_c::code_e_CODE_GAL_E8X => Some(Code::GalE8x),
            signal_c::code_e_CODE_GAL_E5I => Some(Code::GalE5i),
            signal_c::code_e_CODE_GAL_E5Q => Some(Code::GalE5q),
            signal_c::code_e_CODE_GAL_E5X => Some(Code::GalE5x),
            signal_c::code_e_CODE_GLO_L1P => Some(Code::GloL1p),
            signal_c::code_e_CODE_GLO_L2P => Some(Code::GloL2p),
            signal_c::code_e_CODE_QZS_L1CA => Some(Code::QzsL1ca),
            signal_c::code_e_CODE_QZS_L1CI => Some(Code::QzsL1ci),
            signal_c::code_e_CODE_QZS_L1CQ => Some(Code::QzsL1cq),
            signal_c::code_e_CODE_QZS_L1CX => Some(Code::QzsL1cx),
            signal_c::code_e_CODE_QZS_L2CM => Some(Code::QzsL2cm),
            signal_c::code_e_CODE_QZS_L2CL => Some(Code::QzsL2cl),
            signal_c::code_e_CODE_QZS_L2CX => Some(Code::QzsL2cx),
            signal_c::code_e_CODE_QZS_L5I => Some(Code::QzsL5i),
            signal_c::code_e_CODE_QZS_L5Q => Some(Code::QzsL5q),
            signal_c::code_e_CODE_QZS_L5X => Some(Code::QzsL5x),
            signal_c::code_e_CODE_SBAS_L5I => Some(Code::SbasL5i),
            signal_c::code_e_CODE_SBAS_L5Q => Some(Code::SbasL5q),
            signal_c::code_e_CODE_SBAS_L5X => Some(Code::SbasL5x),
            signal_c::code_e_CODE_BDS3_B1CI => Some(Code::Bds3B1ci),
            signal_c::code_e_CODE_BDS3_B1CQ => Some(Code::Bds3B1cq),
            signal_c::code_e_CODE_BDS3_B1CX => Some(Code::Bds3B1cx),
            signal_c::code_e_CODE_BDS3_B5I => Some(Code::Bds3B5i),
            signal_c::code_e_CODE_BDS3_B5Q => Some(Code::Bds3B5q),
            signal_c::code_e_CODE_BDS3_B5X => Some(Code::Bds3B5x),
            signal_c::code_e_CODE_BDS3_B7I => Some(Code::Bds3B7i),
            signal_c::code_e_CODE_BDS3_B7Q => Some(Code::Bds3B7q),
            signal_c::code_e_CODE_BDS3_B7X => Some(Code::Bds3B7x),
            signal_c::code_e_CODE_BDS3_B3I => Some(Code::Bds3B3i),
            signal_c::code_e_CODE_BDS3_B3Q => Some(Code::Bds3B3q),
            signal_c::code_e_CODE_BDS3_B3X => Some(Code::Bds3B3x),
            signal_c::code_e_CODE_GPS_L1CI => Some(Code::GpsL1ci),
            signal_c::code_e_CODE_GPS_L1CQ => Some(Code::GpsL1cq),
            signal_c::code_e_CODE_GPS_L1CX => Some(Code::GpsL1cx),
            signal_c::code_e_CODE_AUX_GPS => Some(Code::AuxGps),
            signal_c::code_e_CODE_AUX_SBAS => Some(Code::AuxSbas),
            signal_c::code_e_CODE_AUX_GAL => Some(Code::AuxGal),
            signal_c::code_e_CODE_AUX_QZS => Some(Code::AuxQzs),
            signal_c::code_e_CODE_AUX_BDS => Some(Code::AuxBds),
            signal_c::code_e_CODE_INVALID | signal_c::code_e_CODE_COUNT | _ => None,
        }
    }

    pub fn from_str(s: &ffi::CStr) -> Option<Code> {
        Self::from_code_t(unsafe { signal_c::code_string_to_enum(s.as_ptr()) })
    }

    pub fn to_string(&self) -> Result<String, Utf8Error> {
        let c_str =
            unsafe { ffi::CStr::from_ptr(signal_c::code_to_string(*self as signal_c::code_t)) };

        Ok(c_str.to_str()?.to_owned())
    }

    pub fn to_constellation(&self) -> Constellation {
        Constellation::from_constellation_t(unsafe {
            signal_c::code_to_constellation(*self as signal_c::code_t)
        })
        .unwrap()
    }

    pub fn signal_count(&self) -> u16 {
        unsafe { signal_c::code_to_sig_count(*self as signal_c::code_t) }
    }

    pub fn chip_count(&self) -> u32 {
        unsafe { signal_c::code_to_chip_count(*self as signal_c::code_t) }
    }

    pub fn chip_rate(&self) -> f64 {
        unsafe { signal_c::code_to_chip_rate(*self as signal_c::code_t) }
    }

    pub fn is_gps(&self) -> bool {
        unsafe { signal_c::is_gps(*self as signal_c::code_t) }
    }

    pub fn is_sbas(&self) -> bool {
        unsafe { signal_c::is_sbas(*self as signal_c::code_t) }
    }

    pub fn is_glo(&self) -> bool {
        unsafe { signal_c::is_glo(*self as signal_c::code_t) }
    }

    pub fn is_bds2(&self) -> bool {
        unsafe { signal_c::is_bds2(*self as signal_c::code_t) }
    }

    pub fn is_gal(&self) -> bool {
        unsafe { signal_c::is_gal(*self as signal_c::code_t) }
    }

    pub fn is_qzss(&self) -> bool {
        unsafe { signal_c::is_qzss(*self as signal_c::code_t) }
    }
}

pub struct GnssSignal(signal_c::gnss_signal_t);

impl GnssSignal {
    pub fn new(sat: u16, code: Code) -> GnssSignal {
        let code = code as signal_c::code_t;
        GnssSignal(signal_c::gnss_signal_t { sat, code })
    }

    pub fn to_constellation(&self) -> Constellation {
        Constellation::from_constellation_t(unsafe { signal_c::sid_to_constellation(self.0) })
            .unwrap()
    }

    pub fn carrier_frequency(&self) -> f64 {
        unsafe { signal_c::sid_to_carr_freq(self.0) }
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
        assert_eq!(Constellation::Bds.sat_count(), 37);
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
