// Copyright (c) 2025 Swift Navigation Inc.
// Contact: Swift Navigation <dev@swiftnav.com>
//
// This source is subject to the license found in the file 'LICENSE' which must
// be be distributed together with this source. All other rights reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND,
// EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A PARTICULAR PURPOSE.
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

pub mod consts;
mod gnss;
mod mjd;
mod utc;

pub use gnss::*;
pub use mjd::*;
pub use utc::*;

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
#[must_use]
pub fn is_leap_year(year: u16) -> bool {
    (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400)
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
            GpsTime::new_unchecked(1234, 567_890.0),
            GpsTime::new_unchecked(1234, 567_890.5),
            GpsTime::new_unchecked(1234, 567_890.0),
            GpsTime::new_unchecked(1234, 0.0),
            GpsTime::new_unchecked(1000, 604_578.0),
            GpsTime::new_unchecked(1001, 222.222),
            GpsTime::new_unchecked(1001, 604_578.0),
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
            assert!(
                diff < TOW_TOL,
                "gps2mjd2gps failure. original: {test_case:?}, round trip: {round_trip:?}, diff: \
                 {diff}, TOW_TOL: {TOW_TOL}"
            );

            // test mjd -> date -> mjd
            let (year, month, day, hour, minute, second) = mjd.to_date();
            let round_trip = MJD::from_parts(year, month, day, hour, minute, second);
            let diff = (mjd.as_f64() - round_trip.as_f64()).abs();
            assert!(
                diff < TOW_TOL,
                "mjd2date2mjd failure. original: {mjd:?}, round trip: {round_trip:?}, diff: \
                 {diff}, TOW_TOL: {TOW_TOL}"
            );

            // test mjd -> utc -> mjd
            let utc = mjd.to_utc();
            let round_trip = utc.to_mjd();
            let diff = (mjd.as_f64() - round_trip.as_f64()).abs();
            assert!(
                diff < TOW_TOL,
                "mjd2utc2mjd failure. original: {mjd:?}, round trip: {round_trip:?}, diff: \
                 {diff}, TOW_TOL: {TOW_TOL}"
            );

            // test gps -> date -> gps
            let (year, month, day, hour, minute, second) = test_case.to_date_hardcoded();
            let round_trip = GpsTime::from_parts_hardcoded(year, month, day, hour, minute, second);
            let diff = test_case.diff(&round_trip).abs();
            assert!(
                diff < TOW_TOL,
                "gps2date2gps failure. original: {test_case:?}, round trip: {round_trip:?}, diff: \
                 {diff}, TOW_TOL: {TOW_TOL}"
            );

            // test utc -> date -> utc
            let (year, month, day, hour, minute, second) = utc.to_date();
            let round_trip = UtcTime::from_parts(year, month, day, hour, minute, second);
            let diff = utc.to_mjd().as_f64() - mjd.as_f64();
            assert!(
                diff < TOW_TOL,
                "utc2date2utc failure. original: {mjd:?}, round trip: {round_trip:?}, diff: \
                 {diff}, TOW_TOL: {TOW_TOL}"
            );
        }
    }
}
