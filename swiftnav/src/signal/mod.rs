// Copyright (c) 2025 Swift Navigation Inc.
// Contact: Swift Navigation <dev@swiftnav.com>
//
// This source is subject to the license found in the file 'LICENSE' which must
// be be distributed together with this source. All other rights reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND,
// EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A PARTICULAR PURPOSE.
//! GNSS Signals and related functionality
//!
//! Signals are specific to a satellite and code combination. A satellite is
//! identified by it's assigned number and the constellation it belongs to. Each
//! satellite can send out multiple signals.
//!
//! This module provides:
//! - [`Constellation`] - Representing the supporting GNSS constellations
//! - [`Code`] - Representing the codes broadcast from the GNSS satellites
//! - [`GnssSignal`] - Represents a [`Code`] broadcast by a specific satellite, using the satellite PRN as the identifier
//!
//! # Examples
//!
//! ```rust
//! # use std::str::FromStr;
//! # use swiftnav::signal::{Code, Constellation, GnssSignal};
//! let sid = GnssSignal::new(22, Code::GpsL1ca).unwrap();
//!
//! assert_eq!(sid.to_constellation(), Constellation::Gps);
//! assert_eq!(sid.to_string(), "GPS L1CA 22");
//!
//! assert_eq!(Constellation::Gal.sat_count(), 36);
//!
//! let code = Code::from_str("BDS3 B1C").unwrap();
//! assert_eq!(code.get_carrier_frequency(), 1575.42e6);
//! ```
mod code;
mod constellation;
pub mod consts;

pub use code::*;
pub use constellation::*;
use std::fmt;

/// GNSS Signal identifier
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct GnssSignal {
    code: Code,
    sat: u16,
}

/// An error encountered when converting an integer into a [`GnssSignal`]
/// and satellite number is not in the valid range for the associated constellation
#[derive(thiserror::Error, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[error("The satellite number is not valid for the associated constellation ({0})")]
pub struct InvalidSatellite(u16);

#[derive(thiserror::Error, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum InvalidGnssSignal {
    #[error("Invalid code")]
    InvalidCode(#[from] InvalidCodeInt),
    #[error("Invalid satellite")]
    InvalidSatellite(#[from] InvalidSatellite),
}

impl GnssSignal {
    /// Make a [`GnssSignal`] from its constituent parts, check for a valid satellite PRN
    ///
    /// The `sat` value is checked to be a valid PRN value for the given constellation
    pub fn new(sat: u16, code: Code) -> Result<GnssSignal, InvalidSatellite> {
        let constellation = code.to_constellation();
        if sat < constellation.first_prn()
            || sat >= (constellation.first_prn() + constellation.sat_count())
        {
            Err(InvalidSatellite(sat))
        } else {
            Ok(GnssSignal { code, sat })
        }
    }

    /// Convert a C `gnss_signal_t` object into a Rust [`GnssSignal`]
    pub(crate) fn from_gnss_signal_t(
        sid: swiftnav_sys::gnss_signal_t,
    ) -> Result<GnssSignal, InvalidGnssSignal> {
        use std::convert::TryInto;

        Ok(Self::new(sid.sat, (sid.code as u8).try_into()?)?)
    }

    /// Convert a Rust [`GnssSignal`] object into a C `gnss_signal_t`
    pub(crate) fn to_gnss_signal_t(self) -> swiftnav_sys::gnss_signal_t {
        swiftnav_sys::gnss_signal_t {
            sat: self.sat,
            code: self.code.to_code_t(),
        }
    }

    /// Get the satellite PRN of the signal
    #[must_use]
    pub fn sat(&self) -> u16 {
        self.sat
    }

    /// Get the [`Code`] of the signal
    #[must_use]
    pub fn code(&self) -> Code {
        self.code
    }

    /// Get the [`Constellation`] of the signal
    #[must_use]
    pub fn to_constellation(self) -> Constellation {
        self.code.to_constellation()
    }

    /// Get the carrier frequency of the signal
    ///
    /// # Note
    ///
    /// GLONASS FDMA codes return the center frequency. To get the channel
    /// frequency use [`GnssSignal::get_glo_channel_frequency()`] instead
    #[must_use]
    pub fn get_carrier_frequency(&self) -> f64 {
        self.code.get_carrier_frequency()
    }

    /// Get the channel frequency for the given GLONASS FDMA code and channel slot
    ///
    /// The code must be either GLO L1OF or L2OF. The slot number must be between -7 and +6
    ///
    /// # Panics
    ///
    /// This function will panic if the code is not a GLONASS FDMA code, or if the channel slot
    /// is invalid.
    pub fn get_glo_channel_frequency(&self, slot: i16) -> f64 {
        self.code.get_glo_channel_frequency(slot)
    }
}

impl fmt::Display for GnssSignal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.code, self.sat)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
                assert_eq!(result.unwrap_err(), InvalidSatellite(sat));
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
                assert_eq!(result.unwrap_err(), InvalidSatellite(sat));
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
                assert_eq!(result.unwrap_err(), InvalidSatellite(sat));
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
                assert_eq!(result.unwrap_err(), InvalidSatellite(sat));
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
                assert_eq!(result.unwrap_err(), InvalidSatellite(sat));
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
                assert_eq!(result.unwrap_err(), InvalidSatellite(sat));
            } else {
                assert!(result.is_ok());
            }
        }
    }

    #[test]
    fn signal_strings() {
        assert_eq!(
            GnssSignal::new(1, Code::GpsL1ca).unwrap().to_string(),
            "GPS L1CA 1"
        );
        assert_eq!(
            GnssSignal::new(32, Code::GpsL1ca).unwrap().to_string(),
            "GPS L1CA 32"
        );
        assert_eq!(
            GnssSignal::new(1, Code::GalE5x).unwrap().to_string(),
            "GAL E5a 1"
        );
        assert_eq!(
            GnssSignal::new(32, Code::GalE5x).unwrap().to_string(),
            "GAL E5a 32"
        );
        assert_eq!(
            GnssSignal::new(1, Code::Bds2B1).unwrap().to_string(),
            "BDS B1 1"
        );
        assert_eq!(
            GnssSignal::new(32, Code::Bds2B1).unwrap().to_string(),
            "BDS B1 32"
        );
    }
}
