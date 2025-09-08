// Copyright (c) 2025 Swift Navigation Inc.
// Contact: Swift Navigation <dev@swiftnav.com>
//
// This source is subject to the license found in the file 'LICENSE' which must
// be be distributed together with this source. All other rights reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND,
// EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A PARTICULAR PURPOSE.

#![expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    reason = "We need to review the math for overflows")]

use crate::time::{consts, GpsTime, UtcParams, UtcTime};
use std::time::Duration;

/// Representation of modified julian dates (MJD)
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct MJD(f64);

impl MJD {
    /// Creates a modified julian date from a floating point representation
    ///
    /// # Panics
    ///
    /// Will panic if the given value is not finite
    #[must_use]
    pub fn from_f64(value: f64) -> Self {
        assert!(value.is_finite());
        Self(value)
    }

    /// Creates a modified julian date from a calendar date and time
    ///
    /// Taken with permission from <http://www.leapsecond.com/tools/gpsdate.c>
    ///
    /// - Valid for Gregorian dates from 17-Nov-1858.
    /// - Adapted from sci.astro FAQ.
    ///
    /// # Note
    ///
    /// This function will be inaccurate by up to a second on the day of a leap second
    ///
    /// # Panics
    ///
    /// This function will panic if given a date before 17-Nov-1858.
    #[must_use]
    pub fn from_parts(year: u16, month: u8, day: u8, hour: u8, minute: u8, seconds: f64) -> MJD {
        // Enforce our assumption that the date is just the MJD period
        assert!(year > 1858 || (year == 1858 && month > 11) || (year == 1858 && month == 11 && day >= 17), "Attempting to convert a date prior to the start of the Modified Julian Date system ({}-{}-{}T{}:{}:{}Z", year, month, day, hour, minute, seconds);

        let full_days = 367 * i64::from(year)
            - 7 * (i64::from(year) + (i64::from(month) + 9) / 12) / 4
            - 3 * ((i64::from(year) + (i64::from(month) - 9) / 7) / 100 + 1) / 4
            + 275 * i64::from(month) / 9
            + i64::from(day)
            + 1_721_028
            - 2_400_000;
        let frac_days = f64::from(hour) / f64::from(consts::DAY_HOURS)
            + f64::from(minute) / f64::from(consts::DAY_HOURS * consts::HOUR_MINUTES)
            + seconds / f64::from(consts::DAY_SECS);
        MJD(full_days as f64 + frac_days)
    }

    /// Gets the floating point value of the modified julian date
    #[must_use]
    pub fn as_f64(&self) -> f64 {
        self.0
    }

    pub(super) fn to_gps_internal(self, params: Option<&UtcParams>) -> GpsTime {
        let utc_days: f64 = self.0 - f64::from(consts::MJD_JAN_6_1980);

        let wn = (utc_days / f64::from(consts::WEEK_DAYS)) as i16;
        let tow =
            (utc_days - f64::from(wn) * f64::from(consts::WEEK_DAYS)) * f64::from(consts::DAY_SECS);
        let utc_time = GpsTime::new_unchecked(wn, tow);

        let leap_secs = params.map_or_else(
            || utc_time.utc_gps_offset_hardcoded(),
            |p| utc_time.utc_gps_offset(p),
        );

        let gps_time = if leap_secs >= 0.0 {
            utc_time - Duration::from_secs_f64(leap_secs)
        } else {
            utc_time + Duration::from_secs_f64(-leap_secs)
        };

        assert!(gps_time.is_valid());
        gps_time
    }

    /// Converts the [`MJD`] into a [`GpsTime`]
    ///
    /// # Panics
    ///
    /// This function will panic if the [`MJD`] does not represent a valid GPS Time
    #[must_use]
    pub fn to_gps(self, utc_params: &UtcParams) -> GpsTime {
        self.to_gps_internal(Some(utc_params))
    }

    /// Converts the [`MJD`] into a [`GpsTime`] using a hard coded list of leap
    /// seconds
    ///
    /// # Panics
    ///
    /// This function will panic if the [`MJD`] does not represent a valid GPS Time
    ///
    /// # ⚠️  🦘  ⏱  ⚠️  - Leap Seconds
    ///
    /// The hard coded list of leap seconds will get out of date, it is
    /// preferable to use [`MJD::to_gps()`] with the newest
    /// set of UTC parameters
    #[must_use]
    pub fn to_gps_hardcoded(self) -> GpsTime {
        self.to_gps_internal(None)
    }

    /// Converts the modified julian date into a UTC time
    #[must_use]
    pub fn to_utc(self) -> UtcTime {
        let utc_days: f64 = self.0 - f64::from(consts::MJD_JAN_6_1980);

        let wn = (utc_days / f64::from(consts::WEEK_DAYS)) as i16;
        let tow =
            (utc_days - f64::from(wn as u32 * consts::WEEK_DAYS)) * f64::from(consts::DAY_SECS);
        let utc_time = GpsTime::new_unchecked(wn, tow);
        UtcTime::from_gps_no_leap(utc_time)
    }

    /// Convert Modified Julian Day to calendar date.
    /// - Assumes Gregorian calendar.
    /// - Adapted from Fliegel/van Flandern ACM 11/#10 p 657 Oct 1968.
    ///
    /// Taken with permission from <http://www.leapsecond.com/tools/gpsdate.c>
    ///
    /// # Note
    ///
    /// This function will be inaccurate by up to a second on the day of a leap
    /// second.
    #[must_use]
    pub fn to_date(self) -> (u16, u8, u8, u8, u8, f64) {
        let j = (self.0 as i32) + 2_400_001 + 68569;
        let c = 4 * j / 146_097;
        let j = j - (146_097 * c + 3) / 4;
        let y = 4000 * (j + 1) / 1_461_001;
        let j = j - 1461 * y / 4 + 31;
        let m = 80 * j / 2447;
        let day: u8 = (j - 2447 * m / 80) as u8;
        let j = m / 11;
        let month: u8 = (m + 2 - (12 * j)) as u8;
        let year: u16 = (100 * (c - 49) + y + j) as u16;
        let frac_part = self.0.fract();
        let hour: u8 = (frac_part * f64::from(consts::DAY_HOURS)) as u8;
        let min: u8 = ((frac_part - f64::from(hour) / f64::from(consts::DAY_HOURS))
            * f64::from(consts::DAY_HOURS)
            * f64::from(consts::HOUR_MINUTES)) as u8;
        let sec: f64 = (frac_part
            - f64::from(hour) / f64::from(consts::DAY_HOURS)
            - f64::from(min) / f64::from(consts::DAY_HOURS) / f64::from(consts::HOUR_MINUTES))
            * f64::from(consts::DAY_SECS);
        (year, month, day, hour, min, sec)
    }
}

impl From<UtcTime> for MJD {
    fn from(utc: UtcTime) -> MJD {
        utc.to_mjd()
    }
}
