//! Time handling
//!
//! GPS time counts the number of seconds since Midnight Jan 8th 1980 UTC. Leap
//! seconds are not counted, so there is an offset between UTC and GPS time. GPS
//! time is usually represented as a week number, counting the number of elapsed
//! weeks since the start of GPS time, and a time of week counting the number of
//! seconds since the beginning of the week. In GPS time the week begins at
//! midnight on Sunday.

use crate::c_bindings;
use std::error::Error;
use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::time::Duration;

pub const MINUTE: Duration = Duration::from_secs(c_bindings::MINUTE_SECS as u64);
pub const HOUR: Duration = Duration::from_secs(c_bindings::HOUR_SECS as u64);
pub const DAY: Duration = Duration::from_secs(c_bindings::DAY_SECS as u64);
pub const WEEK: Duration = Duration::from_secs(c_bindings::WEEK_SECS as u64);

/// Representation of GPS Time
#[derive(Copy, Clone)]
pub struct GpsTime(c_bindings::gps_time_t);

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum InvalidGpsTime {
    InvalidWN(i16),
    InvalidTOW(f64),
}

impl fmt::Display for InvalidGpsTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InvalidGpsTime::InvalidWN(wn) => write!(f, "Invalid Week Number: {}", wn),
            InvalidGpsTime::InvalidTOW(tow) => write!(f, "Invalid Time of Wee: {}", tow),
        }
    }
}

impl Error for InvalidGpsTime {}

impl GpsTime {
    const JIFFY: f64 = c_bindings::FLOAT_EQUALITY_EPS;

    /// Makes a new GPS time object and checks the validity of the given values.
    ///
    /// Invalid values include negative week values, negative, non-finite, or to
    /// large time of week values.
    pub fn new(wn: i16, tow: f64) -> Result<GpsTime, InvalidGpsTime> {
        if wn < 0 {
            Err(InvalidGpsTime::InvalidWN(wn))
        } else if !tow.is_finite() || tow < 0. || tow >= WEEK.as_secs_f64() {
            Err(InvalidGpsTime::InvalidTOW(tow))
        } else {
            Ok(GpsTime::new_unchecked(wn, tow))
        }
    }

    /// Makes a new GPS time object without checking the validity of the given
    /// values.
    pub(crate) fn new_unchecked(wn: i16, tow: f64) -> GpsTime {
        GpsTime(c_bindings::gps_time_t { wn, tow })
    }

    pub(crate) fn to_gps_time_t(self) -> c_bindings::gps_time_t {
        self.0
    }

    pub(crate) fn c_ptr(&self) -> *const c_bindings::gps_time_t {
        &self.0
    }

    pub(crate) fn unknown() -> c_bindings::gps_time_t {
        c_bindings::gps_time_t { tow: -1.0, wn: -1 }
    }

    /// Gets the week number
    pub fn wn(&self) -> i16 {
        self.0.wn
    }

    /// Gets the time of week
    pub fn tow(&self) -> f64 {
        self.0.tow
    }

    /// Checks if the stored time is valid
    pub fn is_valid(&self) -> bool {
        unsafe { c_bindings::gps_time_valid(&self.0) }
    }

    /// Adds a duration to the time
    pub fn add_duration(&mut self, duration: &Duration) {
        unsafe {
            c_bindings::add_secs(&mut self.0, duration.as_secs_f64());
        }
    }

    /// Subtracts a duration from the time
    pub fn subtract_duration(&mut self, duration: &Duration) {
        unsafe {
            c_bindings::add_secs(&mut self.0, -duration.as_secs_f64());
        }
    }

    /// Gets the difference between this and another time value in seconds
    pub fn diff(&self, other: &Self) -> f64 {
        unsafe { c_bindings::gpsdifftime(&self.0, &other.0) }
    }

    /// Converts the GPS time into UTC time
    pub fn to_utc(&self, utc_params: &UtcParams) -> UtcTime {
        let mut utc = UtcTime::default();
        unsafe { c_bindings::gps2utc(self.c_ptr(), utc.mut_c_ptr(), utc_params.c_ptr()); }
        utc
    }

    /// Converts the GPS time into UTC time using the hardcoded list of leap
    /// seconds.
    ///
    /// Note: The hard coded list of leap seconds will get out of date, it is
    /// preferable to use `GpsTime::to_utc()` with the newest set of UTC parameters
    pub fn to_utc_hardcoded(&self) -> UtcTime {
        let mut utc = UtcTime::default();
        unsafe { c_bindings::gps2utc(self.c_ptr(), utc.mut_c_ptr(), std::ptr::null()); }
        utc
    }

    /// Gets the number of seconds difference between GPS and UTC times
    pub fn get_utc_offset(&self, utc_params: &UtcParams) -> f64 {
        unsafe { c_bindings::get_gps_utc_offset(self.c_ptr(), utc_params.c_ptr()) }
    }

    /// Gets the number of seconds difference between GPS and UTC using a hardcoded
    /// list of leap seconds
    ///
    /// Note: The hard coded list of leap seconds will get out of date, it is
    /// preferable to use `GpsTime::get_utc_offset_hardcoded()` with the newest set
    /// of UTC parameters
    pub fn get_utc_offset_hardcoded(&self) -> f64 {
        unsafe { c_bindings::get_gps_utc_offset(self.c_ptr(), std::ptr::null()) }
    }

    /// Checks to see if this point in time is a UTC leap second event
    pub fn is_leap_second_event(&self, utc_params: &UtcParams) -> bool {
        unsafe { c_bindings::is_leap_second_event(self.c_ptr(), utc_params.c_ptr()) }
    }

    /// Checks to see if this point in time is a UTC leap second event using a
    /// hardcoded list of leap seconds
    ///
    /// Note: The hard coded list of leap seconds will get out of date, it is
    /// preferable to use `GpsTime::is_leap_second_event_hardcoded()` with the newest
    /// set of UTC parameters
    pub fn is_leap_second_event_hardcoded(&self) -> bool {
        unsafe { c_bindings::is_leap_second_event(self.c_ptr(), std::ptr::null()) }
    }

    /// Rounds the GPS time to the nearest epoch
    pub fn round_to_epoch(&self, soln_freq: f64) -> GpsTime {
        GpsTime(unsafe { c_bindings::round_to_epoch(self.c_ptr(), soln_freq) })
    }

    /// Rounds the GPS time down to the previous whole epoch
    pub fn floor_to_epoch(&self, soln_freq: f64) -> GpsTime {
        GpsTime(unsafe { c_bindings::floor_to_epoch(self.c_ptr(), soln_freq) })
    }
}

impl fmt::Debug for GpsTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GpsTime")
            .field("WN", &self.0.wn)
            .field("TOW", &self.0.tow)
            .finish()
    }
}

impl PartialEq for GpsTime {
    fn eq(&self, other: &Self) -> bool {
        let diff_seconds = self.diff(other).abs();
        diff_seconds < Self::JIFFY
    }
}

impl PartialOrd for GpsTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let diff_seconds = self.diff(other);

        if diff_seconds.abs() < Self::JIFFY {
            Some(std::cmp::Ordering::Equal)
        } else if diff_seconds > 0.0 {
            Some(std::cmp::Ordering::Greater)
        } else {
            Some(std::cmp::Ordering::Less)
        }
    }
}

impl Add<Duration> for GpsTime {
    type Output = Self;
    fn add(mut self, rhs: Duration) -> Self {
        self.add_duration(&rhs);
        self
    }
}

impl AddAssign<Duration> for GpsTime {
    fn add_assign(&mut self, rhs: Duration) {
        self.add_duration(&rhs);
    }
}

impl Sub<GpsTime> for GpsTime {
    type Output = Duration;
    fn sub(self, rhs: GpsTime) -> Duration {
        let diff_seconds = self.diff(&rhs).abs();
        Duration::from_secs_f64(diff_seconds)
    }
}

impl Sub<Duration> for GpsTime {
    type Output = Self;
    fn sub(mut self, rhs: Duration) -> Self::Output {
        self.subtract_duration(&rhs);
        self
    }
}

impl SubAssign<Duration> for GpsTime {
    fn sub_assign(&mut self, rhs: Duration) {
        self.subtract_duration(&rhs);
    }
}

/// Structure containing GPS UTC correction parameters
#[derive(Clone)]
pub struct UtcParams(c_bindings::utc_params_t);

impl UtcParams {
    pub(crate) fn mut_c_ptr(&mut self) -> *mut c_bindings::utc_params_t {
        &mut self.0
    }

    pub(crate) fn c_ptr(&self) -> *const c_bindings::utc_params_t {
        &self.0
    }

    /// Decodes UTC parameters from GLS LNAV message subframe 4 words 6-10.
    ///
    /// Note: Fills out the full time of week from current gps week cycle. Also
    /// sets t_lse to the exact GPS time at the start of the leap second event.
    ///
    /// References:
    /// -# IS-GPS-200H, Section 20.3.3.5.1.6
    pub fn decode(words: &[u32; 8]) -> Option<Self> {
        let mut params = UtcParams::default();
        let result = unsafe {
            c_bindings::decode_utc_parameters(words, params.mut_c_ptr())
        };

        if result {
            Some(params)
        } else {
            None
        }
    }
}

impl Default for UtcParams {
    fn default() -> Self {
        unsafe { std::mem::zeroed::<UtcParams>() }
    }
}

/// Structure representing UTC time
#[derive(Clone)]
pub struct UtcTime(c_bindings::utc_tm);

impl UtcTime {
    pub(crate) fn mut_c_ptr(&mut self) -> *mut c_bindings::utc_tm {
        &mut self.0
    }

    pub(crate) fn c_ptr(&self) -> *const c_bindings::utc_tm {
        &self.0
    }

    /// Creates a UTC time from it's individual components
    pub fn from_date(year: u16, month: u8, day: u8, hour: u8, minute: u8, second: f64) -> UtcTime {
        UtcTime(unsafe { c_bindings::date2utc(year as i32, month as i32, day as i32, hour as i32, minute as i32, second) })
    }

    /// Number of years CE. In four digit format
    pub fn get_year(&self) -> u16 {
        self.0.year
    }

    /// Day of the year (1 - 366)
    pub fn get_day_of_year(&self) -> u16 {
        self.0.year_day
    }

    /// Month of the year (1 - 12). 1 = January, 12 = December
    pub fn get_month(&self) -> u8 {
        self.0.month
    }

    /// Day of the month (1 - 31)
    pub fn get_day_of_month(&self) -> u8 {
        self.0.month_day
    }

    /// Day of the week (1 - 7). 1 = Monday, 7 = Sunday
    pub fn get_day_of_week(&self) -> u8 {
        self.0.week_day
    }

    /// Hour of the day (0 - 23)
    pub fn get_hour(&self) -> u8 {
        self.0.hour
    }

    /// Minutes of the hour (0 - 59)
    pub fn get_min(&self) -> u8 {
        self.0.minute
    }

    /// seconds of the minute (0 - 60)
    pub fn get_seconds(&self) -> f64 {
        (self.0.second_int as f64) + self.0.second_frac
    }

    /// Converts the UTC time into a modified julian date
    pub fn to_mjd(&self) -> MJD {
        MJD(unsafe { c_bindings::utc2mjd(self.c_ptr()) })
    }

    /// Makes an ISO8601 compatible date time string from the UTC time
    pub fn iso8601_str(&self) -> String {
        format!("{}-{}-{}T{}:{}:{:.3}", self.get_year(), self.get_month(), self.get_day_of_month(), self.get_hour(), self.get_min(), self.get_seconds())
    }

    // TODO: We could easily add conversions to other UTC representations
    //  for interoperability, but which time crates to support?
}

impl Default for UtcTime {
    fn default() -> Self {
        unsafe { std::mem::zeroed::<UtcTime>() }
    }
}

/// Structure representing a modified julian date (MJD)
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct MJD(f64);

impl MJD {
    /// Creates a modified julian date from a floating point representation
    pub fn from_f64(value: f64) -> Self {
        Self(value)
    }

    /// Creates a modified julian date from a calendar date and time
    pub fn from_date(year: u16, month: u8, day: u8, hour: u8, minute: u8, seconds: f64) -> MJD {
        MJD(unsafe { c_bindings::date2mjd(year as i32, month as i32, day as i32, hour as i32, minute as i32, seconds) })
    }

    /// Gets the floating point value of the modified julian date
    pub fn as_f64(&self) -> f64 {
        self.0
    }

    /// Converts the modified julian date into a UTC time
    pub fn to_utc(&self) -> UtcTime {
        UtcTime(unsafe { c_bindings::mjd2utc(self.0) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validity() {
        assert!(GpsTime::new(0, 0.0).is_ok());
        assert!(GpsTime::new(-1, -1.0).is_err());
        assert!(GpsTime::new(-1, -1.0).is_err());
        assert!(GpsTime::new(12, WEEK.as_secs_f64()).is_err());
        assert!(GpsTime::new(12, std::f64::NAN).is_err());
        assert!(GpsTime::new(12, std::f64::INFINITY).is_err());
    }

    #[test]
    fn equality() {
        let t1 = GpsTime::new(10, 234.567).unwrap();
        assert!(t1 == t1);

        let t2 = GpsTime::new(10, 234.5678).unwrap();
        assert!(t1 != t2);
        assert!(t2 != t1);
    }

    #[test]
    fn ordering() {
        let t1 = GpsTime::new(10, 234.566).unwrap();
        let t2 = GpsTime::new(10, 234.567).unwrap();
        let t3 = GpsTime::new(10, 234.568).unwrap();

        assert!(t1 < t2);
        assert!(t1 < t3);
        assert!(t2 > t1);
        assert!(t2 < t3);
        assert!(t3 > t1);
        assert!(t3 > t2);

        assert!(t1 <= t1);
        assert!(t1 >= t1);
        assert!(t1 <= t2);
        assert!(t1 <= t3);
        assert!(t2 >= t1);
        assert!(t2 <= t2);
        assert!(t2 >= t2);
        assert!(t2 <= t3);
        assert!(t3 >= t1);
        assert!(t3 >= t2);
        assert!(t3 <= t3);
        assert!(t3 >= t3);
    }

    #[test]
    fn add_duration() {
        let mut t = GpsTime::new(0, 0.0).unwrap();
        let t_expected = GpsTime::new(0, 1.001).unwrap();
        let d = Duration::new(1, 1000000);

        t.add_duration(&d);
        assert_eq!(t, t_expected);

        let t = GpsTime::new(0, 0.0).unwrap();
        let t = t + d;
        assert_eq!(t, t_expected);

        let mut t = GpsTime::new(0, 0.0).unwrap();
        t += d;
        assert_eq!(t, t_expected);
    }

    #[test]
    fn subtract_duration() {
        let mut t = GpsTime::new(0, 1.001).unwrap();
        let t_expected = GpsTime::new(0, 0.0).unwrap();
        let d = Duration::new(1, 1000000);

        t.subtract_duration(&d);
        assert_eq!(t, t_expected);

        t.subtract_duration(&d);
        assert!(!t.is_valid());

        let t = GpsTime::new(0, 1.001).unwrap();
        let t = t - d;
        assert_eq!(t, t_expected);

        let mut t = GpsTime::new(0, 1.001).unwrap();
        t -= d;
        assert_eq!(t, t_expected);
    }

    #[test]
    fn gps_utc_offset() {
        struct UtcOffsetTestdata {
            t: GpsTime,
            d_utc: f64,
            is_lse: bool,
        }
        let test_cases: &[UtcOffsetTestdata] = &[
            /* July 1 1981 */
            UtcOffsetTestdata{t: GpsTime::new_unchecked(77, 259199.0), d_utc: 0.0, is_lse: false},
            UtcOffsetTestdata{t: GpsTime::new_unchecked(77, 259199.5), d_utc: 0.0, is_lse: false},
            UtcOffsetTestdata{t: GpsTime::new_unchecked(77, 259200.0), d_utc: 0.0, is_lse: true},
            UtcOffsetTestdata{t: GpsTime::new_unchecked(77, 259200.5), d_utc: 0.0, is_lse: true},
            UtcOffsetTestdata{t: GpsTime::new_unchecked(77, 259201.0), d_utc: 1.0, is_lse: false},
            UtcOffsetTestdata{t: GpsTime::new_unchecked(77, 259202.0), d_utc: 1.0, is_lse: false},
            /* Jan 1 2017 */
            UtcOffsetTestdata{t: GpsTime::new_unchecked(1930, 16.0), d_utc: 17.0, is_lse: false},
            UtcOffsetTestdata{t: GpsTime::new_unchecked(1930, 16.5), d_utc: 17.0, is_lse: false},
            UtcOffsetTestdata{t: GpsTime::new_unchecked(1930, 17.0), d_utc: 17.0, is_lse: true},
            UtcOffsetTestdata{t: GpsTime::new_unchecked(1930, 17.5), d_utc: 17.0, is_lse: true},
            UtcOffsetTestdata{t: GpsTime::new_unchecked(1930, 18.0), d_utc: 18.0, is_lse: false},
            UtcOffsetTestdata{t: GpsTime::new_unchecked(1930, 18.5), d_utc: 18.0, is_lse: false},
            UtcOffsetTestdata{t: GpsTime::new_unchecked(1930, 19.0), d_utc: 18.0, is_lse: false},
        ];
        for test_case in test_cases {
            let d_utc = test_case.t.get_utc_offset_hardcoded();
            let is_lse = test_case.t.is_leap_second_event_hardcoded();

            assert!(d_utc == test_case.d_utc && is_lse == test_case.is_lse);
        }
    }
}
