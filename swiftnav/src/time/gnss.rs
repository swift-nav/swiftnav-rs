use crate::time::{consts, UtcParams, UtcTime, MJD, UTC_LEAPS, WEEK};
use std::{
    ops::{Add, AddAssign, Sub, SubAssign},
    time::Duration,
};

/// Representation of GPS Time
#[derive(Debug, Copy, Clone)]
pub struct GpsTime {
    /// Seconds since the GPS start of week.
    tow: f64,
    /// GPS week number
    wn: i16,
}

/// GPS timestamp of the start of Galileo time
pub const GAL_TIME_START: GpsTime = GpsTime {
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
            Ok(GpsTime { wn, tow })
        }
    }

    pub(crate) const fn new_unchecked(wn: i16, tow: f64) -> GpsTime {
        GpsTime { wn, tow }
    }

    /// Makes a new GPS time object from a date and time, using
    pub fn from_date(
        year: u16,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        seconds: f64,
        utc_params: &UtcParams,
    ) -> GpsTime {
        MJD::from_date(year, month, day, hour, minute, seconds).to_gps(utc_params)
    }

    /// Makes a new GPS time object from a date and time using a hardcoded list of leap seconds
    ///
    /// # ⚠️  🦘  ⏱  ⚠️  - Leap Seconds
    ///
    /// The hard coded list of leap seconds will get out of date, it is
    /// preferable to use [`GpsTime::from_date()`] with the newest set of UTC parameters
    pub fn from_date_hardcoded(
        year: u16,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        seconds: f64,
    ) -> GpsTime {
        MJD::from_date(year, month, day, hour, minute, seconds).to_gps_hardcoded()
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
        self.tow.is_finite()
            && self.tow >= 0.0
            && self.tow < consts::WEEK_SECS as f64
            && self.wn >= 0
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
            || {
                (
                    self.is_leap_second_event_hardcoded(),
                    self.gps_utc_offset_hardcoded(),
                )
            },
            |p| (self.is_leap_second_event(p), self.gps_utc_offset(p)),
        );

        let mut tow_utc = self.tow - dt_utc;

        if is_lse {
            /* positive leap second event ongoing, so we are at 23:59:60.xxxx
             * subtract one second from time for now to make the conversion
             * into yyyy/mm/dd HH:MM:SS.sssss format, and add it back later */
            tow_utc -= 1.0;
        }

        let mut t_u = GpsTime {
            wn: self.wn,
            tow: tow_utc,
        };
        t_u.normalize();

        /* break the time into components */
        let mut u = UtcTime::from_gps(t_u);

        if is_lse {
            assert!(u.hour() == 23);
            assert!(u.minute() == 59);
            assert!(u.seconds_int() == 59);
            /* add the extra second back in*/
            u.add_second();
        }

        u
    }

    /// Converts the GPS time into UTC time
    ///
    /// # Panics
    ///
    /// This function will panic if the GPS time is not valid
    pub fn to_utc(self, utc_params: &UtcParams) -> UtcTime {
        self.to_utc_internal(Some(utc_params))
    }

    /// Converts the GPS time into UTC time using the hardcoded list of leap
    /// seconds.
    ///
    /// # ⚠️  🦘  ⏱  ⚠️  - Leap Seconds
    ///
    /// The hard coded list of leap seconds will get out of date, it is
    /// preferable to use [`GpsTime::to_utc()`] with the newest set of UTC parameters
    ///
    /// # Panics
    ///
    /// This function will panic if the GPS time is not valid
    pub fn to_utc_hardcoded(self) -> UtcTime {
        self.to_utc_internal(None)
    }

    /// Gets the number of seconds difference between GPS and UTC times
    pub fn gps_utc_offset(&self, utc_params: &UtcParams) -> f64 {
        let dt = self.diff(&utc_params.tot());

        /* The polynomial UTC to GPS correction */
        let mut dt_utc: f64 =
            utc_params.a0() + (utc_params.a1() * dt) + (utc_params.a2() * dt * dt);

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
    ///
    /// The hard coded list of leap seconds will get out of date, it is
    /// preferable to use [`GpsTime::gps_utc_offset()`] with the newest set
    /// of UTC parameters
    pub fn gps_utc_offset_hardcoded(&self) -> f64 {
        for (t_leap, offset) in UTC_LEAPS.iter().rev() {
            if self.diff(t_leap) >= 1.0 {
                return *offset;
            }
        }

        /* time is before the first known leap second event */
        0.0
    }

    /// Gets the number of seconds difference between UTC and GPS using the hardcoded
    /// list of leap seconds
    pub fn utc_gps_offset(&self, utc_params: &UtcParams) -> f64 {
        let dt = self.diff(&utc_params.tot()) + utc_params.dt_ls() as f64;

        /* The polynomial UTC to GPS correction */
        let mut dt_utc = utc_params.a0() + utc_params.a1() * dt + utc_params.a2() * dt * dt;

        /* the new UTC offset takes effect after the leap second event */
        if self.diff(&utc_params.t_lse()) >= ((-utc_params.dt_ls() as f64) - dt_utc) {
            dt_utc += utc_params.dt_lsf() as f64;
        } else {
            dt_utc += utc_params.dt_ls() as f64;
        }

        -dt_utc
    }

    /// Gets the number of seconds difference between UTC and GPS using the hardcoded
    /// list of leap seconds
    ///
    /// # ⚠️  🦘  ⏱  ⚠️  - Leap Seconds
    /// The hard coded list of leap seconds will get out of date, it is
    /// preferable to use [`GpsTime::utc_gps_offset()`] with the newest set
    /// of UTC parameters
    pub fn utc_gps_offset_hardcoded(&self) -> f64 {
        for (t_leap, offset) in UTC_LEAPS.iter().rev() {
            if self.diff(t_leap) >= (-offset + 1.0) {
                return -offset;
            }
        }

        /* time is before the first known leap second event */
        0.0
    }

    /// Checks to see if this point in time is a UTC leap second event
    pub fn is_leap_second_event(&self, params: &UtcParams) -> bool {
        /* the UTC offset takes effect exactly 1 second after the start of
         * the (positive) leap second event */
        let dt = self.diff(&params.t_lse());

        /* True only when self is during the leap second event */
        (0.0..1.0).contains(&dt)
    }

    /// Checks to see if this point in time is a UTC leap second event using the
    /// hardcoded list of leap seconds
    ///
    /// # ⚠️  🦘  ⏱  ⚠️  - Leap Seconds
    ///
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
            if (0.0..1.0).contains(&dt) {
                /* time is during the leap second event */
                return true;
            }
        }

        /* time is before the first known leap second event */
        false
    }

    /// Converts the GPS time into a [`MJD`] (modified julian date)
    pub fn to_mjd(self, utc_params: &UtcParams) -> MJD {
        self.to_utc(utc_params).to_mjd()
    }

    /// Converts the GPS time into a [`MJD`] (modified julian date) using a hard
    /// coded list of leap seconds
    ///
    /// # ⚠️  🦘  ⏱  ⚠️  - Leap Seconds
    ///
    /// The hard coded list of leap seconds will get out of date, it is
    /// preferable to use [`GpsTime::to_mjd()`] with the newest
    /// set of UTC parameters
    pub fn to_mjd_hardcoded(self) -> MJD {
        self.to_utc_hardcoded().to_mjd()
    }

    /// Converts the GPS time into Galileo time
    ///
    /// # Panics
    ///
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
    ///
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

    /// Converts the GPS time into a fractional year
    ///
    /// # Notes
    ///
    /// A fractional year is a decimal representation of the date. For example
    /// January 1, 2025 has a fractional year value of $2025.0$, while January
    /// 30, 2025 is 30 days into the year so has a fractional year value of
    /// approximately $2025.082$ ($30 \div 365 \approx 0.082$).
    pub fn to_fractional_year(&self, utc_params: &UtcParams) -> f64 {
        let utc = self.to_utc(utc_params);
        utc.to_fractional_year()
    }

    /// Converts the GPS time into a fractional year
    ///
    /// # Notes
    ///
    /// A fractional year is a decimal representation of the date. For example
    /// January 1, 2025 has a fractional year value of $2025.0$, while January
    /// 30, 2025 is 30 days into the year so has a fractional year value of
    /// approximately $2025.082$ ($30 \div 365 \approx 0.082$).
    ///
    /// # ⚠️  🦘  ⏱  ⚠️  - Leap Seconds
    ///
    /// The hard coded list of leap seconds will get out of date, it is
    /// preferable to use [`GpsTime::to_fractional_year()`] with the newest
    /// set of UTC parameters
    pub fn to_fractional_year_hardcoded(&self) -> f64 {
        let utc = self.to_utc_hardcoded();
        utc.to_fractional_year()
    }

    /// Converts the GPS time into a date and time
    pub fn to_date(self, utc_params: &UtcParams) -> (u16, u8, u8, u8, u8, f64) {
        self.to_utc(utc_params).to_date()
    }

    /// Converts the GPS time into a date and time
    ///
    /// # ⚠️  🦘  ⏱  ⚠️  - Leap Seconds
    ///
    /// The hard coded list of leap seconds will get out of date, it is
    /// preferable to use [`GpsTime::to_date()`] with the newest
    /// set of UTC parameters
    pub fn to_date_hardcoded(self) -> (u16, u8, u8, u8, u8, f64) {
        self.to_utc_hardcoded().to_date()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validity() {
        assert!(GpsTime::new(0, 0.0).is_ok());
        assert!(GpsTime::new(-1, -1.0).is_err());
        assert!(GpsTime::new(-1, -1.0).is_err());
        assert!(GpsTime::new(12, WEEK.as_secs_f64()).is_err());
        assert!(GpsTime::new(12, f64::NAN).is_err());
        assert!(GpsTime::new(12, f64::INFINITY).is_err());
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
    fn gps_to_gal() {
        let gal = GAL_TIME_START.to_gal();
        assert_eq!(gal.wn(), 0);
        assert!(gal.tow().abs() < 1e-9);
        let gps = gal.to_gps();
        assert_eq!(gps.wn(), consts::GAL_WEEK_TO_GPS_WEEK);
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
        assert_eq!(gps.wn(), consts::BDS_WEEK_TO_GPS_WEEK);
        assert!((gps.tow() - consts::BDS_SECOND_TO_GPS_SECOND).abs() < 1e-9);

        assert!(BdsTime::new(-1, 0.0).is_err());
        assert!(BdsTime::new(0, -1.0).is_err());
        assert!(BdsTime::new(0, consts::WEEK_SECS as f64 + 1.0).is_err());
    }
}
