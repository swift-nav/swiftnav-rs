// Copyright (c) 2025 Swift Navigation Inc.
// Contact: Swift Navigation <dev@swiftnav.com>
//
// This source is subject to the license found in the file 'LICENSE' which must
// be be distributed together with this source. All other rights reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND,
// EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A PARTICULAR PURPOSE.

use std::{
    fmt::{self},
    time::Duration,
};

use bon::Builder;
use chrono::{DateTime, Timelike, Utc};

use crate::{
    coords::LLHDegrees,
    nmea::{self, Source},
};

/// Quality of GPS solution
#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub enum GPSQuality {
    /// Fix not available or invalid
    #[default]
    NoFix,
    /// GPS SPS Mode, fix valid
    SPS,
    /// Differential GPS, SPS Mode, fix valid
    DGPS,
    /// GPS PPS (precise positioning service, military encrypted signals), fix valid
    PPS,
    /// RTK (real time kinematic). System used in RTK mode with fixed integers
    RTK,
    /// Float RTK, satelite system used in RTK mode, floating integers
    FRTK,
    /// Estimated (dead reckoning) mode.
    DeadReckoning,
    /// Manual input mode
    Manual,
    /// Simulated mode
    Simulated,
}

impl fmt::Display for GPSQuality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GPSQuality::NoFix => write!(f, "0"),
            GPSQuality::SPS => write!(f, "1"),
            GPSQuality::DGPS => write!(f, "2"),
            GPSQuality::PPS => write!(f, "3"),
            GPSQuality::RTK => write!(f, "4"),
            GPSQuality::FRTK => write!(f, "5"),
            GPSQuality::DeadReckoning => write!(f, "6"),
            GPSQuality::Manual => write!(f, "7"),
            GPSQuality::Simulated => write!(f, "8"),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GGAParseError {
    #[error("Invalid time format")]
    InvalidTimeFormat,

    #[error("Invalid or missing GPS quality")]
    InvalidGPSQuality,
}

/// Global Positioning System Fix Data including time, position and fix related data for a GNSS
/// receiver
#[derive(Debug, PartialEq, Clone, Builder)]
pub struct GGA {
    #[builder(default)]
    pub source: Source,
    /// Time of fix in UTC.
    #[builder(default = Utc::now())]
    pub time: DateTime<Utc>,
    /// Latitude, longitude and height in degrees.
    pub llh: LLHDegrees,
    /// Quality of GPS solution.
    #[builder(default)]
    pub gps_quality: GPSQuality,
    /// Sattelites in use
    pub sat_in_use: Option<u8>,
    /// Horizontal dilusion of presicion
    pub hdop: Option<f32>,
    /// The difference between reference ellipsoid surface and mean-sea-level.
    pub geoidal_separation: Option<f32>,
    /// DGPS data age
    pub age_dgps: Option<Duration>,
    /// ID of reference DGPS station used for fix
    pub reference_station_id: Option<u16>,
}

impl GGA {
    /// converts the GGA struct into an NMEA sentence
    ///
    /// <https://gpsd.gitlab.io/gpsd/NMEA.html#_gga_global_positioning_system_fix_data>
    ///
    /// ```text
    ///        1         2       3 4        5 6 7  8   9  10 |  12 13  14   15
    ///        |         |       | |        | | |  |   |   | |   | |   |    |
    /// $--GGA,hhmmss.ss,ddmm.mm,a,ddmm.mm,a,x,xx,x.x,x.x,M,x.x,M,x.x,xxxx*hh<CR><LF>
    /// ```
    #[must_use]
    pub fn to_sentence(&self) -> String {
        let talker_id = self.source.to_nmea_talker_id();
        // NOTE(ted): We are formatting here a bit strange because for some ungodly reason,
        // chrono chose not to allow for abitrary fractional seconds precision when formatting
        // Construct timestamp in HHMMSS.SS format
        let hour = self.time.hour();
        let minute = self.time.minute();
        let second = f64::from(self.time.second());
        let second_fracs = f64::from(self.time.nanosecond()) / 1_000_000_000.0;

        let timestamp = format!("{hour}{minute}{:.2}", second + second_fracs);

        let (lat_deg, lat_mins) = self.llh.latitude_degree_decimal_minutes();
        let lat_hemisphere = self.llh.latitudinal_hemisphere();

        let (lon_deg, lon_mins) = self.llh.longitude_degree_decimal_minutes();
        let lon_hemisphere = self.llh.longitudinal_hemisphere();

        let gps_quality = self.gps_quality;

        let sat_in_use = self.sat_in_use.map_or(String::new(), |sat| sat.to_string());

        let hdop = self.hdop.map_or(String::new(), |hdop| format!("{hdop:.1}"));

        // NOTE(ted): This is actually not the right value to use, however, we don't really use
        // height for finding information like nearest station so it's ok to use for now
        let height = "0.0";

        let age_dgps = self
            .age_dgps
            .map_or(String::new(), |age| format!("{:.1}", age.as_secs_f64()));

        let geoidal_separation = self
            .geoidal_separation
            .map_or(String::new(), |sep| format!("{sep:.2}"));

        let reference_station_id = self
            .reference_station_id
            .map_or(String::new(), |id| id.to_string());

        let sentence = format!(
            "{talker_id}GGA,{timestamp},{lat_deg:02}{lat_mins:010.7},{lat_hemisphere},{lon_deg:\
             03}{lon_mins:010.7},{lon_hemisphere},{gps_quality},{sat_in_use},{hdop},{height:.6},M,\
             {geoidal_separation},{age_dgps:.1},{reference_station_id}",
        );

        let checksum = nmea::calculate_checksum(&sentence);

        let sentence = format!("${sentence}*{checksum}\r\n");

        sentence
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gga_can_be_turned_into_an_nmea_sentence() {
        let gga = GGA::builder()
            .sat_in_use(12)
            .time(DateTime::from_timestamp(1_761_351_489, 0).unwrap())
            .gps_quality(GPSQuality::SPS)
            .hdop(0.9)
            .llh(super::LLHDegrees::new(37.7749, -122.4194, 10.0))
            .build();

        let sentence = gga.to_sentence();

        assert_eq!(
            sentence,
            "$GPGGA,0189.00,3746.4940000,N,12225.1640000,W,1,12,0.9,0.0,M,,,*01\r\n"
        );
    }

    #[test]
    fn gga_with_dgps_can_be_turned_into_an_nmea_sentence() {
        let gga = GGA::builder()
            .sat_in_use(8)
            .time(DateTime::from_timestamp(1_761_351_489, 0).unwrap())
            .hdop(1.2)
            .llh(super::LLHDegrees::new(34.0522, -18.2437, 15.0))
            .gps_quality(GPSQuality::DGPS)
            .age_dgps(Duration::from_secs_f64(2.5))
            .geoidal_separation(1.0)
            .reference_station_id(42)
            .build();

        let sentence = gga.to_sentence();

        assert_eq!(
            sentence,
            "$GPGGA,0189.00,3403.1320000,N,01814.6220000,W,2,8,1.2,0.0,M,1.00,2,42*1C\r\n"
        );
    }

    #[test]
    fn gga_sentence_is_always_less_than_82_characters() {
        // we are going to set some very large decimal places and the highest possible values in
        // terms of character count to ensure our sentence is always below 82 characters
        let gga = GGA::builder()
            .sat_in_use(12)
            .time(DateTime::from_timestamp(1_761_351_489, 0).unwrap())
            .hdop(1.210_123_1)
            .llh(super::LLHDegrees::new(
                -90.000_000_001,
                -180.000_000_000_1,
                1_000.000_000_000,
            ))
            .gps_quality(GPSQuality::DGPS)
            .age_dgps(Duration::from_secs_f64(2.500_000_000_001))
            .geoidal_separation(1.00)
            .reference_station_id(1023) // 1023 is the max value for a 4 digit station ID
            .build();

        let sentence = gga.to_sentence();

        assert!(sentence.len() < 82);
    }
}
