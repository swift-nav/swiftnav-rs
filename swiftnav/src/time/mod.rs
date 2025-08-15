//! Time handling
//!
//! GPS time counts the number of seconds since Midnight Jan 8th 1980 UTC. Leap
//! seconds are not counted, so there is an offset between UTC and GPS time. GPS
//! time is usually represented as a week number, counting the number of elapsed
//! weeks since the start of GPS time, and a time of week counting the number of
//! seconds since the beginning of the week. In GPS time the week begins at
//! midnight on Sunday.
//!
//! [`GpsTime`] is the primary representation used in swiftnav. Other time bases
//! are available, such as [`UtcTime`], [`GalTime`], and [`BdsTime`]
//! along with conversions for all of these to and from [`GpsTime`].
//! Not all functionality is available in these other representations, so it's
//! intended that all times are to converted to [`GpsTime`] before use with
//! swiftnav.
//!
//! # ⚠️  🦘  ⏱  ⚠️  - Leap Seconds
//! UTC time occasinally adds additional seconds to keep it synchronized with the
//! slowly changing spin of the earth. This complicates time keeping, so most
//! GNSS time bases ignore leap seconds and thus slowly grow out of sync with UTC.
//! This is fine when dealing with GNSS data, but it's common that people want time
//! to be represented as a UTC time since that's what people are more familiar with.
//! swiftnav provides ways to convert to and from UTC synchronized time bases
//! and is able to correctly compensate for leap seconds in two ways.
//!
//! The first is by using the UTC conversion parameters broadcast by GNSS systems
//! that receivers can decode. [`UtcParams`] is how swiftnav represents this
//! information. This is the prefered method since it  is usually available when
//! processing raw GNSS data and ensures that the right  offset is applied at the
//! right time.
//!
//! The second way is to use a table of historical leap seconds that is compiled
//! in to swftnav-rs. This list is kept up to date in the source code as new leap
//! seconds are announced, but once the code is compiled there is no way to update
//! this table with new leap seconds. This obviously means that sooner or later
//! the hard coded leap second information will become out of date and the
//! converted times will be inaccurate. This is fine if you are processing
//! historical data, but processing live data runs the risk of an incorrect time
//! conversion.
//!
//! When converting to or from a time base that uses leap seconds (e.g. [`UtcTime`])
//! two functions are always provided, one which takes a  [`UtcParams`] object to
//! handle the leap second conversion and one which doesn't take a [`UtcParams`]
//! object but has `_hardcoded` appended to the function name.

use std::time::Duration;

mod gnss;
mod mjd;
mod utc;

pub use gnss::*;
pub use mjd::*;
pub use utc::*;

/// Common constant values related to time manipulation
pub mod consts {
/// Number of days in a common (non-leap) year.
pub const YEAR_DAYS: u32 = 365;

/// Number of days in a leap year.
pub const LEAP_YEAR_DAYS: u32 = YEAR_DAYS + 1;

/// Number of days in a week.
pub const WEEK_DAYS: u32 = 7;

/// Number of months in a year.
pub const YEAR_MONTHS: u32 = 12;

/// Days in (leap) year 1980 since GPS epoch Jan 6th
pub const YEAR_1980_GPS_DAYS: u32 = 361;

/// Year of GPS epoch
pub const GPS_EPOCH_YEAR: u32 = 1980;

// /// UTC (SU) offset (hours)
// pub const UTC_SU_OFFSET: u32 = 3;

/// Number of seconds in a minute.
pub const MINUTE_SECS: u32 = 60;

/// Number of minutes in an hour.
pub const HOUR_MINUTES: u32 = 60;

/// Number of seconds in an hour.
pub const HOUR_SECS: u32 = MINUTE_SECS * HOUR_MINUTES;

/// Number of hours in a day.
pub const DAY_HOURS: u32 = 24;

/// Number of seconds in a day.
pub const DAY_SECS: u32 =  DAY_HOURS * HOUR_MINUTES * MINUTE_SECS;

/// Number of seconds in a week. 
pub const WEEK_SECS: u32 = WEEK_DAYS * DAY_SECS;

/// Number of nanoseconds in a second. 
pub const SECS_NS: u32 = 1_000_000_000;

/// Number of microseconds in a second. 
pub const SECS_US: u32 = 1_000_000;

/// Number of milliseconds in a second. 
pub const SECS_MS: u32 = 1_000;

/// Number of milliseconds in a week 
pub const WEEK_MS: u32 = SECS_MS * WEEK_SECS;

/// Number of days in four years. 
pub const FOUR_YEARS_DAYS: u32 = 3 * YEAR_DAYS + LEAP_YEAR_DAYS;

/// Number of days in 100 years. 
pub const HUNDRED_YEARS_DAYS: u32 = 24 * FOUR_YEARS_DAYS + 4 * YEAR_DAYS;

/// Number of days in 400 years. 
pub const FOUR_HUNDRED_YEARS_DAYS: u32 = 3 * HUNDRED_YEARS_DAYS + 25 * FOUR_YEARS_DAYS;

/** Number of rollovers in the 10-bit broadcast GPS week number.
 * Update on next rollover on April 7, 2019.
 * \todo Detect and handle rollover more gracefully. */
pub const GPS_WEEK_CYCLE: u16 = 1;

/** The GPS week reference number. The current GPS WN is always assumed to be
 * later than this reference number. It will keep the WN calculated from the
 * truncated 10-bit broadcast WN valid for ~20 years after this week.
 *
 * Current week number is set to 20 December 2015.
 *
 * TODO: update this from build date */
pub const GPS_WEEK_REFERENCE: i16 = 1876;

/** The GPS week number at which we won't be able to figure out what
    time it is with the current reference. */
pub const GPS_MAX_WEEK: i16 = 2899;

/// Unix timestamp of the GPS epoch 1980-01-06 00:00:00 UTC 
pub const GPS_EPOCH: i64 = 315964800;

/// Modified Julian days of the GPS epoch 1980-01-06 00:00:00 UTC 
pub const MJD_JAN_6_1980: i32 = 44244;

/// Modified Julian days of 1601-01-01 
pub const MJD_JAN_1_1601: i32 = -94187;

/// Constant difference of Galileo time from GPS time 
pub const GAL_WEEK_TO_GPS_WEEK: i16 = 1024;
pub const GAL_SECOND_TO_GPS_SECOND: f64 = 0.0;

/// Constant difference of Beidou time from GPS time 
pub const BDS_WEEK_TO_GPS_WEEK: i16 = 1356;
pub const BDS_SECOND_TO_GPS_SECOND: f64 = 14.0;
}

/// A minute long [`Duration`]
pub const MINUTE: Duration = Duration::from_secs(consts::MINUTE_SECS as u64);
/// An hour long [`Duration`]
pub const HOUR: Duration = Duration::from_secs(consts::HOUR_SECS as u64);
/// A day long [`Duration`]
pub const DAY: Duration = Duration::from_secs(consts::DAY_SECS as u64);
/// A week long [`Duration`]
pub const WEEK: Duration = Duration::from_secs(consts::WEEK_SECS as u64);

/// Checks to see if a year is a leap year
/// 
/// # Note
/// 
/// All year values are treated as if they are in the Gregorian calendar
pub fn is_leap_year(year: u16) -> bool {
    ((year % 4 == 0) && (year % 100 != 0)) || (year % 400 == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_leap_year() {
        use super::is_leap_year;
        assert!(!is_leap_year(2019));
        assert!(is_leap_year(2020));
        assert!(!is_leap_year(1900));
        assert!(is_leap_year(2000));
    }

    #[test]
    fn conversions() {
        const TEST_CASES: [GpsTime; 10] = [
            GpsTime::new_unchecked(1234, 567890.0),
            GpsTime::new_unchecked(1234, 567890.5),
            GpsTime::new_unchecked(1234, 567890.0),
            GpsTime::new_unchecked(1234, 0.0),
            GpsTime::new_unchecked(1000, 604578.0),
            GpsTime::new_unchecked(1001, 222.222),
            GpsTime::new_unchecked(1001, 604578.0),
            GpsTime::new_unchecked(1939, 222.222),
            GpsTime::new_unchecked(1930, 16.0),
            GpsTime::new_unchecked(1930, 18.0), /* around Jan 2017 leap second */
        ];

        const TOW_TOL: f64 = 1e-6;

        for test_case in TEST_CASES {
            // test gps -> mjd -> gps
            let mjd = test_case.to_mjd_hardcoded();
            let round_trip = mjd.to_gps_hardcoded();
            let diff = test_case.diff(&round_trip).abs();
            assert!(diff < TOW_TOL, "gps2mjd2gps failure. original: {:?}, round trip: {:?}, diff: {}, TOW_TOL: {}", test_case, round_trip, diff, TOW_TOL);

            // test mjd -> date -> mjd
            let (year, month, day, hour, minute, second) = mjd.to_date();
            let round_trip = MJD::from_date(year, month, day, hour, minute, second);
            let diff = (mjd.as_f64() - round_trip.as_f64()).abs();
            assert!(diff < TOW_TOL, "mjd2date2mjd failure. original: {:?}, round trip: {:?}, diff: {}, TOW_TOL: {}", mjd, round_trip, diff, TOW_TOL);

            // test mjd -> utc -> mjd
            let utc = mjd.to_utc();
            let round_trip = utc.to_mjd();
            let diff = (mjd.as_f64() - round_trip.as_f64()).abs();
            assert!(diff < TOW_TOL, "mjd2utc2mjd failure. original: {:?}, round trip: {:?}, diff: {}, TOW_TOL: {}", mjd, round_trip, diff, TOW_TOL);


            // test gps -> date -> gps
            let (year, month, day, hour, minute, second) = test_case.to_date_hardcoded();
            let round_trip = GpsTime::from_date_hardcoded(year, month, day, hour, minute, second);
            let diff = test_case.diff(&round_trip).abs();
            assert!(diff < TOW_TOL, "gps2date2gps failure. original: {:?}, round trip: {:?}, diff: {}, TOW_TOL: {}", test_case, round_trip, diff, TOW_TOL);

            // test utc -> date -> utc
            let (year, month, day, hour, minute, second) = utc.to_date();
            let round_trip = UtcTime::from_date(year, month, day, hour, minute, second);
            let diff = utc.to_mjd().as_f64() - mjd.as_f64();
            assert!(diff < TOW_TOL, "utc2date2utc failure. original: {:?}, round trip: {:?}, diff: {}, TOW_TOL: {}", mjd, round_trip, diff, TOW_TOL);
        }
    }
}
