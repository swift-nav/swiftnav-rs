use std::{
    fmt::{self},
    time::Duration,
};

use bon::Builder;
use chrono::{DateTime, Timelike, Utc};

use crate::{coords::LLHDegrees, nmea::Source};

/// Quality of GPS solution
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GPSQuality {
    /// Fix not available or invalid
    NoFix,
    /// GPS SPS Mode, fix valid
    GPS,
    /// Differential GPS, SPS Mode, fix valid
    DGPS,
    /// GPS PPS (pulse per second), fix valid
    PPS,
    /// RTK (real time kinematic). System used in RTK mode with fixed integers
    RTK,
    /// Float RTK, satelite system used in RTK mode, floating integers
    FRTK,
    /// Estimated (dead reckoning) mode.
    Estimated,
    /// Manual input mode
    Manual,
    /// Simulated mode
    Simulated,
}

impl fmt::Display for GPSQuality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GPSQuality::NoFix => write!(f, "0"),
            GPSQuality::GPS => write!(f, "1"),
            GPSQuality::DGPS => write!(f, "2"),
            GPSQuality::PPS => write!(f, "3"),
            GPSQuality::RTK => write!(f, "4"),
            GPSQuality::FRTK => write!(f, "5"),
            GPSQuality::Estimated => write!(f, "6"),
            GPSQuality::Manual => write!(f, "7"),
            GPSQuality::Simulated => write!(f, "8"),
        }
    }
}

/// Geographic coordinates including altitude, GPS solution quality, DGPS usage information.
#[derive(Debug, PartialEq, Clone, Builder)]
pub struct GGA {
    /// Navigational system.
    #[builder(default = Source::GPS)]
    pub source: Source,
    /// Time of fix in UTC.
    #[builder(default = Utc::now())]
    pub time: DateTime<Utc>,
    /// Latitude, longitude and height in degrees.
    pub llh: LLHDegrees,
    /// Quality of GPS solution.
    #[builder(default = GPSQuality::GPS)]
    pub gps_quality: GPSQuality,
    /// Sattelites in use
    pub sat_in_use: u8,
    /// Horizontal dilusion of presicion
    pub hdop: f32,
    /// The difference between reference ellipsoid surface and mean-sea-level.
    pub geoidal_separation: Option<f32>,
    /// DGPS data age. None if DGPS not in use.
    pub age_dgps: Option<Duration>,
    /// ID of reference DGPS station used for fix. None if DGPS not in use.
    pub dgps_station_id: Option<u16>,
}

impl GGA {
    // converts the GGA struct into an NMEA sentence
    #[must_use]
    pub fn to_sentence(&self) -> String {
        // NOTE(ted): We are formatting here a bit strange because for some ungodly reason,
        // chrono chose not to allow for abitrary fractional seconds precision when formatting
        // Construct timestamp in HHMMSS.SS format
        let hour = self.time.hour();
        let minute = self.time.minute();
        let second = f64::from(self.time.second());
        let second_fracs = f64::from(self.time.nanosecond()) / 1_000_000_000.0;

        let timestamp = format!("{hour}{minute}{:.2}", second + second_fracs);

        let latitude = self.llh.latitude();
        let latitudinal_hemisphere = self.llh.latitudinal_hemisphere();

        let longitude = self.llh.longitude();
        let longitudinal_hemisphere = self.llh.longitudinal_hemisphere();

        let gps_quality = self.gps_quality;

        let sat_in_use = self.sat_in_use;

        let hdop = self.hdop;

        // NOTE(ted): This is actually not the right value to use, however, we don't really use height for finding information like nearest station so it's ok to use for now
        let height = "0.0";

        // if DGPS is not used, this should be a null field
        let age_dgps = if matches!(gps_quality, GPSQuality::DGPS) {
            let age = self.age_dgps.map_or(0.0, |age| age.as_secs_f64());

            format!("{age:.1}")
        } else {
            String::new()
        };

        let geoidal_separation = self
            .geoidal_separation
            .map_or(String::new(), |sep| format!("{sep:.2}"));

        let dgps_station_id = self
            .dgps_station_id
            .map_or(String::new(), |id| id.to_string());

        let sentence = format!(
            "$GPGGA,{timestamp},{latitude:.6},{latitudinal_hemisphere},{longitude:.6},{longitudinal_hemisphere},{gps_quality},{sat_in_use},{hdop:.1},{height:.6},M,{geoidal_separation},{age_dgps:.1},{dgps_station_id}",
        );

        // NOTE(ted): We should skip the first character in the sentence (the '$')
        // https://forum.arduino.cc/t/nmea-checksums-explained/1046083
        let checksum = nmea_checksum(&sentence[1..]);

        format!("{sentence}*{checksum}\r\n")
    }
}

fn u8_to_nibbles(byte: u8) -> (u8, u8) {
    // The high nibble is obtained by shifting the byte 4 bits to the right.
    // This discards the lower 4 bits and moves the upper 4 bits into the lower 4 bit positions.
    let high_nibble = byte >> 4;

    // The low nibble is obtained by masking the byte with 0x0F (binary 0000_1111).
    // This keeps only the lower 4 bits and sets the upper 4 bits to zero.
    let low_nibble = byte & 0x0F;

    (high_nibble, low_nibble)
}

/// Convert a nibble (4 bits) to its ASCII character representation
fn u8_to_ascii_char(nibble: u8) -> char {
    // if we are between 0 and 9, we map to '0' to '9'
    if nibble <= 0x9 {
        (nibble + b'0') as char
    // else, we map to 'A' to 'F'
    } else {
        (nibble - 10 + b'A') as char
    }
}

fn nmea_checksum(sentence: &str) -> String {
    let mut checksum = 0;
    for byte in sentence.bytes() {
        checksum ^= byte;
    }

    let (nibble1, nibble2) = u8_to_nibbles(checksum);

    let char1 = u8_to_ascii_char(nibble1);
    let char2 = u8_to_ascii_char(nibble2);

    format!("{char1}{char2}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gga_can_be_turned_into_an_nmea_sentence() {
        let gga = GGA::builder()
            .sat_in_use(12)
            .time(DateTime::from_timestamp(1_761_351_489, 0).unwrap())
            .hdop(0.9)
            .llh(super::LLHDegrees::new(37.7749, -122.4194, 10.0))
            .build();

        let sentence = gga.to_sentence();

        assert_eq!(
            sentence,
            "$GPGGA,0189.00,37.774900,N,-122.419400,W,1,12,0.9,0.0,M,,,*26\r\n"
        );
    }
}
