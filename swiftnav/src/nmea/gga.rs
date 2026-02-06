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
    fmt::{self, Write},
    time::Duration,
};

use bon::Builder;
use chrono::{DateTime, Timelike, Utc};

use crate::{
    coords::LLHDegrees,
    nmea::{Source, calculate_checksum},
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
    /// If true, enforces the NMEA 0183 82-character limit by truncating coordinate precision.
    #[builder(default = true)]
    pub strict: bool,
}

impl GGA {
    /// Converts the GGA struct into an NMEA sentence.
    ///
    /// In strict mode, coordinate decimal places are dynamically reduced to
    /// stay within the 82-character NMEA 0183 limit. In non-strict mode,
    /// 7 decimal places (~1.8mm resolution) are always used.
    #[must_use]
    pub fn to_sentence(&self) -> String {
        let talker_id = self.source.to_nmea_talker_id();

        // Format the tail first so we can figure out how many decimal places we can use
        let mut tail = String::with_capacity(32);
        self.write_tail(&mut tail);

        let dp = if self.strict {
            // Fixed overhead (everything except tail and lat/lon minute decimals):
            //   $ (1) + talker_id + GGA (3) + timestamp (9) + 6 head commas (6)
            //   + lat DD (2) + lat MM (2) + hemisphere (1)
            //   + lon DDD (3) + lon MM (2) + hemisphere (1)
            //   + * (1) + checksum (2) + \r\n (2)
            let base_len = talker_id.len() + 34 + tail.len();
            let remaining = 82usize.saturating_sub(base_len);
            if remaining >= 4 {
                ((remaining - 2) / 2).min(7)
            } else {
                0
            }
        } else {
            7
        };
        let w = if dp > 0 { 3 + dp } else { 2 };

        let seconds = f64::from(self.time.second()) + f64::from(self.time.nanosecond()) / 1e9;
        let (lat_deg, lat_mins) = self.llh.latitude_degree_decimal_minutes();
        let (lon_deg, lon_mins) = self.llh.longitude_degree_decimal_minutes();

        let mut sentence = String::with_capacity(88);
        sentence.push('$');
        write!(
            sentence,
            "{talker_id}GGA,{:02}{:02}{:05.2},{lat_deg:02}{lat_mins:0w$.dp$},{},{lon_deg:\
             03}{lon_mins:0w$.dp$},{}",
            self.time.hour(),
            self.time.minute(),
            seconds,
            self.llh.latitudinal_hemisphere(),
            self.llh.longitudinal_hemisphere(),
            dp = dp,
        )
        .unwrap();
        sentence.push_str(&tail);

        let checksum = calculate_checksum(&sentence);
        write!(sentence, "*{checksum:02X}\r\n").unwrap();
        sentence
    }

    /// Writes the variable-width tail fields (everything after the lon hemisphere).
    fn write_tail(&self, w: &mut impl fmt::Write) {
        write!(w, ",{},", self.gps_quality).unwrap();
        if let Some(sat) = self.sat_in_use {
            write!(w, "{sat}").unwrap();
        }
        write!(w, ",").unwrap();
        if let Some(h) = self.hdop {
            write!(w, "{h:.1}").unwrap();
        }
        // NOTE(ted): This is actually not the right value to use, however, we don't really use
        // height for finding information like nearest station so it's ok to use for now
        write!(w, ",0.0,M,").unwrap();
        if let Some(sep) = self.geoidal_separation {
            write!(w, "{sep:.1}").unwrap();
        }
        write!(w, ",M,").unwrap();
        if let Some(age) = self.age_dgps {
            write!(w, "{:.1}", age.as_secs_f64()).unwrap();
        }
        write!(w, ",").unwrap();
        if let Some(id) = self.reference_station_id {
            write!(w, "{id}").unwrap();
        }
    }
}

#[cfg(test)]
mod test {
    use proptest::prelude::*;

    use super::*;

    const ALL_QUALITIES: [GPSQuality; 9] = [
        GPSQuality::NoFix,
        GPSQuality::SPS,
        GPSQuality::DGPS,
        GPSQuality::PPS,
        GPSQuality::RTK,
        GPSQuality::FRTK,
        GPSQuality::DeadReckoning,
        GPSQuality::Manual,
        GPSQuality::Simulated,
    ];

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]

        #[test]
        fn gga_sentence_can_be_parsed_by_nmea_crate(
            sat_in_use in 0u8..=99,
            timestamp in 0i64..=4_102_444_800,
            hdop in 0.0f32..=99.9,
            lat in -90.0f64..=90.0,
            lon in -180.0f64..=180.0,
            height in -1000.0f64..=100_000.0,
            gps_quality in 0usize..=8,
            age_dgps in proptest::option::of(0.0f64..=999.9),
            geoidal_separation in proptest::option::of(-999.9f32..=999.9),
            reference_station_id in proptest::option::of(0u16..=1023),
            strict in proptest::bool::ANY,
        ) {

            let gga = GGA {
                source: Source::default(),
                time: DateTime::from_timestamp(timestamp, 0).unwrap(),
                llh: LLHDegrees::new(lat, lon, height),
                gps_quality: ALL_QUALITIES[gps_quality],
                sat_in_use: Some(sat_in_use),
                hdop: Some(hdop),
                geoidal_separation,
                age_dgps: age_dgps.map(Duration::from_secs_f64),
                reference_station_id,
                strict,
            };

            let sentence = gga.to_sentence();

            let parse_result = ::nmea::parse_str(&sentence);
            prop_assert!(parse_result.is_ok(), "Failed to parse: {sentence}");

            let ::nmea::ParseResult::GGA(parsed) = parse_result.unwrap() else {
                prop_assert!(false, "Parsed result is not GGA");
                unreachable!();
            };

            let parsed_lat = parsed.latitude.unwrap();
            let parsed_lon = parsed.longitude.unwrap();

            // Lat/lon minute precision varies dynamically (4-7 decimal places) to
            // fit within the 82-char NMEA limit. At worst case (4dp), the max
            // formatting error is ~8.3e-7 degrees.
            prop_assert!(
                (parsed_lat - lat).abs() < 1e-5,
                "Latitude mismatch: expected {lat}, got {parsed_lat}",
            );
            prop_assert!(
                (parsed_lon - lon).abs() < 1e-5,
                "Longitude mismatch: expected {lon}, got {parsed_lon}",
            );
        }

        #[test]
        fn strict_gga_sentence_is_always_less_than_or_equal_to_82_characters(
            sat_in_use in proptest::option::of(0u8..=99),
            timestamp in 0i64..=4_102_444_800,
            nanosecond in 0u32..=999_999_999,
            hdop in proptest::option::of(0.0f32..=99.9),
            lat in -90.0f64..=90.0,
            lon in -180.0f64..=180.0,
            height in -1000.0f64..=100_000.0,
            gps_quality in 0usize..=8,
            age_dgps in proptest::option::of(0.0f64..=999.9),
            geoidal_separation in proptest::option::of(-999.9f32..=999.9),
            reference_station_id in proptest::option::of(0u16..=1023),
        ) {
            let gga = GGA {
                source: Source::default(),
                time: DateTime::from_timestamp(timestamp, nanosecond).unwrap(),
                llh: LLHDegrees::new(lat, lon, height),
                gps_quality: ALL_QUALITIES[gps_quality],
                sat_in_use,
                hdop,
                geoidal_separation,
                age_dgps: age_dgps.map(Duration::from_secs_f64),
                reference_station_id,
                strict: true,
            };

            let sentence = gga.to_sentence();

            prop_assert!(
                sentence.len() <= 82,
                "Sentence length {} exceeds 82 characters: {}", sentence.len(), sentence
            );
        }

        #[test]
        fn gga_sentence_is_always_less_than_or_equal_to_88_characters(
            sat_in_use in proptest::option::of(0u8..=99),
            timestamp in 0i64..=4_102_444_800,
            nanosecond in 0u32..=999_999_999,
            hdop in proptest::option::of(0.0f32..=99.9),
            lat in -90.0f64..=90.0,
            lon in -180.0f64..=180.0,
            height in -1000.0f64..=100_000.0,
            gps_quality in 0usize..=8,
            age_dgps in proptest::option::of(0.0f64..=999.9),
            geoidal_separation in proptest::option::of(-999.9f32..=999.9),
            reference_station_id in proptest::option::of(0u16..=1023),
        ) {
            let gga = GGA {
                source: Source::default(),
                time: DateTime::from_timestamp(timestamp, nanosecond).unwrap(),
                llh: LLHDegrees::new(lat, lon, height),
                gps_quality: ALL_QUALITIES[gps_quality],
                sat_in_use,
                hdop,
                geoidal_separation,
                age_dgps: age_dgps.map(Duration::from_secs_f64),
                reference_station_id,
                strict: false,
            };

            let sentence = gga.to_sentence();

            prop_assert!(
                sentence.len() <= 88,
                "Sentence length {} exceeds 88 characters: {}", sentence.len(), sentence
            );
        }
    }
}
