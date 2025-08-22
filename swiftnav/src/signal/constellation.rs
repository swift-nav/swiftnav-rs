// Copyright (c) 2025 Swift Navigation Inc.
// Contact: Swift Navigation <dev@swiftnav.com>
//
// This source is subject to the license found in the file 'LICENSE' which must
// be be distributed together with this source. All other rights reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND,
// EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A PARTICULAR PURPOSE.

use super::consts;

/// GNSS satellite constellations
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
#[strum(serialize_all = "UPPERCASE")]
pub enum Constellation {
    /// GPS
    Gps,
    /// SBAS - Space based augmentation systems
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
    #[must_use]
    pub fn sat_count(self) -> u16 {
        match self {
            Constellation::Gps => consts::NUM_SATS_GPS,
            Constellation::Sbas => consts::NUM_SATS_SBAS,
            Constellation::Glo => consts::NUM_SATS_GLO,
            Constellation::Bds => consts::NUM_SATS_BDS,
            Constellation::Gal => consts::NUM_SATS_GAL,
            Constellation::Qzs => consts::NUM_SATS_QZS,
        }
    }

    /// Get the first PRN value used by the constellation
    #[must_use]
    pub fn first_prn(self) -> u16 {
        match self {
            Constellation::Gps => consts::GPS_FIRST_PRN,
            Constellation::Sbas => consts::SBAS_FIRST_PRN,
            Constellation::Glo => consts::GLO_FIRST_PRN,
            Constellation::Bds => consts::BDS_FIRST_PRN,
            Constellation::Gal => consts::GAL_FIRST_PRN,
            Constellation::Qzs => consts::QZS_FIRST_PRN,
        }
    }

    /// Get an iterator through the constellations
    pub fn iter() -> impl Iterator<Item = Self> {
        <Self as strum::IntoEnumIterator>::iter()
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
        Constellation::from_repr(value).ok_or(InvalidConstellationInt(value))
    }
}

/// The character abbreviations used follow the RINEX conventions
impl std::convert::From<Constellation> for char {
    fn from(c: Constellation) -> char {
        match c {
            Constellation::Gps => 'G',
            Constellation::Sbas => 'S',
            Constellation::Glo => 'R',
            Constellation::Bds => 'C',
            Constellation::Gal => 'E',
            Constellation::Qzs => 'J',
        }
    }
}

/// An error encountered when converting an integer into a [`Constellation`]
/// and no constellation is associated with the given value
#[derive(thiserror::Error, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[error("Invalid character for GNSS Constellation ({0})")]
pub struct InvalidConstellationChar(char);

/// The character abbreviations used follow the RINEX conventions
impl std::convert::TryFrom<char> for Constellation {
    type Error = InvalidConstellationChar;

    fn try_from(c: char) -> Result<Constellation, Self::Error> {
        match c {
            'G' => Ok(Constellation::Gps),
            'S' => Ok(Constellation::Sbas),
            'R' => Ok(Constellation::Glo),
            'C' => Ok(Constellation::Bds),
            'E' => Ok(Constellation::Gal),
            'J' => Ok(Constellation::Qzs),
            _ => Err(InvalidConstellationChar(c)),
        }
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
    fn constellation_chars() {
        use std::convert::TryInto;

        assert_eq!('G', Constellation::Gps.into());
        assert_eq!('S', Constellation::Sbas.into());
        assert_eq!('R', Constellation::Glo.into());
        assert_eq!('C', Constellation::Bds.into());
        assert_eq!('J', Constellation::Qzs.into());
        assert_eq!('E', Constellation::Gal.into());

        assert_eq!('G'.try_into(), Ok(Constellation::Gps));
        assert_eq!('S'.try_into(), Ok(Constellation::Sbas));
        assert_eq!('R'.try_into(), Ok(Constellation::Glo));
        assert_eq!('C'.try_into(), Ok(Constellation::Bds));
        assert_eq!('J'.try_into(), Ok(Constellation::Qzs));
        assert_eq!('E'.try_into(), Ok(Constellation::Gal));
    }

    #[test]
    fn constellation_strings() {
        use std::str::FromStr;

        assert_eq!(Constellation::Gps.to_string(), "GPS");
        assert_eq!(Constellation::Sbas.to_string(), "SBAS");
        assert_eq!(Constellation::Glo.to_string(), "GLO");
        assert_eq!(Constellation::Bds.to_string(), "BDS");
        assert_eq!(Constellation::Qzs.to_string(), "QZS");
        assert_eq!(Constellation::Gal.to_string(), "GAL");

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
    fn swiftnav_sys_int_values() {
        for (i, e) in ((swiftnav_sys::constellation_e_CONSTELLATION_INVALID + 1)
            ..(swiftnav_sys::constellation_e_CONSTELLATION_COUNT))
            .zip(Constellation::iter())
        {
            assert_eq!(i, e as i32);
        }
    }
}
