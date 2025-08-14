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
//! are available, such as [`UtcTime`], [`GalTime`], [`BdsTime`], and [`GloTime`]
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
//! information, and [`UtcParams::decode()`] is provided for decoding the raw GPS
//! navigation subframe with this information. This is the prefered method since it
//! is usually available when processing raw GNSS data and ensures that the right
//! offset is applied at the right time.
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
//! When converting to or from a time base that uses leap seconds (i.e. [`UtcTime`]
//! and [`GloTime`]) two functions are always provided, one which takes a
//! [`UtcParams`] object to handle the leap second conversion and one which doesn't
//! take a [`UtcParams`] object but has `_hardcoded` appended to the function name.

use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::time::Duration;

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

pub const MINUTE: Duration = Duration::from_secs(consts::MINUTE_SECS as u64);
pub const HOUR: Duration = Duration::from_secs(consts::HOUR_SECS as u64);
pub const DAY: Duration = Duration::from_secs(consts::DAY_SECS as u64);
pub const WEEK: Duration = Duration::from_secs(consts::WEEK_SECS as u64);

/// Representation of GPS Time
#[derive(Copy, Clone)]
pub struct GpsTime {
    /// Seconds since the GPS start of week.
    tow: f64,
    /// GPS week number
    wn: i16,
}

/// GPS timestamp of the start of Galileo time
pub const GAL_TIME_START: GpsTime =
    GpsTime {
        wn: consts::GAL_WEEK_TO_GPS_WEEK,
        tow: consts::GAL_SECOND_TO_GPS_SECOND,
    };

/// GPS timestamp of the start of Beidou time
pub const BDS_TIME_START: GpsTime = GpsTime {
    wn: consts::BDS_WEEK_TO_GPS_WEEK,
    tow: consts::BDS_SECOND_TO_GPS_SECOND,
};

/// Error type when a given GPS time is not valid
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, thiserror::Error)]
pub enum InvalidGpsTime {
    #[error("Invalid Week Number: {0}")]
    /// Indicates an invalid week number was given, with the invalid value returned
    InvalidWN(i16),
    #[error("Invalid Time of Week: {0}")]
    /// Indicates an invalid time of week was given, with the invalid value returned
    InvalidTOW(f64),
}

impl GpsTime {
    const JIFFY: f64 = 1e-12;

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
            Ok(GpsTime{ wn, tow })
        }
    }

    /// Gets the week number
    pub fn wn(&self) -> i16 {
        self.wn
    }

    /// Gets the time of week
    pub fn tow(&self) -> f64 {
        self.tow
    }

    /// Checks if the stored time is valid
    pub fn is_valid(&self) -> bool {
        self.tow.is_finite() && self.tow >= 0.0 && self.tow < consts::WEEK_SECS as f64 && self.wn >= 0
    }

    fn normalize(&mut self) {
        while self.tow < 0.0 {
            self.tow += consts::WEEK_SECS as f64;
            self.wn -= 1;
        }

        while self.tow >= consts::WEEK_SECS as f64 {
            self.tow -= consts::WEEK_SECS as f64;
            self.wn += 1;
        }
    }

    /// Adds a duration to the time
    pub fn add_duration(&mut self, duration: &Duration) {
        self.tow += duration.as_secs_f64();
        self.normalize();
    }

    /// Subtracts a duration from the time
    pub fn subtract_duration(&mut self, duration: &Duration) {
        self.tow -= duration.as_secs_f64();
        self.normalize();
    }

    /// Gets the difference between this and another time value in seconds
    pub fn diff(&self, other: &Self) -> f64 {
        let dt = self.tow - other.tow;
        dt + (self.wn - other.wn) as f64 * consts::WEEK_SECS as f64
    }

    fn to_utc_internal(self, params: Option<&UtcParams>) -> UtcTime {
        // Is it during a (positive) leap second event
        // Get the UTC offset at the time we're converting
        let (is_lse, dt_utc) = params.map_or_else(
            || (self.is_leap_second_event_hardcoded(), self.utc_offset_hardcoded()),
            |p| (self.is_leap_second_event(p), self.utc_offset(p))
        );

        let mut tow_utc = self.tow - dt_utc;

        if is_lse {
            /* positive leap second event ongoing, so we are at 23:59:60.xxxx
            * subtract one second from time for now to make the conversion
            * into yyyy/mm/dd HH:MM:SS.sssss format, and add it back later */
            tow_utc -= 1.0;
        }

        let mut t_u = GpsTime{ wn: self.wn, tow: tow_utc };
        t_u.normalize();

        /* break the time into components */
        let mut u = t_u.make_utc();

        if is_lse {
            assert!(u.hour == 23);
            assert!(u.minute == 59);
            assert!(u.second_int == 59);
            /* add the extra second back in*/
            u.second_int += 1;
        }

        u
    }

    pub(self) fn make_utc(&self) -> UtcTime {
        /* see http://www.ngs.noaa.gov/gps-toolbox/bwr-c.txt */

        /* seconds of the day */
        let t_utc = self.tow % (consts::DAY_SECS as f64);

        /* Convert this into hours, minutes and seconds */
        let second_int = t_utc.floor() as u32;   /* The integer part of the seconds */
        let second_frac: f64 = t_utc % 1.0;    /* The fractional part of the seconds */
        let hour: u8 = (second_int / consts::HOUR_SECS) as u8;     /* The hours (1 - 23) */
        let second_int = second_int - ((hour as u32) * consts::HOUR_SECS);    /* Remove the hours from seconds */
        let minute: u8 = (second_int / consts::MINUTE_SECS) as u8; /* The minutes (1 - 59) */
        let second_int: u8 = (second_int - minute as u32 * consts::MINUTE_SECS) as u8; /* Remove the minutes from seconds */ /* The seconds (1 - 60) */

        /* Calculate the years */

        /* Days from 1 Jan 1980. GPS epoch is 6 Jan 1980 */
        let modified_julian_days: i32 =
            consts::MJD_JAN_6_1980 + self.wn as i32 * 7 + (self.tow / consts::DAY_SECS as f64).floor() as i32;
        let days_since_1601: u32 = (modified_julian_days - consts::MJD_JAN_1_1601) as u32;

        /* Calculate the number of leap years */
        let num_400_years: u32 = days_since_1601 / consts::FOUR_HUNDRED_YEARS_DAYS;
        let days_left: u32 = days_since_1601 - num_400_years * consts::FOUR_HUNDRED_YEARS_DAYS;
        let num_100_years: u32 = days_left / consts::HUNDRED_YEARS_DAYS -
                            days_left / (consts::FOUR_HUNDRED_YEARS_DAYS - 1);
        let days_left: u32 = days_left - num_100_years * consts::HUNDRED_YEARS_DAYS;
        let num_4_years: u32 = days_left / consts::FOUR_YEARS_DAYS;
        let days_left: u32 = days_left - num_4_years * consts::FOUR_YEARS_DAYS;
        let num_non_leap_years =
            days_left / consts::YEAR_DAYS - days_left / (consts::FOUR_YEARS_DAYS - 1);

        /* Calculate the total number of years from 1980 */
        let year = (1601 + num_400_years * 400 + num_100_years * 100 + num_4_years * 4 +
                    num_non_leap_years) as u16;

        /* Calculate the month of the year */

        /* Calculate the day of the current year */
        let year_day = (days_left - num_non_leap_years * consts::YEAR_DAYS + 1) as u16;

        /* Work out if it is currently a leap year, 0 = no, 1 = yes` */
        let leap_year: usize = if is_leap_year(year) { 1 } else { 0 };

        /* Roughly work out the month number */
        let month_guess: u8 = (year_day as f32 * 0.032) as u8;

        /* Lookup table of the total number of days in the year after each month */
        /* First row is for a non-leap year, second row is for a leap year */
        const DAYS_AFTER_MONTH: [[u16; 13]; 2] = [
            [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365],
            [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335, 366]];

        /* Check if our guess was out, and what the correction is, */
        /* 0 = correct, 1 = wrong */
        let month_correction: u8 =
            if year_day > DAYS_AFTER_MONTH[leap_year][(month_guess + 1) as usize] { 1 } else { 0 };

        /* Calculate the corrected number of months */
        let month = month_guess + month_correction + 1;

        /* Calculate the day of the month */
        let month_day =
            (year_day - DAYS_AFTER_MONTH[leap_year][(month_guess + month_correction) as usize]) as u8;

        /* Calculate the day of the week. 1 Jan 1601 was a Monday */
        let week_day = (days_since_1601 % 7 + 1) as u8;

        UtcTime {
            year,
            year_day,
            month,      
            month_day,  
            week_day,   
            hour,       
            minute,     
            second_int, 
            second_frac,
        }
    }

    /// Converts the GPS time into UTC time
    ///
    /// # Panics
    /// This function will panic if the GPS time is not valid
    pub fn to_utc(self, utc_params: &UtcParams) -> UtcTime {
        self.to_utc_internal(Some(utc_params))
    }

    /// Converts the GPS time into UTC time using the hardcoded list of leap
    /// seconds.
    ///
    /// # ⚠️  🦘  ⏱  ⚠️  - Leap Seconds
    /// The hard coded list of leap seconds will get out of date, it is
    /// preferable to use [`GpsTime::to_utc()`] with the newest set of UTC parameters
    ///
    /// # Panics
    /// This function will panic if the GPS time is not valid
    pub fn to_utc_hardcoded(self) -> UtcTime {
        self.to_utc_internal(None)
    }

    /// Gets the number of seconds difference between GPS and UTC times
    pub fn utc_offset(&self, utc_params: &UtcParams) -> f64 {
        let dt = self.diff(&utc_params.tot());

        /* The polynomial UTC to GPS correction */
        let mut dt_utc: f64 = utc_params.a0() + (utc_params.a1() * dt) + (utc_params.a2() * dt * dt);

        /* the new UTC offset takes effect after the leap second event */
        if self.diff(&utc_params.t_lse()) >= 1.0 {
            dt_utc += utc_params.dt_lsf() as f64;
        } else {
            dt_utc += utc_params.dt_ls() as f64;
        }

        dt_utc
    }

    /// Gets the number of seconds difference between GPS and UTC using the hardcoded
    /// list of leap seconds
    ///
    /// # ⚠️  🦘  ⏱  ⚠️  - Leap Seconds
    /// The hard coded list of leap seconds will get out of date, it is
    /// preferable to use [`GpsTime::utc_offset()`] with the newest set
    /// of UTC parameters
    pub fn utc_offset_hardcoded(&self) -> f64 {
        for (t_leap, offset) in UTC_LEAPS.iter().rev() {
            if self.diff(t_leap) >= 1.0 {
                return *offset as f64;
            }
        }

        /* time is before the first known leap second event */
        0.0
    }

    /// Checks to see if this point in time is a UTC leap second event
    pub fn is_leap_second_event(&self, params: &UtcParams) -> bool {
        /* the UTC offset takes effect exactly 1 second after the start of
        * the (positive) leap second event */
        let dt = self.diff(&params.t_lse);

        /* True only when self is during the leap second event */
        dt >= 0.0 && dt < 1.0
    }

    /// Checks to see if this point in time is a UTC leap second event using the
    /// hardcoded list of leap seconds
    ///
    /// # ⚠️  🦘  ⏱  ⚠️  - Leap Seconds
    /// The hard coded list of leap seconds will get out of date, it is
    /// preferable to use [`GpsTime::is_leap_second_event()`] with the newest
    /// set of UTC parameters
    pub fn is_leap_second_event_hardcoded(&self) -> bool {
        for (t_leap, _offset) in UTC_LEAPS.iter().rev() {
            let dt = self.diff(t_leap);

            if dt > 1.0 {
                /* time is past the last known leap second event */
                return false;
            }
            if dt >= 0.0 && dt < 1.0 {
                /* time is during the leap second event */
                return true;
            }
        }

        /* time is before the first known leap second event */
        false
    }

    /// Converts the GPS time into Galileo time
    ///
    /// # Panics
    /// This function will panic if the GPS time is before the start of Galileo
    /// time, i.e. [`GAL_TIME_START`]
    pub fn to_gal(self) -> GalTime {
        assert!(self.is_valid());
        assert!(self >= GAL_TIME_START);
        GalTime {
            wn: self.wn() - consts::GAL_WEEK_TO_GPS_WEEK,
            tow: self.tow(),
        }
    }

    /// Converts the GPS time into Beidou time
    ///
    /// # Panics
    /// This function will panic if the GPS time is before the start of Beidou
    /// time, i.e. [`BDS_TIME_START`]
    pub fn to_bds(self) -> BdsTime {
        assert!(self.is_valid());
        assert!(self >= BDS_TIME_START);
        let bds = GpsTime {
            wn: self.wn() - consts::BDS_WEEK_TO_GPS_WEEK,
            tow: self.tow(),
        };
        let bds = bds - Duration::from_secs_f64(consts::BDS_SECOND_TO_GPS_SECOND);
        BdsTime {
            wn: bds.wn(),
            tow: bds.tow(),
        }
    }

    #[rustversion::since(1.62)]
    /// Compare between itself and other GpsTime
    /// Checks whether week number is same which then mirrors
    /// [f64::total_cmp()](https://doc.rust-lang.org/std/primitive.f64.html#method.total_cmp)
    pub fn total_cmp(&self, other: &GpsTime) -> std::cmp::Ordering {
        if self.wn() != other.wn() {
            self.wn().cmp(&other.wn())
        } else {
            let other = other.tow();
            self.tow().total_cmp(&other)
        }
    }

    pub fn to_fractional_year(&self, utc_params: &UtcParams) -> f64 {
        let utc = self.to_utc(utc_params);
        utc.to_fractional_year()
    }

    pub fn to_fractional_year_hardcoded(&self) -> f64 {
        let utc = self.to_utc_hardcoded();
        utc.to_fractional_year()
    }
}

impl fmt::Debug for GpsTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GpsTime")
            .field("WN", &self.wn)
            .field("TOW", &self.tow)
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

impl From<GalTime> for GpsTime {
    fn from(gal: GalTime) -> Self {
        gal.to_gps()
    }
}

impl From<BdsTime> for GpsTime {
    fn from(bds: BdsTime) -> Self {
        bds.to_gps()
    }
}

/// Representation of Galileo Time
#[derive(Debug, Copy, Clone)]
pub struct GalTime {
    wn: i16,
    tow: f64,
}

impl GalTime {
    pub fn new(wn: i16, tow: f64) -> Result<GalTime, InvalidGpsTime> {
        if wn < 0 {
            Err(InvalidGpsTime::InvalidWN(wn))
        } else if !tow.is_finite() || tow < 0. || tow >= WEEK.as_secs_f64() {
            Err(InvalidGpsTime::InvalidTOW(tow))
        } else {
            Ok(GalTime { wn, tow })
        }
    }

    pub fn wn(&self) -> i16 {
        self.wn
    }

    pub fn tow(&self) -> f64 {
        self.tow
    }

    pub fn to_gps(self) -> GpsTime {
        GpsTime {
            wn: self.wn + consts::GAL_WEEK_TO_GPS_WEEK,
            tow: self.tow,
        }
    }

    pub fn to_bds(self) -> BdsTime {
        self.to_gps().to_bds()
    }
}

impl From<GpsTime> for GalTime {
    fn from(gps: GpsTime) -> Self {
        gps.to_gal()
    }
}

impl From<BdsTime> for GalTime {
    fn from(bds: BdsTime) -> Self {
        bds.to_gal()
    }
}

/// Representation of Beidou Time
#[derive(Debug, Copy, Clone)]
pub struct BdsTime {
    wn: i16,
    tow: f64,
}

impl BdsTime {
    pub fn new(wn: i16, tow: f64) -> Result<BdsTime, InvalidGpsTime> {
        if wn < 0 {
            Err(InvalidGpsTime::InvalidWN(wn))
        } else if !tow.is_finite() || tow < 0. || tow >= WEEK.as_secs_f64() {
            Err(InvalidGpsTime::InvalidTOW(tow))
        } else {
            Ok(BdsTime { wn, tow })
        }
    }

    pub fn wn(&self) -> i16 {
        self.wn
    }

    pub fn tow(&self) -> f64 {
        self.tow
    }

    pub fn to_gps(self) -> GpsTime {
        let gps = GpsTime {
            wn: self.wn() + consts::BDS_WEEK_TO_GPS_WEEK,
            tow: self.tow(),
        };
        gps + Duration::from_secs_f64(consts::BDS_SECOND_TO_GPS_SECOND)
    }

    pub fn to_gal(self) -> GalTime {
        self.to_gps().to_gal()
    }
}

impl From<GpsTime> for BdsTime {
    fn from(gps: GpsTime) -> Self {
        gps.to_bds()
    }
}

impl From<GalTime> for BdsTime {
    fn from(gal: GalTime) -> Self {
        gal.to_bds()
    }
}

/// GPS UTC correction parameters
#[derive(Clone)]
pub struct UtcParams {
    /// Modulo 1 sec offset from GPS to UTC [s]
    a0: f64,        
    /// Drift of time offset from GPS to UTC [s/s]
    a1: f64,        
    /// Drift rate correction from GPS to UTC [s/s]
    a2: f64,        
    /// Reference time of UTC parameters.
    tot: GpsTime,   
    /// Time of leap second event.
    t_lse: GpsTime, 
    /// Leap second delta from GPS to UTC before LS event [s]
    dt_ls: i8,         
    /// Leap second delta from GPS to UTC after LS event [s]
    dt_lsf: i8,       
}

impl UtcParams {
    /// Build the UTC parameters from the already decoded parameters
    ///
    /// # Panics
    /// This function will panic if either `tot` or `t_lse` are not valid
    pub fn from_components(
        a0: f64,
        a1: f64,
        a2: f64,
        tot: &GpsTime,
        t_lse: &GpsTime,
        dt_ls: i8,
        dt_lsf: i8,
    ) -> UtcParams {
        assert!(tot.is_valid() && t_lse.is_valid());

        UtcParams {
            a0,
            a1,
            a2,
            tot: *tot,
            t_lse: *t_lse,
            dt_ls,
            dt_lsf,
        }
    }

    /// Modulo 1 sec offset from GPS to UTC \[s\]
    pub fn a0(&self) -> f64 {
        self.a0
    }
    /// Drift of time offset from GPS to UTC \[s/s\]
    pub fn a1(&self) -> f64 {
        self.a1
    }
    /// Drift rate correction from GPS to UTC \[s/s\]
    pub fn a2(&self) -> f64 {
        self.a2
    }
    /// Reference time of UTC parameters.
    pub fn tot(&self) -> GpsTime {
        self.tot
    }
    /// Time of leap second event.
    pub fn t_lse(&self) -> GpsTime {
        self.t_lse
    }
    /// Leap second delta from GPS to UTC before LS event \[s\]
    pub fn dt_ls(&self) -> i8 {
        self.dt_ls
    }
    /// Leap second delta from GPS to UTC after LS event \[s\]
    pub fn dt_lsf(&self) -> i8 {
        self.dt_lsf
    }
}

/// Representation of UTC time
///
/// Note: This implementation does not aim to be able to represent arbitrary dates and times.
/// It is only meant to represent dates and times over the period that GNSS systems have been
/// around. Specifically it shouldn't be relied on for dates significantly before January 6th 1980,
/// the start of GPS time.
#[derive(Clone)]
pub struct UtcTime{
    /// Number of years AD. In four digit format.
    year: u16,
    /// Day of the year (1 - 366).
    year_day: u16,
    /// Month of the year (1 - 12). 1 = January, 12 = December.
    month: u8,      
    /// Day of the month (1 - 31).
    month_day: u8,  
    /// Day of the week (1 - 7). 1 = Monday, 7 = Sunday.
    week_day: u8,   
    /// Minutes of the hour (0 - 59).
    hour: u8,       
    /// Minutes of the hour (0 - 59).
    minute: u8,     
    /// Integer part of seconds of the minute (0 - 60).
    second_int: u8, 
    /// Fractional part of seconds (0 - .99...).
    second_frac: f64, 
}

impl UtcTime {
    /// Creates a UTC time from its individual components
    pub fn from_date(year: u16, month: u8, day: u8, hour: u8, minute: u8, second: f64) -> UtcTime {
        let mjd = MJD::from_date(year, month, day, hour, minute, second);
        mjd.to_utc()
    }

    /// Number of years CE. In four digit format
    pub fn year(&self) -> u16 {
        self.year
    }

    /// Day of the year (1 - 366)
    pub fn day_of_year(&self) -> u16 {
        self.year_day
    }

    /// Month of the year (1 - 12). 1 = January, 12 = December
    pub fn month(&self) -> u8 {
        self.month
    }

    /// Day of the month (1 - 31)
    pub fn day_of_month(&self) -> u8 {
        self.month_day
    }

    /// Day of the week (1 - 7). 1 = Monday, 7 = Sunday
    pub fn day_of_week(&self) -> u8 {
        self.week_day
    }

    /// Hour of the day (0 - 23)
    pub fn hour(&self) -> u8 {
        self.hour
    }

    /// Minutes of the hour (0 - 59)
    pub fn minute(&self) -> u8 {
        self.minute
    }

    /// seconds of the minute (0 - 60)
    pub fn seconds(&self) -> f64 {
        (self.second_int as f64) + self.second_frac
    }

    /// Converts the UTC time into a modified julian date
    pub fn to_mjd(&self) -> MJD {
        MJD::from_date(self.year(), self.month(), self.day_of_month(), self.hour(), self.minute(), self.seconds())
    }

    /// Makes an ISO8601 compatible date time string from the UTC time
    pub fn iso8601_str(&self) -> String {
        format!(
            "{}-{}-{}T{}:{}:{:.3}Z",
            self.year(),
            self.month(),
            self.day_of_month(),
            self.hour(),
            self.minute(),
            self.seconds()
        )
    }

    fn to_gps_internal(&self, utc_params: Option<&UtcParams>) -> GpsTime {
        let is_lse = self.second_int >= 60;
        let mjd = self.to_mjd();

        let mut gps = mjd.to_gps_internal(utc_params);

        // During a leap second event the MJD is wrong by a second, so remove the
        // erroneous second here
        if is_lse {
            gps -= Duration::from_secs(1);
        }

        gps
    }

    /// Converts the UTC time into GPS time
    pub fn to_gps(&self, utc_params: &UtcParams) -> GpsTime {
        self.to_gps_internal(Some(utc_params))
    }

    /// Converts the UTC time into GPS time using the hardcoded list of leap
    /// seconds.
    ///
    /// # ⚠️  🦘  ⏱  ⚠️  - Leap Seconds
    /// The hard coded list of leap seconds will get out of date, it is
    /// preferable to use [`UtcTime::to_gps()`] with the newest set of UTC parameters
    pub fn to_gps_hardcoded(&self) -> GpsTime {
        self.to_gps_internal(None)
    }

    pub fn to_fractional_year(&self) -> f64 {
        let year = self.year() as f64;
        let days = self.day_of_year() as f64;
        let hours = self.hour() as f64;
        let minutes = self.minute() as f64;
        let seconds = self.seconds();
        let total_days = days + hours / consts::DAY_HOURS as f64 + (minutes / consts::MINUTE_SECS as f64 + seconds) / consts::DAY_SECS as f64;

        if is_leap_year(self.year()) {
            year + total_days / consts::LEAP_YEAR_DAYS as f64
        } else {
            year + total_days / consts::YEAR_DAYS as f64
        }
    }
}

impl From<MJD> for UtcTime {
    fn from(mjd: MJD) -> UtcTime {
        mjd.to_utc()
    }
}

#[cfg(feature = "chrono")]
impl From<UtcTime> for chrono::DateTime<chrono::offset::Utc> {
    fn from(utc: UtcTime) -> chrono::DateTime<chrono::offset::Utc> {
        use chrono::prelude::*;

        let date = NaiveDate::from_ymd_opt(
            utc.year() as i32,
            utc.month() as u32,
            utc.day_of_month() as u32,
        )
        .unwrap();
        let whole_seconds = utc.seconds().floor() as u32;
        let frac_seconds = utc.seconds().fract();
        let nanoseconds = (frac_seconds * 1_000_000_000.0).round() as u32;
        let time = NaiveTime::from_hms_nano_opt(
            utc.hour() as u32,
            utc.minute() as u32,
            whole_seconds,
            nanoseconds,
        )
        .unwrap();

        DateTime::from_naive_utc_and_offset(NaiveDateTime::new(date, time), Utc)
    }
}

#[cfg(feature = "chrono")]
impl<Tz: chrono::offset::TimeZone> From<chrono::DateTime<Tz>> for UtcTime {
    fn from(chrono: chrono::DateTime<Tz>) -> UtcTime {
        use chrono::prelude::*;

        let datetime = chrono.naive_utc();
        let seconds = datetime.second() as f64 + (datetime.nanosecond() as f64 / 1_000_000_000.0);

        UtcTime::from_date(
            datetime.year() as u16,
            datetime.month() as u8,
            datetime.day() as u8,
            datetime.hour() as u8,
            datetime.minute() as u8,
            seconds,
        )
    }
}

/// Representation of modified julian dates (MJD)
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct MJD(f64);

impl MJD {
    /// Creates a modified julian date from a floating point representation
    pub fn from_f64(value: f64) -> Self {
        Self(value)
    }

    /// Creates a modified julian date from a calendar date and time
    pub fn from_date(year: u16, month: u8, day: u8, hour: u8, minute: u8, seconds: f64) -> MJD {
        let full_days = 367 * year as i64 - 7 * (year as i64 + (month as i64 + 9) / 12) / 4 -
            3 * ((year as i64 + (month as i64 - 9) / 7) / 100 + 1) / 4 +
            275 * month as i64 / 9 + day as i64 + 1721028 - 2400000;
        let frac_days = (hour as f64) / (consts::DAY_HOURS as f64) + (minute as f64) / ((consts::DAY_HOURS * consts::HOUR_MINUTES) as f64) + seconds / (consts::DAY_SECS as f64);
        MJD(full_days as f64 + frac_days)
    }

    /// Gets the floating point value of the modified julian date
    pub fn as_f64(&self) -> f64 {
        self.0
    }
    
    fn to_gps_internal(self, params: Option<&UtcParams>) -> GpsTime {
        let utc_days: f64 = self.0 - (consts::MJD_JAN_6_1980 as f64);
        
        let wn = (utc_days / consts::WEEK_DAYS as f64) as i16;
        let tow = (utc_days - wn as f64 * consts::WEEK_DAYS as f64) * (consts::DAY_SECS as f64);
        let mut utc_time = GpsTime { wn, tow };

        let leap_secs = params.map_or_else(|| utc_time.utc_offset_hardcoded(), |p| utc_time.utc_offset(p));
        if leap_secs >= 0.0 {
            utc_time += Duration::from_secs_f64(leap_secs);
            utc_time
        } else {
            utc_time -= Duration::from_secs_f64(-leap_secs);
            utc_time
        }
    }

    /// Converts the modified julian date into a UTC time
    pub fn to_utc(&self) -> UtcTime {
        // utc_tm ret;
        let utc_days: f64 = self.0 - consts::MJD_JAN_6_1980 as f64;
        
        let wn = (utc_days / consts::WEEK_DAYS as f64) as i16;
        let tow = (utc_days - (wn as u32 * consts::WEEK_DAYS) as f64) * (consts::DAY_SECS as f64);
        let utc_time = GpsTime { wn, tow };
        utc_time.make_utc()
    }
}

impl From<UtcTime> for MJD {
    fn from(utc: UtcTime) -> MJD {
        utc.to_mjd()
    }
}

pub fn is_leap_year(year: u16) -> bool {
    ((year % 4 == 0) && (year % 100 != 0)) || (year % 400 == 0)
}

/**
 * Start times of UTC leap second events given in GPS time {wn, tow, gps-utc}
 * The leap second event lasts for one second from the start time, and after
 * that the new offset is in effect.
 */
const UTC_LEAPS: [(GpsTime, i32); 18] = [
    (GpsTime{ wn: 77, tow: 259200.}, 1),    /* 01-07-1981 */
    (GpsTime{ wn: 129, tow: 345601.}, 2),   /* 01-07-1982 */
    (GpsTime{ wn: 181, tow: 432002.}, 3),   /* 01-07-1983 */
    (GpsTime{ wn: 286, tow: 86403.}, 4),    /* 01-07-1985 */
    (GpsTime{ wn: 416, tow: 432004.}, 5),   /* 01-01-1988 */
    (GpsTime{ wn: 521, tow: 86405.}, 6),    /* 01-01-1990 */
    (GpsTime{ wn: 573, tow: 172806.}, 7),   /* 01-01-1991 */
    (GpsTime{ wn: 651, tow: 259207.}, 8),   /* 01-07-1992 */
    (GpsTime{ wn: 703, tow: 345608.}, 9),   /* 01-07-1993 */
    (GpsTime{ wn: 755, tow: 432009.}, 10),  /* 01-07-1994 */
    (GpsTime{ wn: 834, tow: 86410.}, 11),   /* 01-01-1996 */
    (GpsTime{ wn: 912, tow: 172811.}, 12),  /* 01-07-1997 */
    (GpsTime{ wn: 990, tow: 432012.}, 13),  /* 01-01-1999 */
    (GpsTime{ wn: 1356, tow: 13.}, 14),     /* 01-01-2006 */
    (GpsTime{ wn: 1512, tow: 345614.}, 15), /* 01-01-2009 */
    (GpsTime{ wn: 1695, tow: 15.}, 16),     /* 01-07-2012 */
    (GpsTime{ wn: 1851, tow: 259216.}, 17), /* 01-07-2015 */
    (GpsTime{ wn: 1930, tow: 17.}, 18),     /* 01-01-2017 */
];

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

    #[rustversion::since(1.62)]
    #[test]
    fn total_order() {
        use std::cmp::Ordering;

        let t1 = GpsTime::new(10, 234.566).unwrap();
        let t2 = GpsTime::new(10, 234.567).unwrap();
        let t3 = GpsTime::new(10, 234.568).unwrap();

        assert!(t1.total_cmp(&t2) == Ordering::Less);
        assert!(t2.total_cmp(&t3) == Ordering::Less);
        assert!(t1.total_cmp(&t3) == Ordering::Less);

        assert!(t2.total_cmp(&t1) == Ordering::Greater);
        assert!(t3.total_cmp(&t2) == Ordering::Greater);
        assert!(t3.total_cmp(&t1) == Ordering::Greater);

        assert!(t1.total_cmp(&t1) == Ordering::Equal);
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
            UtcOffsetTestdata {
                t: GpsTime{ wn: 77, tow:  259199.0 },
                d_utc: 0.0,
                is_lse: false,
            },
            UtcOffsetTestdata {
                t: GpsTime{ wn: 77, tow:  259199.5 },
                d_utc: 0.0,
                is_lse: false,
            },
            UtcOffsetTestdata {
                t: GpsTime{ wn: 77, tow:  259200.0 },
                d_utc: 0.0,
                is_lse: true,
            },
            UtcOffsetTestdata {
                t: GpsTime{ wn: 77, tow:  259200.5 },
                d_utc: 0.0,
                is_lse: true,
            },
            UtcOffsetTestdata {
                t: GpsTime{ wn: 77, tow:  259201.0 },
                d_utc: 1.0,
                is_lse: false,
            },
            UtcOffsetTestdata {
                t: GpsTime{ wn: 77, tow:  259202.0 },
                d_utc: 1.0,
                is_lse: false,
            },
            /* Jan 1 2017 */
            UtcOffsetTestdata {
                t: GpsTime{ wn: 1930, tow:  16.0 },
                d_utc: 17.0,
                is_lse: false,
            },
            UtcOffsetTestdata {
                t: GpsTime{ wn: 1930, tow:  16.5 },
                d_utc: 17.0,
                is_lse: false,
            },
            UtcOffsetTestdata {
                t: GpsTime{ wn: 1930, tow:  17.0 },
                d_utc: 17.0,
                is_lse: true,
            },
            UtcOffsetTestdata {
                t: GpsTime{ wn: 1930, tow:  17.5 },
                d_utc: 17.0,
                is_lse: true,
            },
            UtcOffsetTestdata {
                t: GpsTime{ wn: 1930, tow:  18.0 },
                d_utc: 18.0,
                is_lse: false,
            },
            UtcOffsetTestdata {
                t: GpsTime{ wn: 1930, tow:  18.5 },
                d_utc: 18.0,
                is_lse: false,
            },
            UtcOffsetTestdata {
                t: GpsTime{ wn: 1930, tow:  19.0 },
                d_utc: 18.0,
                is_lse: false,
            },
        ];
        for test_case in test_cases {
            let d_utc = test_case.t.utc_offset_hardcoded();
            let is_lse = test_case.t.is_leap_second_event_hardcoded();

            assert!(d_utc == test_case.d_utc && is_lse == test_case.is_lse, "test_case.t: {:?}, test_case.d_utc: {}, test_case.is_lse: {}, d_utc: {}, is_lse: {}", test_case.t, test_case.d_utc, test_case.is_lse, d_utc, is_lse);
        }
    }

    /* test a fictional leap second on 1st Jan 2020 */
    /* note also the polynomial correction which shifts the time of effectivity */
    fn make_p_neg_offset() -> UtcParams {
        UtcParams::from_components(
            -0.125,
            0.0,
            0.0,
            &GpsTime{ wn: 2080, tow:  0.0 },
            &GpsTime{ wn: 2086, tow:  259218.0 - 0.125 },
            18,
            19,
        )
    }

    fn make_p_pos_offset() -> UtcParams {
        UtcParams::from_components(
            0.125,
            0.0,
            0.0,
            &GpsTime{ wn: 2080, tow:  0.0 },
            &GpsTime{ wn: 2086, tow:  259218.125 },
            18,
            19,
        )
    }

    fn make_p_pos_trend() -> UtcParams {
        UtcParams::from_components(
            0.0,
            1e-12,
            0.0,
            &GpsTime{ wn: 2080, tow:  0.0 },
            &GpsTime{
                wn: 2086,
                tow: 259218.0 + 1e-12 * (6.0 * consts::WEEK_SECS as f64 + 259218.0),
            },
            18,
            19,
        )
    }

    fn make_p_neg_trend() -> UtcParams {
        UtcParams::from_components(
            0.0,
            -1e-12,
            0.0,
            &GpsTime{ wn: 2080, tow:  0.0 },
            &GpsTime{
                wn: 2086,
                tow: 259218.0 - 1e-12 * (6.0 * consts::WEEK_SECS as f64 + 259218.0),
            },
            18,
            19,
        )
    }

    #[test]
    fn utc_params() {
        struct TestCase {
            t: GpsTime,
            d_utc: f64,
            is_lse: bool,
            params: Option<UtcParams>,
        }

        let test_cases = [
            /* Jan 1 2020 (constant negative UTC offset) */
            TestCase {
                t: GpsTime{ wn: 2086, tow:  259217.0 - 0.125 },
                d_utc: 18.0 - 0.125,
                is_lse: false,
                params: Some(make_p_neg_offset()),
            },
            TestCase {
                t: GpsTime{ wn: 2086, tow:  259217.5 - 0.125 },
                d_utc: 18.0 - 0.125,
                is_lse: false,
                params: Some(make_p_neg_offset()),
            },
            TestCase {
                t: GpsTime{ wn: 2086, tow:  259218.0 - 0.125 },
                d_utc: 18.0 - 0.125,
                is_lse: true,
                params: Some(make_p_neg_offset()),
            },
            TestCase {
                t: GpsTime{ wn: 2086, tow:  259218.5 - 0.125 },
                d_utc: 18.0 - 0.125,
                is_lse: true,
                params: Some(make_p_neg_offset()),
            },
            TestCase {
                t: GpsTime{ wn: 2086, tow:  259219.0 - 0.125 },
                d_utc: 19.0 - 0.125,
                is_lse: false,
                params: Some(make_p_neg_offset()),
            },
            TestCase {
                t: GpsTime{ wn: 2086, tow:  259219.5 - 0.125 },
                d_utc: 19.0 - 0.125,
                is_lse: false,
                params: Some(make_p_neg_offset()),
            },
            /* Jan 1 2020 (constant positive UTC offset) */
            TestCase {
                t: GpsTime{ wn: 2086, tow:  259217.125 },
                d_utc: 18.125,
                is_lse: false,
                params: Some(make_p_pos_offset()),
            },
            TestCase {
                t: GpsTime{ wn: 2086, tow:  259217.5 + 0.125 },
                d_utc: 18.125,
                is_lse: false,
                params: Some(make_p_pos_offset()),
            },
            TestCase {
                t: GpsTime{ wn: 2086, tow:  259218.125 },
                d_utc: 18.125,
                is_lse: true,
                params: Some(make_p_pos_offset()),
            },
            TestCase {
                t: GpsTime{ wn: 2086, tow:  259218.5 + 0.125 },
                d_utc: 18.125,
                is_lse: true,
                params: Some(make_p_pos_offset()),
            },
            TestCase {
                t: GpsTime{ wn: 2086, tow:  259219.125 },
                d_utc: 19.125,
                is_lse: false,
                params: Some(make_p_pos_offset()),
            },
            TestCase {
                t: GpsTime{ wn: 2086, tow:  259219.5 + 0.125 },
                d_utc: 19.125,
                is_lse: false,
                params: Some(make_p_pos_offset()),
            },
            /* Jan 1 2020 (positive UTC linear correction) */
            TestCase {
                t: GpsTime{ wn: 2086, tow:  259217.0 },
                d_utc: 18.0,
                is_lse: false,
                params: Some(make_p_pos_trend()),
            },
            TestCase {
                t: GpsTime{ wn: 2086, tow:  259217.5 },
                d_utc: 18.0,
                is_lse: false,
                params: Some(make_p_pos_trend()),
            },
            TestCase {
                t: GpsTime{ wn: 2086, tow:  259218.0001 },
                d_utc: 18.0,
                is_lse: true,
                params: Some(make_p_pos_trend()),
            },
            TestCase {
                t: GpsTime{ wn: 2086, tow:  259218.5 },
                d_utc: 18.0,
                is_lse: true,
                params: Some(make_p_pos_trend()),
            },
            TestCase {
                t: GpsTime{ wn: 2086, tow:  259219.0001 },
                d_utc: 19.0,
                is_lse: false,
                params: Some(make_p_pos_trend()),
            },
            TestCase {
                t: GpsTime{ wn: 2086, tow:  259219.5 },
                d_utc: 19.0,
                is_lse: false,
                params: Some(make_p_pos_trend()),
            },
            /* Jan 1 2020 (negative UTC linear correction) */
            TestCase {
                t: GpsTime{ wn: 2086, tow:  259217.0 },
                d_utc: 18.0,
                is_lse: false,
                params: Some(make_p_neg_trend()),
            },
            TestCase {
                t: GpsTime{ wn: 2086, tow:  259217.5 },
                d_utc: 18.0,
                is_lse: false,
                params: Some(make_p_neg_trend()),
            },
            TestCase {
                t: GpsTime{ wn: 2086, tow:  259218.0 },
                d_utc: 18.0,
                is_lse: true,
                params: Some(make_p_neg_trend()),
            },
            TestCase {
                t: GpsTime{ wn: 2086, tow:  259218.5 },
                d_utc: 18.0,
                is_lse: true,
                params: Some(make_p_neg_trend()),
            },
            TestCase {
                t: GpsTime{ wn: 2086, tow:  259219.0 },
                d_utc: 19.0,
                is_lse: false,
                params: Some(make_p_neg_trend()),
            },
            TestCase {
                t: GpsTime{ wn: 2086, tow:  259219.5 },
                d_utc: 19.0,
                is_lse: false,
                params: Some(make_p_neg_trend()),
            },
        ];

        for test_case in test_cases {
            let is_lse = if let Some(params) = &test_case.params {
                test_case.t.is_leap_second_event(params)
            } else {
                test_case.t.is_leap_second_event_hardcoded()
            };
            assert_eq!(is_lse, test_case.is_lse);

            let d_utc = if let Some(params) = &test_case.params {
                test_case.t.utc_offset(params)
            } else {
                test_case.t.utc_offset_hardcoded()
            };
            assert!(
                (d_utc - test_case.d_utc).abs() < 1e-5,
                "d_utc: {} test_case.d_utc: {} test_case.t.tow: {}",
                d_utc,
                test_case.d_utc,
                test_case.t.tow()
            );
        }
    }

    #[test]
    fn gps2utc() {
        /* test leap second on 1st Jan 2020 */
        /* note also the polynomial correction which shifts the time of effectivity */

        struct UtcExpectation {
            year: u16,
            month: u8,
            day: u8,
            hour: u8,
            minute: u8,
            second: f64,
        }

        impl UtcExpectation {
            pub fn new(
                year: u16,
                month: u8,
                day: u8,
                hour: u8,
                minute: u8,
                second: f64,
            ) -> UtcExpectation {
                UtcExpectation {
                    year,
                    month,
                    day,
                    hour,
                    minute,
                    second,
                }
            }
        }

        struct TestCase {
            t: GpsTime,
            u: UtcExpectation,
            p: Option<UtcParams>,
        }

        let test_cases = [
            /* July 1 1981 */
            TestCase {
                t: GpsTime{ wn: 77, tow:  259199.0 },
                u: UtcExpectation::new(1981, 6, 30, 23, 59, 59.0),
                p: None,
            },
            TestCase {
                t: GpsTime{ wn: 77, tow:  259199.5 },
                u: UtcExpectation::new(1981, 6, 30, 23, 59, 59.5),
                p: None,
            },
            TestCase {
                t: GpsTime{ wn: 77, tow:  259200.0 },
                u: UtcExpectation::new(1981, 6, 30, 23, 59, 60.0),
                p: None,
            },
            TestCase {
                t: GpsTime{ wn: 77, tow:  259200.5 },
                u: UtcExpectation::new(1981, 6, 30, 23, 59, 60.5),
                p: None,
            },
            TestCase {
                t: GpsTime{ wn: 77, tow:  259201.0 },
                u: UtcExpectation::new(1981, 7, 01, 00, 00, 00.0),
                p: None,
            },
            /* Jan 1 2017 */
            TestCase {
                t: GpsTime{ wn: 1930, tow:  16.0 },
                u: UtcExpectation::new(2016, 12, 31, 23, 59, 59.0),
                p: None,
            },
            TestCase {
                t: GpsTime{ wn: 1930, tow:  16.5 },
                u: UtcExpectation::new(2016, 12, 31, 23, 59, 59.5),
                p: None,
            },
            TestCase {
                t: GpsTime{ wn: 1930, tow:  17.0 },
                u: UtcExpectation::new(2016, 12, 31, 23, 59, 60.0),
                p: None,
            },
            TestCase {
                t: GpsTime{ wn: 1930, tow:  17.5 },
                u: UtcExpectation::new(2016, 12, 31, 23, 59, 60.5),
                p: None,
            },
            TestCase {
                t: GpsTime{ wn: 1930, tow:  18.0 },
                u: UtcExpectation::new(2017, 01, 01, 00, 00, 00.0),
                p: None,
            },
            /* Jan 8 2017 */
            TestCase {
                t: GpsTime{ wn: 1931, tow:  17.0 },
                u: UtcExpectation::new(2017, 01, 7, 23, 59, 59.0),
                p: None,
            },
            TestCase {
                t: GpsTime{ wn: 1931, tow:  17.5 },
                u: UtcExpectation::new(2017, 01, 7, 23, 59, 59.5),
                p: None,
            },
            TestCase {
                t: GpsTime{ wn: 1931, tow:  18.0 - 6e-11 },
                u: UtcExpectation::new(2017, 01, 7, 23, 59, 59.0 + 1.0 - 6e-11),
                p: None,
            },
            TestCase {
                t: GpsTime{ wn: 1931, tow:  18.0 - 5e-11 },
                u: UtcExpectation::new(2017, 01, 8, 00, 00, 00.0),
                p: None,
            },
            TestCase {
                t: GpsTime{ wn: 1931, tow:  18.0 },
                u: UtcExpectation::new(2017, 01, 8, 00, 00, 00.0),
                p: None,
            },
            /* Jan 1 2020 (leap second announced in utc_params_t above, constant
            negative offset) */
            TestCase {
                t: GpsTime{ wn: 2086, tow:  259217.0 - 0.125 },
                u: UtcExpectation::new(2019, 12, 31, 23, 59, 59.0),
                p: Some(make_p_neg_offset()),
            },
            TestCase {
                t: GpsTime{ wn: 2086, tow:  259217.5 - 0.125 },
                u: UtcExpectation::new(2019, 12, 31, 23, 59, 59.5),
                p: Some(make_p_neg_offset()),
            },
            TestCase {
                t: GpsTime{ wn: 2086, tow:  259218.0 - 0.125 },
                u: UtcExpectation::new(2019, 12, 31, 23, 59, 60.0),
                p: Some(make_p_neg_offset()),
            },
            TestCase {
                t: GpsTime{ wn: 2086, tow:  259218.5 - 0.125 },
                u: UtcExpectation::new(2019, 12, 31, 23, 59, 60.5),
                p: Some(make_p_neg_offset()),
            },
            TestCase {
                t: GpsTime{ wn: 2086, tow:  259219.0 - 0.125 },
                u: UtcExpectation::new(2020, 01, 01, 00, 00, 00.0),
                p: Some(make_p_neg_offset()),
            },
            /* Jan 1 2020 (leap second announced in utc_params_t above, constant
            positive offset) */
            TestCase {
                t: GpsTime{ wn: 2086, tow:  259217.125 },
                u: UtcExpectation::new(2019, 12, 31, 23, 59, 59.0),
                p: Some(make_p_pos_offset()),
            },
            TestCase {
                t: GpsTime{ wn: 2086, tow:  259217.5 + 0.125 },
                u: UtcExpectation::new(2019, 12, 31, 23, 59, 59.5),
                p: Some(make_p_pos_offset()),
            },
            TestCase {
                t: GpsTime{ wn: 2086, tow:  259218.125 },
                u: UtcExpectation::new(2019, 12, 31, 23, 59, 60.0),
                p: Some(make_p_pos_offset()),
            },
            TestCase {
                t: GpsTime{ wn: 2086, tow:  259218.5 + 0.125 },
                u: UtcExpectation::new(2019, 12, 31, 23, 59, 60.5),
                p: Some(make_p_pos_offset()),
            },
            TestCase {
                t: GpsTime{ wn: 2086, tow:  259219.125 },
                u: UtcExpectation::new(2020, 01, 01, 00, 00, 00.0),
                p: Some(make_p_pos_offset()),
            },
            /* Jan 1 2020 (leap second announced in utc_params_t above, positive UTC
            linear correction) */
            TestCase {
                t: GpsTime{ wn: 2086, tow:  259217.0 },
                u: UtcExpectation::new(2019, 12, 31, 23, 59, 59.0),
                p: Some(make_p_pos_trend()),
            },
            TestCase {
                t: GpsTime{ wn: 2086, tow:  259217.5 },
                u: UtcExpectation::new(2019, 12, 31, 23, 59, 59.5),
                p: Some(make_p_pos_trend()),
            },
            TestCase {
                t: GpsTime{ wn: 2086, tow:  259218.0 },
                u: UtcExpectation::new(2019, 12, 31, 23, 59, 60.0),
                p: Some(make_p_pos_trend()),
            },
            TestCase {
                t: GpsTime{ wn: 2086, tow:  259218.5 },
                u: UtcExpectation::new(2019, 12, 31, 23, 59, 60.5),
                p: Some(make_p_pos_trend()),
            },
            TestCase {
                t: GpsTime{ wn: 2086, tow:  259219.00001 },
                u: UtcExpectation::new(2020, 01, 01, 00, 00, 00.0),
                p: Some(make_p_pos_trend()),
            },
            /* Jan 1 2020 (leap second announced in utc_params_t above, negative UTC
            linear correction) */
            TestCase {
                t: GpsTime{ wn: 2086, tow:  259217.0 },
                u: UtcExpectation::new(2019, 12, 31, 23, 59, 59.0),
                p: Some(make_p_neg_trend()),
            },
            TestCase {
                t: GpsTime{ wn: 2086, tow:  259217.5 },
                u: UtcExpectation::new(2019, 12, 31, 23, 59, 59.5),
                p: Some(make_p_neg_trend()),
            },
            TestCase {
                t: GpsTime{ wn: 2086, tow:  259218.0 },
                u: UtcExpectation::new(2019, 12, 31, 23, 59, 60.0),
                p: Some(make_p_neg_trend()),
            },
            TestCase {
                t: GpsTime{ wn: 2086, tow:  259218.5 },
                u: UtcExpectation::new(2019, 12, 31, 23, 59, 60.5),
                p: Some(make_p_neg_trend()),
            },
            TestCase {
                t: GpsTime{ wn: 2086, tow:  259219.0 },
                u: UtcExpectation::new(2020, 01, 01, 00, 00, 00.0),
                p: Some(make_p_neg_trend()),
            },
        ];

        for test_case in test_cases {
            let expected = &test_case.u;
            let u = if let Some(p) = &test_case.p {
                test_case.t.to_utc(p)
            } else {
                test_case.t.to_utc_hardcoded()
            };

            assert_eq!(u.year(), expected.year, "u.year: {}, expected.year: {}, tow: {}", u.year(), expected.year, test_case.t.tow());
            assert_eq!(u.month(), expected.month, "u.month: {}, expected.month: {}, tow: {}", u.month(), expected.month, test_case.t.tow());
            assert_eq!(u.day_of_month(), expected.day, "u.day_of_month: {}, expected.day: {}, tow: {}", u.day_of_month(), expected.day, test_case.t.tow());
            assert_eq!(u.hour(), expected.hour, "u.hour: {}, expected.hour: {}, tow: {}", u.hour(), expected.hour, test_case.t.tow());
            assert_eq!(u.minute(), expected.minute, "u.minute: {}, expected.minute: {}, tow: {}", u.minute(), expected.minute, test_case.t.tow());
            assert!(
                (u.seconds() - expected.second).abs() < 1e-5,
                "{} {} {}",
                u.seconds(),
                expected.second,
                test_case.t.tow()
            );
        }
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn chrono_conversions() {
        use chrono::prelude::*;
        let epsilon = std::time::Duration::from_secs_f64(1e-6);
        let swift_date = UtcTime::from_date(2021, 8, 1, 00, 11, 0.0);
        let expected_utc = DateTime::from_naive_utc_and_offset(
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2021, 8, 1).unwrap(),
                NaiveTime::from_hms_nano_opt(00, 11, 0, 0).unwrap(),
            ),
            Utc,
        );

        let converted: DateTime<Utc> = swift_date.clone().into();
        assert!((converted - expected_utc).to_std().unwrap() < epsilon);
        assert_eq!(converted.year(), swift_date.year() as i32);
        assert_eq!(converted.month(), swift_date.month() as u32);
        assert_eq!(converted.day(), swift_date.day_of_month() as u32);
        assert_eq!(converted.hour(), swift_date.hour() as u32);
        assert_eq!(converted.minute(), swift_date.minute() as u32);
        assert_eq!(converted.second(), swift_date.seconds() as u32);
    }

    #[test]
    fn gps_to_gal() {
        let gal = GAL_TIME_START.to_gal();
        assert_eq!(gal.wn(), 0);
        assert!(gal.tow().abs() < 1e-9);
        let gps = gal.to_gps();
        assert_eq!(gps.wn(), consts::GAL_WEEK_TO_GPS_WEEK as i16);
        assert!(gps.tow().abs() < 1e-9);

        assert!(GalTime::new(-1, 0.0).is_err());
        assert!(GalTime::new(0, -1.0).is_err());
        assert!(GalTime::new(0, consts::WEEK_SECS as f64 + 1.0).is_err());
    }

    #[test]
    fn gps_to_bds() {
        let bds = BDS_TIME_START.to_bds();
        assert_eq!(bds.wn(), 0);
        assert!(bds.tow().abs() < 1e-9);
        let gps = bds.to_gps();
        assert_eq!(gps.wn(), consts::BDS_WEEK_TO_GPS_WEEK as i16);
        assert!((gps.tow() - consts::BDS_SECOND_TO_GPS_SECOND).abs() < 1e-9);

        assert!(BdsTime::new(-1, 0.0).is_err());
        assert!(BdsTime::new(0, -1.0).is_err());
        assert!(BdsTime::new(0, consts::WEEK_SECS as f64 + 1.0).is_err());
    }

    #[test]
    fn is_leap_year() {
        use super::is_leap_year;
        assert!(!is_leap_year(2019));
        assert!(is_leap_year(2020));
        assert!(!is_leap_year(1900));
        assert!(is_leap_year(2000));
    }
}
