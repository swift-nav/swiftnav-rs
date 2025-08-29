// Copyright (c) 2025 Swift Navigation Inc.
// Contact: Swift Navigation <dev@swiftnav.com>
//
// This source is subject to the license found in the file 'LICENSE' which must
// be be distributed together with this source. All other rights reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND,
// EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A PARTICULAR PURPOSE.

//! Common constant values related to time manipulation

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

/// Number of seconds in a minute.
pub const MINUTE_SECS: u32 = 60;

/// Number of minutes in an hour.
pub const HOUR_MINUTES: u32 = 60;

/// Number of seconds in an hour.
pub const HOUR_SECS: u32 = MINUTE_SECS * HOUR_MINUTES;

/// Number of hours in a day.
pub const DAY_HOURS: u32 = 24;

/// Number of seconds in a day.
pub const DAY_SECS: u32 = DAY_HOURS * HOUR_MINUTES * MINUTE_SECS;

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

/// The threshold for considering two time values as equivalent
/// in [`PartialEq`] and [`PartialOrd`]
pub const JIFFY: f64 = 1e-12;
