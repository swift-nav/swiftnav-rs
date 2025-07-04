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

use std::error::Error;
use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::time::Duration;

pub const MINUTE: Duration = Duration::from_secs(swiftnav_sys::MINUTE_SECS as u64);
pub const HOUR: Duration = Duration::from_secs(swiftnav_sys::HOUR_SECS as u64);
pub const DAY: Duration = Duration::from_secs(swiftnav_sys::DAY_SECS as u64);
pub const WEEK: Duration = Duration::from_secs(swiftnav_sys::WEEK_SECS as u64);

/// Representation of GPS Time
#[derive(Copy, Clone)]
pub struct GpsTime(swiftnav_sys::gps_time_t);

/// GPS timestamp of the start of Galileo time
pub const GAL_TIME_START: GpsTime =
    GpsTime::new_unchecked(swiftnav_sys::GAL_WEEK_TO_GPS_WEEK as i16, 0.0);
/// GPS timestamp of the start of Beidou time
pub const BDS_TIME_START: GpsTime = GpsTime::new_unchecked(
    swiftnav_sys::BDS_WEEK_TO_GPS_WEEK as i16,
    swiftnav_sys::BDS_SECOND_TO_GPS_SECOND as f64,
);
/// GPS timestamp of the start of Glonass time
pub const GLO_TIME_START: GpsTime = GpsTime::new_unchecked(
    swiftnav_sys::GLO_EPOCH_WN as i16,
    swiftnav_sys::GLO_EPOCH_TOW,
);

/// Error type when a given GPS time is not valid
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum InvalidGpsTime {
    /// Indicates an invalid week number was given, with the invalid value returned
    InvalidWN(i16),
    /// Indicates an invalid time of week was given, with the invalid value returned
    InvalidTOW(f64),
}

impl fmt::Display for InvalidGpsTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InvalidGpsTime::InvalidWN(wn) => write!(f, "Invalid Week Number: {wn}"),
            InvalidGpsTime::InvalidTOW(tow) => write!(f, "Invalid Time of Week: {tow}"),
        }
    }
}

impl Error for InvalidGpsTime {}

impl GpsTime {
    const JIFFY: f64 = swiftnav_sys::FLOAT_EQUALITY_EPS;

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
    pub(crate) const fn new_unchecked(wn: i16, tow: f64) -> GpsTime {
        GpsTime(swiftnav_sys::gps_time_t { wn, tow })
    }

    pub(crate) fn to_gps_time_t(self) -> swiftnav_sys::gps_time_t {
        self.0
    }

    pub(crate) fn c_ptr(&self) -> *const swiftnav_sys::gps_time_t {
        &self.0
    }

    pub(crate) fn mut_c_ptr(&mut self) -> *mut swiftnav_sys::gps_time_t {
        &mut self.0
    }

    pub(crate) fn unknown() -> swiftnav_sys::gps_time_t {
        swiftnav_sys::gps_time_t { tow: -1.0, wn: -1 }
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
        unsafe { swiftnav_sys::gps_time_valid(&self.0) }
    }

    /// Adds a duration to the time
    pub fn add_duration(&mut self, duration: &Duration) {
        unsafe {
            swiftnav_sys::add_secs(&mut self.0, duration.as_secs_f64());
        }
    }

    /// Subtracts a duration from the time
    pub fn subtract_duration(&mut self, duration: &Duration) {
        unsafe {
            swiftnav_sys::add_secs(&mut self.0, -duration.as_secs_f64());
        }
    }

    /// Gets the difference between this and another time value in seconds
    pub fn diff(&self, other: &Self) -> f64 {
        unsafe { swiftnav_sys::gpsdifftime(&self.0, &other.0) }
    }

    /// Converts the GPS time into UTC time
    ///
    /// # Panics
    /// This function will panic if the GPS time is not valid
    pub fn to_utc(self, utc_params: &UtcParams) -> UtcTime {
        assert!(self.is_valid());

        let mut utc = UtcTime::default();
        unsafe {
            swiftnav_sys::gps2utc(self.c_ptr(), utc.mut_c_ptr(), utc_params.c_ptr());
        }
        utc
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
        assert!(self.is_valid());

        let mut utc = UtcTime::default();
        unsafe {
            swiftnav_sys::gps2utc(self.c_ptr(), utc.mut_c_ptr(), std::ptr::null());
        }
        utc
    }

    /// Gets the number of seconds difference between GPS and UTC times
    pub fn utc_offset(&self, utc_params: &UtcParams) -> f64 {
        unsafe { swiftnav_sys::get_gps_utc_offset(self.c_ptr(), utc_params.c_ptr()) }
    }

    /// Gets the number of seconds difference between GPS and UTC using the hardcoded
    /// list of leap seconds
    ///
    /// # ⚠️  🦘  ⏱  ⚠️  - Leap Seconds
    /// The hard coded list of leap seconds will get out of date, it is
    /// preferable to use [`GpsTime::utc_offset()`] with the newest set
    /// of UTC parameters
    pub fn utc_offset_hardcoded(&self) -> f64 {
        unsafe { swiftnav_sys::get_gps_utc_offset(self.c_ptr(), std::ptr::null()) }
    }

    /// Checks to see if this point in time is a UTC leap second event
    pub fn is_leap_second_event(&self, utc_params: &UtcParams) -> bool {
        unsafe { swiftnav_sys::is_leap_second_event(self.c_ptr(), utc_params.c_ptr()) }
    }

    /// Checks to see if this point in time is a UTC leap second event using the
    /// hardcoded list of leap seconds
    ///
    /// # ⚠️  🦘  ⏱  ⚠️  - Leap Seconds
    /// The hard coded list of leap seconds will get out of date, it is
    /// preferable to use [`GpsTime::is_leap_second_event()`] with the newest
    /// set of UTC parameters
    pub fn is_leap_second_event_hardcoded(&self) -> bool {
        unsafe { swiftnav_sys::is_leap_second_event(self.c_ptr(), std::ptr::null()) }
    }

    /// Gets the GPS time of the nearest solution epoch
    pub fn round_to_epoch(&self, soln_freq: f64) -> GpsTime {
        GpsTime(unsafe { swiftnav_sys::round_to_epoch(self.c_ptr(), soln_freq) })
    }

    /// Gets the GPS time of the previous solution epoch
    pub fn floor_to_epoch(&self, soln_freq: f64) -> GpsTime {
        GpsTime(unsafe { swiftnav_sys::floor_to_epoch(self.c_ptr(), soln_freq) })
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
            wn: self.wn() - swiftnav_sys::GAL_WEEK_TO_GPS_WEEK as i16,
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
        let bds = GpsTime::new_unchecked(
            self.wn() - swiftnav_sys::BDS_WEEK_TO_GPS_WEEK as i16,
            self.tow(),
        );
        let bds = bds - Duration::from_secs(swiftnav_sys::BDS_SECOND_TO_GPS_SECOND as u64);
        BdsTime {
            wn: bds.wn(),
            tow: bds.tow(),
        }
    }

    /// Converts a GPS time into a Glonass time
    ///
    /// # Panics
    /// This function will panic if the GPS time is before the start of Glonass
    /// time, i.e. [`GLO_TIME_START`]
    pub fn to_glo(self, utc_params: &UtcParams) -> GloTime {
        assert!(self.is_valid());
        assert!(self >= GLO_TIME_START);
        GloTime(unsafe { swiftnav_sys::gps2glo(self.c_ptr(), utc_params.c_ptr()) })
    }

    /// Converts a GPS time into a Glonass time using the hardcoded list of leap
    /// seconds.
    ///
    /// # ⚠️  🦘  ⏱  ⚠️  - Leap Seconds
    /// The hard coded list of leap seconds will get out of date, it is
    /// preferable to use [`GpsTime::to_glo()`] with the newest set of UTC parameters
    ///
    /// # Panics
    /// This function will panic if the GPS time is before the start of Glonass
    /// time, i.e. [`GLO_TIME_START`]
    pub fn to_glo_hardcoded(self) -> GloTime {
        assert!(self.is_valid());
        assert!(self >= GLO_TIME_START);
        GloTime(unsafe { swiftnav_sys::gps2glo(self.c_ptr(), std::ptr::null()) })
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
        let diff_seconds = self.diff(&rhs);
        Duration::from_secs_f64(diff_seconds)
    }
}

impl Sub<&GpsTime> for GpsTime {
    type Output = Duration;
    fn sub(self, rhs: &GpsTime) -> Duration {
        let diff_seconds = self.diff(rhs);
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
        GpsTime::new_unchecked(
            self.wn + swiftnav_sys::GAL_WEEK_TO_GPS_WEEK as i16,
            self.tow,
        )
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
        let gps = GpsTime::new_unchecked(
            self.wn() + swiftnav_sys::BDS_WEEK_TO_GPS_WEEK as i16,
            self.tow(),
        );
        gps + Duration::from_secs(swiftnav_sys::BDS_SECOND_TO_GPS_SECOND as u64)
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

/// Representation of Glonass Time
#[derive(Copy, Clone)]
pub struct GloTime(swiftnav_sys::glo_time_t);

impl GloTime {
    pub(crate) fn c_ptr(&self) -> *const swiftnav_sys::glo_time_t {
        &self.0
    }

    /// Creates a new GloTime
    /// nt - Day number within the four-year interval [1-1461].
    ///      Comes from the field NT in the GLO string 4.
    ///
    /// n4 - Four-year interval number starting from 1996 [1- ].
    ///      Comes from the field N4 in the GLO string 5.
    ///
    /// h/m/s come either from the field tb in the GLO string 2
    ///      or the field tk in the GLO string 1
    /// h - Hours [0-24]
    /// m - Minutes [0-59]
    /// s - Seconds [0-60]
    pub fn new(nt: u16, n4: u8, h: u8, m: u8, s: f64) -> GloTime {
        GloTime(swiftnav_sys::glo_time_t { nt, n4, h, m, s })
    }

    pub fn nt(&self) -> u16 {
        self.0.nt
    }

    pub fn n4(&self) -> u8 {
        self.0.n4
    }

    pub fn h(&self) -> u8 {
        self.0.h
    }

    pub fn m(&self) -> u8 {
        self.0.m
    }

    pub fn s(&self) -> f64 {
        self.0.s
    }

    /// Converts a Glonass time into a GPS time
    pub fn to_gps(self, utc_params: &UtcParams) -> GpsTime {
        GpsTime(unsafe { swiftnav_sys::glo2gps(self.c_ptr(), utc_params.c_ptr()) })
    }

    /// Converts a Glonass time into a GPS time using the hardcoded list of leap
    /// seconds.
    ///
    /// Note: The hard coded list of leap seconds will get out of date, it is
    /// preferable to use [`GloTime::to_gps()`] with the newest set of UTC parameters
    pub fn to_gps_hardcoded(self) -> GpsTime {
        GpsTime(unsafe { swiftnav_sys::glo2gps(self.c_ptr(), std::ptr::null()) })
    }
}

/// GPS UTC correction parameters
#[derive(Clone)]
pub struct UtcParams(swiftnav_sys::utc_params_t);

impl UtcParams {
    pub(crate) fn mut_c_ptr(&mut self) -> *mut swiftnav_sys::utc_params_t {
        &mut self.0
    }

    pub(crate) fn c_ptr(&self) -> *const swiftnav_sys::utc_params_t {
        &self.0
    }

    /// Decodes UTC parameters from GPS LNAV message subframe 4 words 6-10.
    ///
    /// Note: Fills out the full time of week from current gps week cycle. Also
    /// sets t_lse to the exact GPS time at the start of the leap second event.
    ///
    /// # References
    ///   * IS-GPS-200H, Section 20.3.3.5.1.6
    pub fn decode(words: &[u32; 8]) -> Option<Self> {
        let mut params = UtcParams::default();
        let result = unsafe { swiftnav_sys::decode_utc_parameters(words, params.mut_c_ptr()) };

        if result {
            Some(params)
        } else {
            None
        }
    }

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

        let tot = tot.to_gps_time_t();
        let t_lse = t_lse.to_gps_time_t();
        UtcParams(swiftnav_sys::utc_params_t {
            a0,
            a1,
            a2,
            tot,
            t_lse,
            dt_ls,
            dt_lsf,
        })
    }

    /// Modulo 1 sec offset from GPS to UTC \[s\]
    pub fn a0(&self) -> f64 {
        self.0.a0
    }
    /// Drift of time offset from GPS to UTC \[s/s\]
    pub fn a1(&self) -> f64 {
        self.0.a1
    }
    /// Drift rate correction from GPS to UTC \[s/s\]
    pub fn a2(&self) -> f64 {
        self.0.a2
    }
    /// Reference time of UTC parameters.
    pub fn tot(&self) -> GpsTime {
        GpsTime(self.0.tot)
    }
    /// Time of leap second event.
    pub fn t_lse(&self) -> GpsTime {
        GpsTime(self.0.t_lse)
    }
    /// Leap second delta from GPS to UTC before LS event \[s\]
    pub fn dt_ls(&self) -> i8 {
        self.0.dt_ls
    }
    /// Leap second delta from GPS to UTC after LS event \[s\]
    pub fn dt_lsf(&self) -> i8 {
        self.0.dt_lsf
    }
}

impl Default for UtcParams {
    fn default() -> Self {
        unsafe { std::mem::zeroed::<UtcParams>() }
    }
}

/// Representation of UTC time
///
/// Note: This implementation does not aim to be able to represent arbitrary dates and times.
/// It is only meant to represent dates and times over the period that GNSS systems have been
/// around. Specifically it shouldn't be relied on for dates significantly before January 6th 1980,
/// the start of GPS time.
#[derive(Clone)]
pub struct UtcTime(swiftnav_sys::utc_tm);

impl UtcTime {
    pub(crate) fn mut_c_ptr(&mut self) -> *mut swiftnav_sys::utc_tm {
        &mut self.0
    }

    pub(crate) fn c_ptr(&self) -> *const swiftnav_sys::utc_tm {
        &self.0
    }

    /// Creates a UTC time from its individual components
    pub fn from_date(year: u16, month: u8, day: u8, hour: u8, minute: u8, second: f64) -> UtcTime {
        UtcTime(unsafe {
            swiftnav_sys::date2utc(
                year as i32,
                month as i32,
                day as i32,
                hour as i32,
                minute as i32,
                second,
            )
        })
    }

    /// Number of years CE. In four digit format
    pub fn year(&self) -> u16 {
        self.0.year
    }

    /// Day of the year (1 - 366)
    pub fn day_of_year(&self) -> u16 {
        self.0.year_day
    }

    /// Month of the year (1 - 12). 1 = January, 12 = December
    pub fn month(&self) -> u8 {
        self.0.month
    }

    /// Day of the month (1 - 31)
    pub fn day_of_month(&self) -> u8 {
        self.0.month_day
    }

    /// Day of the week (1 - 7). 1 = Monday, 7 = Sunday
    pub fn day_of_week(&self) -> u8 {
        self.0.week_day
    }

    /// Hour of the day (0 - 23)
    pub fn hour(&self) -> u8 {
        self.0.hour
    }

    /// Minutes of the hour (0 - 59)
    pub fn minute(&self) -> u8 {
        self.0.minute
    }

    /// seconds of the minute (0 - 60)
    pub fn seconds(&self) -> f64 {
        (self.0.second_int as f64) + self.0.second_frac
    }

    /// Converts the UTC time into a modified julian date
    pub fn to_mjd(&self) -> MJD {
        MJD(unsafe { swiftnav_sys::utc2mjd(self.c_ptr()) })
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

    /// Converts the UTC time into GPS time
    pub fn to_gps(&self, utc_params: &UtcParams) -> GpsTime {
        let mut gps = GpsTime::new_unchecked(0, 0.0);
        unsafe {
            swiftnav_sys::utc2gps(self.c_ptr(), gps.mut_c_ptr(), utc_params.c_ptr());
        }
        gps
    }

    /// Converts the UTC time into GPS time using the hardcoded list of leap
    /// seconds.
    ///
    /// # ⚠️  🦘  ⏱  ⚠️  - Leap Seconds
    /// The hard coded list of leap seconds will get out of date, it is
    /// preferable to use [`UtcTime::to_gps()`] with the newest set of UTC parameters
    pub fn to_gps_hardcoded(&self) -> GpsTime {
        let mut gps = GpsTime::new_unchecked(0, 0.0);
        unsafe {
            swiftnav_sys::utc2gps(self.c_ptr(), gps.mut_c_ptr(), std::ptr::null());
        }
        gps
    }

    pub fn to_fractional_year(&self) -> f64 {
        let year = self.year() as f64;
        let days = self.day_of_year() as f64;
        let hours = self.hour() as f64;
        let minutes = self.minute() as f64;
        let seconds = self.seconds();
        let total_days = days + hours / 24. + minutes / 1440. + seconds / 86400.;

        if is_leap_year(self.year()) {
            year + total_days / 366.0
        } else {
            year + total_days / 365.0
        }
    }
}

impl Default for UtcTime {
    fn default() -> Self {
        unsafe { std::mem::zeroed::<UtcTime>() }
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
        MJD(unsafe {
            swiftnav_sys::date2mjd(
                year as i32,
                month as i32,
                day as i32,
                hour as i32,
                minute as i32,
                seconds,
            )
        })
    }

    /// Gets the floating point value of the modified julian date
    pub fn as_f64(&self) -> f64 {
        self.0
    }

    /// Converts the modified julian date into a UTC time
    pub fn to_utc(&self) -> UtcTime {
        UtcTime(unsafe { swiftnav_sys::mjd2utc(self.0) })
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
                t: GpsTime::new_unchecked(77, 259199.0),
                d_utc: 0.0,
                is_lse: false,
            },
            UtcOffsetTestdata {
                t: GpsTime::new_unchecked(77, 259199.5),
                d_utc: 0.0,
                is_lse: false,
            },
            UtcOffsetTestdata {
                t: GpsTime::new_unchecked(77, 259200.0),
                d_utc: 0.0,
                is_lse: true,
            },
            UtcOffsetTestdata {
                t: GpsTime::new_unchecked(77, 259200.5),
                d_utc: 0.0,
                is_lse: true,
            },
            UtcOffsetTestdata {
                t: GpsTime::new_unchecked(77, 259201.0),
                d_utc: 1.0,
                is_lse: false,
            },
            UtcOffsetTestdata {
                t: GpsTime::new_unchecked(77, 259202.0),
                d_utc: 1.0,
                is_lse: false,
            },
            /* Jan 1 2017 */
            UtcOffsetTestdata {
                t: GpsTime::new_unchecked(1930, 16.0),
                d_utc: 17.0,
                is_lse: false,
            },
            UtcOffsetTestdata {
                t: GpsTime::new_unchecked(1930, 16.5),
                d_utc: 17.0,
                is_lse: false,
            },
            UtcOffsetTestdata {
                t: GpsTime::new_unchecked(1930, 17.0),
                d_utc: 17.0,
                is_lse: true,
            },
            UtcOffsetTestdata {
                t: GpsTime::new_unchecked(1930, 17.5),
                d_utc: 17.0,
                is_lse: true,
            },
            UtcOffsetTestdata {
                t: GpsTime::new_unchecked(1930, 18.0),
                d_utc: 18.0,
                is_lse: false,
            },
            UtcOffsetTestdata {
                t: GpsTime::new_unchecked(1930, 18.5),
                d_utc: 18.0,
                is_lse: false,
            },
            UtcOffsetTestdata {
                t: GpsTime::new_unchecked(1930, 19.0),
                d_utc: 18.0,
                is_lse: false,
            },
        ];
        for test_case in test_cases {
            let d_utc = test_case.t.utc_offset_hardcoded();
            let is_lse = test_case.t.is_leap_second_event_hardcoded();

            assert!(d_utc == test_case.d_utc && is_lse == test_case.is_lse);
        }
    }

    /* test a fictional leap second on 1st Jan 2020 */
    /* note also the polynomial correction which shifts the time of effectivity */
    fn make_p_neg_offset() -> UtcParams {
        UtcParams::from_components(
            -0.125,
            0.0,
            0.0,
            &GpsTime::new_unchecked(2080, 0.0),
            &GpsTime::new_unchecked(2086, 259218.0 - 0.125),
            18,
            19,
        )
    }

    fn make_p_pos_offset() -> UtcParams {
        UtcParams::from_components(
            0.125,
            0.0,
            0.0,
            &GpsTime::new_unchecked(2080, 0.0),
            &GpsTime::new_unchecked(2086, 259218.125),
            18,
            19,
        )
    }

    fn make_p_pos_trend() -> UtcParams {
        UtcParams::from_components(
            0.0,
            1e-12,
            0.0,
            &GpsTime::new_unchecked(2080, 0.0),
            &GpsTime::new_unchecked(
                2086,
                259218.0 + 1e-12 * (6.0 * swiftnav_sys::WEEK_SECS as f64 + 259218.0),
            ),
            18,
            19,
        )
    }

    fn make_p_neg_trend() -> UtcParams {
        UtcParams::from_components(
            0.0,
            -1e-12,
            0.0,
            &GpsTime::new_unchecked(2080, 0.0),
            &GpsTime::new_unchecked(
                2086,
                259218.0 - 1e-12 * (6.0 * swiftnav_sys::WEEK_SECS as f64 + 259218.0),
            ),
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
                t: GpsTime::new_unchecked(2086, 259217.0 - 0.125),
                d_utc: 18.0 - 0.125,
                is_lse: false,
                params: Some(make_p_neg_offset()),
            },
            TestCase {
                t: GpsTime::new_unchecked(2086, 259217.5 - 0.125),
                d_utc: 18.0 - 0.125,
                is_lse: false,
                params: Some(make_p_neg_offset()),
            },
            TestCase {
                t: GpsTime::new_unchecked(2086, 259218.0 - 0.125),
                d_utc: 18.0 - 0.125,
                is_lse: true,
                params: Some(make_p_neg_offset()),
            },
            TestCase {
                t: GpsTime::new_unchecked(2086, 259218.5 - 0.125),
                d_utc: 18.0 - 0.125,
                is_lse: true,
                params: Some(make_p_neg_offset()),
            },
            TestCase {
                t: GpsTime::new_unchecked(2086, 259219.0 - 0.125),
                d_utc: 19.0 - 0.125,
                is_lse: false,
                params: Some(make_p_neg_offset()),
            },
            TestCase {
                t: GpsTime::new_unchecked(2086, 259219.5 - 0.125),
                d_utc: 19.0 - 0.125,
                is_lse: false,
                params: Some(make_p_neg_offset()),
            },
            /* Jan 1 2020 (constant positive UTC offset) */
            TestCase {
                t: GpsTime::new_unchecked(2086, 259217.125),
                d_utc: 18.125,
                is_lse: false,
                params: Some(make_p_pos_offset()),
            },
            TestCase {
                t: GpsTime::new_unchecked(2086, 259217.5 + 0.125),
                d_utc: 18.125,
                is_lse: false,
                params: Some(make_p_pos_offset()),
            },
            TestCase {
                t: GpsTime::new_unchecked(2086, 259218.125),
                d_utc: 18.125,
                is_lse: true,
                params: Some(make_p_pos_offset()),
            },
            TestCase {
                t: GpsTime::new_unchecked(2086, 259218.5 + 0.125),
                d_utc: 18.125,
                is_lse: true,
                params: Some(make_p_pos_offset()),
            },
            TestCase {
                t: GpsTime::new_unchecked(2086, 259219.125),
                d_utc: 19.125,
                is_lse: false,
                params: Some(make_p_pos_offset()),
            },
            TestCase {
                t: GpsTime::new_unchecked(2086, 259219.5 + 0.125),
                d_utc: 19.125,
                is_lse: false,
                params: Some(make_p_pos_offset()),
            },
            /* Jan 1 2020 (positive UTC linear correction) */
            TestCase {
                t: GpsTime::new_unchecked(2086, 259217.0),
                d_utc: 18.0,
                is_lse: false,
                params: Some(make_p_pos_trend()),
            },
            TestCase {
                t: GpsTime::new_unchecked(2086, 259217.5),
                d_utc: 18.0,
                is_lse: false,
                params: Some(make_p_pos_trend()),
            },
            TestCase {
                t: GpsTime::new_unchecked(2086, 259218.0001),
                d_utc: 18.0,
                is_lse: true,
                params: Some(make_p_pos_trend()),
            },
            TestCase {
                t: GpsTime::new_unchecked(2086, 259218.5),
                d_utc: 18.0,
                is_lse: true,
                params: Some(make_p_pos_trend()),
            },
            TestCase {
                t: GpsTime::new_unchecked(2086, 259219.0001),
                d_utc: 19.0,
                is_lse: false,
                params: Some(make_p_pos_trend()),
            },
            TestCase {
                t: GpsTime::new_unchecked(2086, 259219.5),
                d_utc: 19.0,
                is_lse: false,
                params: Some(make_p_pos_trend()),
            },
            /* Jan 1 2020 (negative UTC linear correction) */
            TestCase {
                t: GpsTime::new_unchecked(2086, 259217.0),
                d_utc: 18.0,
                is_lse: false,
                params: Some(make_p_neg_trend()),
            },
            TestCase {
                t: GpsTime::new_unchecked(2086, 259217.5),
                d_utc: 18.0,
                is_lse: false,
                params: Some(make_p_neg_trend()),
            },
            TestCase {
                t: GpsTime::new_unchecked(2086, 259218.0),
                d_utc: 18.0,
                is_lse: true,
                params: Some(make_p_neg_trend()),
            },
            TestCase {
                t: GpsTime::new_unchecked(2086, 259218.5),
                d_utc: 18.0,
                is_lse: true,
                params: Some(make_p_neg_trend()),
            },
            TestCase {
                t: GpsTime::new_unchecked(2086, 259219.0),
                d_utc: 19.0,
                is_lse: false,
                params: Some(make_p_neg_trend()),
            },
            TestCase {
                t: GpsTime::new_unchecked(2086, 259219.5),
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
                "{} {} {}",
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
                t: GpsTime::new_unchecked(77, 259199.0),
                u: UtcExpectation::new(1981, 6, 30, 23, 59, 59.0),
                p: None,
            },
            TestCase {
                t: GpsTime::new_unchecked(77, 259199.5),
                u: UtcExpectation::new(1981, 6, 30, 23, 59, 59.5),
                p: None,
            },
            TestCase {
                t: GpsTime::new_unchecked(77, 259200.0),
                u: UtcExpectation::new(1981, 6, 30, 23, 59, 60.0),
                p: None,
            },
            TestCase {
                t: GpsTime::new_unchecked(77, 259200.5),
                u: UtcExpectation::new(1981, 6, 30, 23, 59, 60.5),
                p: None,
            },
            TestCase {
                t: GpsTime::new_unchecked(77, 259201.0),
                u: UtcExpectation::new(1981, 7, 01, 00, 00, 00.0),
                p: None,
            },
            /* Jan 1 2017 */
            TestCase {
                t: GpsTime::new_unchecked(1930, 16.0),
                u: UtcExpectation::new(2016, 12, 31, 23, 59, 59.0),
                p: None,
            },
            TestCase {
                t: GpsTime::new_unchecked(1930, 16.5),
                u: UtcExpectation::new(2016, 12, 31, 23, 59, 59.5),
                p: None,
            },
            TestCase {
                t: GpsTime::new_unchecked(1930, 17.0),
                u: UtcExpectation::new(2016, 12, 31, 23, 59, 60.0),
                p: None,
            },
            TestCase {
                t: GpsTime::new_unchecked(1930, 17.5),
                u: UtcExpectation::new(2016, 12, 31, 23, 59, 60.5),
                p: None,
            },
            TestCase {
                t: GpsTime::new_unchecked(1930, 18.0),
                u: UtcExpectation::new(2017, 01, 01, 00, 00, 00.0),
                p: None,
            },
            /* Jan 8 2017 */
            TestCase {
                t: GpsTime::new_unchecked(1931, 17.0),
                u: UtcExpectation::new(2017, 01, 7, 23, 59, 59.0),
                p: None,
            },
            TestCase {
                t: GpsTime::new_unchecked(1931, 17.5),
                u: UtcExpectation::new(2017, 01, 7, 23, 59, 59.5),
                p: None,
            },
            TestCase {
                t: GpsTime::new_unchecked(1931, 18.0 - 6e-11),
                u: UtcExpectation::new(2017, 01, 7, 23, 59, 59.0 + 1.0 - 6e-11),
                p: None,
            },
            TestCase {
                t: GpsTime::new_unchecked(1931, 18.0 - 5e-11),
                u: UtcExpectation::new(2017, 01, 8, 00, 00, 00.0),
                p: None,
            },
            TestCase {
                t: GpsTime::new_unchecked(1931, 18.0),
                u: UtcExpectation::new(2017, 01, 8, 00, 00, 00.0),
                p: None,
            },
            /* Jan 1 2020 (leap second announced in utc_params_t above, constant
            negative offset) */
            TestCase {
                t: GpsTime::new_unchecked(2086, 259217.0 - 0.125),
                u: UtcExpectation::new(2019, 12, 31, 23, 59, 59.0),
                p: Some(make_p_neg_offset()),
            },
            TestCase {
                t: GpsTime::new_unchecked(2086, 259217.5 - 0.125),
                u: UtcExpectation::new(2019, 12, 31, 23, 59, 59.5),
                p: Some(make_p_neg_offset()),
            },
            TestCase {
                t: GpsTime::new_unchecked(2086, 259218.0 - 0.125),
                u: UtcExpectation::new(2019, 12, 31, 23, 59, 60.0),
                p: Some(make_p_neg_offset()),
            },
            TestCase {
                t: GpsTime::new_unchecked(2086, 259218.5 - 0.125),
                u: UtcExpectation::new(2019, 12, 31, 23, 59, 60.5),
                p: Some(make_p_neg_offset()),
            },
            TestCase {
                t: GpsTime::new_unchecked(2086, 259219.0 - 0.125),
                u: UtcExpectation::new(2020, 01, 01, 00, 00, 00.0),
                p: Some(make_p_neg_offset()),
            },
            /* Jan 1 2020 (leap second announced in utc_params_t above, constant
            positive offset) */
            TestCase {
                t: GpsTime::new_unchecked(2086, 259217.125),
                u: UtcExpectation::new(2019, 12, 31, 23, 59, 59.0),
                p: Some(make_p_pos_offset()),
            },
            TestCase {
                t: GpsTime::new_unchecked(2086, 259217.5 + 0.125),
                u: UtcExpectation::new(2019, 12, 31, 23, 59, 59.5),
                p: Some(make_p_pos_offset()),
            },
            TestCase {
                t: GpsTime::new_unchecked(2086, 259218.125),
                u: UtcExpectation::new(2019, 12, 31, 23, 59, 60.0),
                p: Some(make_p_pos_offset()),
            },
            TestCase {
                t: GpsTime::new_unchecked(2086, 259218.5 + 0.125),
                u: UtcExpectation::new(2019, 12, 31, 23, 59, 60.5),
                p: Some(make_p_pos_offset()),
            },
            TestCase {
                t: GpsTime::new_unchecked(2086, 259219.125),
                u: UtcExpectation::new(2020, 01, 01, 00, 00, 00.0),
                p: Some(make_p_pos_offset()),
            },
            /* Jan 1 2020 (leap second announced in utc_params_t above, positive UTC
            linear correction) */
            TestCase {
                t: GpsTime::new_unchecked(2086, 259217.0),
                u: UtcExpectation::new(2019, 12, 31, 23, 59, 59.0),
                p: Some(make_p_pos_trend()),
            },
            TestCase {
                t: GpsTime::new_unchecked(2086, 259217.5),
                u: UtcExpectation::new(2019, 12, 31, 23, 59, 59.5),
                p: Some(make_p_pos_trend()),
            },
            TestCase {
                t: GpsTime::new_unchecked(2086, 259218.0),
                u: UtcExpectation::new(2019, 12, 31, 23, 59, 60.0),
                p: Some(make_p_pos_trend()),
            },
            TestCase {
                t: GpsTime::new_unchecked(2086, 259218.5),
                u: UtcExpectation::new(2019, 12, 31, 23, 59, 60.5),
                p: Some(make_p_pos_trend()),
            },
            TestCase {
                t: GpsTime::new_unchecked(2086, 259219.00001),
                u: UtcExpectation::new(2020, 01, 01, 00, 00, 00.0),
                p: Some(make_p_pos_trend()),
            },
            /* Jan 1 2020 (leap second announced in utc_params_t above, negative UTC
            linear correction) */
            TestCase {
                t: GpsTime::new_unchecked(2086, 259217.0),
                u: UtcExpectation::new(2019, 12, 31, 23, 59, 59.0),
                p: Some(make_p_neg_trend()),
            },
            TestCase {
                t: GpsTime::new_unchecked(2086, 259217.5),
                u: UtcExpectation::new(2019, 12, 31, 23, 59, 59.5),
                p: Some(make_p_neg_trend()),
            },
            TestCase {
                t: GpsTime::new_unchecked(2086, 259218.0),
                u: UtcExpectation::new(2019, 12, 31, 23, 59, 60.0),
                p: Some(make_p_neg_trend()),
            },
            TestCase {
                t: GpsTime::new_unchecked(2086, 259218.5),
                u: UtcExpectation::new(2019, 12, 31, 23, 59, 60.5),
                p: Some(make_p_neg_trend()),
            },
            TestCase {
                t: GpsTime::new_unchecked(2086, 259219.0),
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

            assert_eq!(u.year(), expected.year);
            assert_eq!(u.month(), expected.month);
            assert_eq!(u.day_of_month(), expected.day);
            assert_eq!(u.hour(), expected.hour);
            assert_eq!(u.minute(), expected.minute);
            assert!(
                (u.seconds() - expected.second).abs() < 1e-5,
                "{} {} {}",
                u.seconds(),
                expected.second,
                test_case.t.tow()
            );
        }
    }

    #[test]
    fn round_to_epoch() {
        let soln_freq = 10.0;
        let epsilon = std::time::Duration::from_secs_f64(1e-5);

        let test_cases = [
            GpsTime::new_unchecked(1234, 567890.01),
            GpsTime::new_unchecked(1234, 567890.0501),
            GpsTime::new_unchecked(1234, 604800.06),
        ];

        let expectations = [
            GpsTime::new_unchecked(1234, 567890.00),
            GpsTime::new_unchecked(1234, 567890.10),
            GpsTime::new_unchecked(1235, 0.1),
        ];

        for (test_case, expectation) in test_cases.iter().zip(expectations.iter()) {
            let rounded = test_case.round_to_epoch(soln_freq);

            let diff = if &rounded >= expectation {
                rounded - expectation
            } else {
                *expectation - rounded
            };
            assert!(diff < epsilon);
        }
    }

    #[test]
    fn floor_to_epoch() {
        let soln_freq = 10.0;
        let epsilon = std::time::Duration::from_secs_f64(1e-6);

        let test_cases = [
            GpsTime::new_unchecked(1234, 567890.01),
            GpsTime::new_unchecked(1234, 567890.0501),
            GpsTime::new_unchecked(1234, 604800.06),
        ];

        let expectations = [
            GpsTime::new_unchecked(1234, 567890.00),
            GpsTime::new_unchecked(1234, 567890.00),
            GpsTime::new_unchecked(1235, 0.0),
        ];

        for (test_case, expectation) in test_cases.iter().zip(expectations.iter()) {
            let rounded = test_case.floor_to_epoch(soln_freq);
            assert!((rounded - expectation) < epsilon);
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
        assert_eq!(gps.wn(), swiftnav_sys::GAL_WEEK_TO_GPS_WEEK as i16);
        assert!(gps.tow().abs() < 1e-9);

        assert!(GalTime::new(-1, 0.0).is_err());
        assert!(GalTime::new(0, -1.0).is_err());
        assert!(GalTime::new(0, swiftnav_sys::WEEK_SECS as f64 + 1.0).is_err());
    }

    #[test]
    fn gps_to_bds() {
        let bds = BDS_TIME_START.to_bds();
        assert_eq!(bds.wn(), 0);
        assert!(bds.tow().abs() < 1e-9);
        let gps = bds.to_gps();
        assert_eq!(gps.wn(), swiftnav_sys::BDS_WEEK_TO_GPS_WEEK as i16);
        assert!((gps.tow() - swiftnav_sys::BDS_SECOND_TO_GPS_SECOND as f64).abs() < 1e-9);

        assert!(BdsTime::new(-1, 0.0).is_err());
        assert!(BdsTime::new(0, -1.0).is_err());
        assert!(BdsTime::new(0, swiftnav_sys::WEEK_SECS as f64 + 1.0).is_err());
    }

    #[test]
    fn gps_to_glo() {
        let glo = GLO_TIME_START.to_glo_hardcoded();
        assert_eq!(glo.nt(), 1);
        assert_eq!(glo.n4(), 1);
        assert_eq!(glo.h(), 0);
        assert_eq!(glo.m(), 0);
        assert!(glo.s().abs() < 1e-9);
        let gps = glo.to_gps_hardcoded();
        assert_eq!(gps.wn(), swiftnav_sys::GLO_EPOCH_WN as i16);
        assert!((gps.tow() - swiftnav_sys::GLO_EPOCH_TOW as f64).abs() < 1e-9);
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
