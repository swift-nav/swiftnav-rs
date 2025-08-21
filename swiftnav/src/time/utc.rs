use crate::time::{consts, is_leap_year, GpsTime, MJD};
use std::time::Duration;

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
#[derive(Debug, Clone, Copy)]
pub struct UtcTime {
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

    pub(super) fn from_gps(gps: GpsTime) -> UtcTime {
        /* see http://www.ngs.noaa.gov/gps-toolbox/bwr-c.txt */

        /* seconds of the day */
        let t_utc = gps.tow() % (consts::DAY_SECS as f64);

        /* Convert this into hours, minutes and seconds */
        let second_int = t_utc.floor() as u32; /* The integer part of the seconds */
        let second_frac: f64 = t_utc % 1.0; /* The fractional part of the seconds */
        let hour: u8 = (second_int / consts::HOUR_SECS) as u8; /* The hours (1 - 23) */
        let second_int = second_int - ((hour as u32) * consts::HOUR_SECS); /* Remove the hours from seconds */
        let minute: u8 = (second_int / consts::MINUTE_SECS) as u8; /* The minutes (1 - 59) */
        let second_int: u8 = (second_int - minute as u32 * consts::MINUTE_SECS) as u8; /* Remove the minutes from seconds */
 /* The seconds (1 - 60) */

        /* Calculate the years */

        /* Days from 1 Jan 1980. GPS epoch is 6 Jan 1980 */
        let modified_julian_days: i32 = consts::MJD_JAN_6_1980
            + gps.wn() as i32 * 7
            + (gps.tow() / consts::DAY_SECS as f64).floor() as i32;
        let days_since_1601: u32 = (modified_julian_days - consts::MJD_JAN_1_1601) as u32;

        /* Calculate the number of leap years */
        let num_400_years: u32 = days_since_1601 / consts::FOUR_HUNDRED_YEARS_DAYS;
        let days_left: u32 = days_since_1601 - num_400_years * consts::FOUR_HUNDRED_YEARS_DAYS;
        let num_100_years: u32 = days_left / consts::HUNDRED_YEARS_DAYS
            - days_left / (consts::FOUR_HUNDRED_YEARS_DAYS - 1);
        let days_left: u32 = days_left - num_100_years * consts::HUNDRED_YEARS_DAYS;
        let num_4_years: u32 = days_left / consts::FOUR_YEARS_DAYS;
        let days_left: u32 = days_left - num_4_years * consts::FOUR_YEARS_DAYS;
        let num_non_leap_years =
            days_left / consts::YEAR_DAYS - days_left / (consts::FOUR_YEARS_DAYS - 1);

        /* Calculate the total number of years from 1980 */
        let year = (1601
            + num_400_years * 400
            + num_100_years * 100
            + num_4_years * 4
            + num_non_leap_years) as u16;

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
            [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335, 366],
        ];

        /* Check if our guess was out, and what the correction is, */
        /* 0 = correct, 1 = wrong */
        let month_correction: u8 =
            if year_day > DAYS_AFTER_MONTH[leap_year][(month_guess + 1) as usize] {
                1
            } else {
                0
            };

        /* Calculate the corrected number of months */
        let month = month_guess + month_correction + 1;

        /* Calculate the day of the month */
        let month_day = (year_day
            - DAYS_AFTER_MONTH[leap_year][(month_guess + month_correction) as usize])
            as u8;

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

    /// Integer second of the minue (0 - 60)
    pub fn seconds_int(&self) -> u8 {
        self.second_int
    }

    /// Converts the UTC time into a modified julian date
    pub fn to_mjd(&self) -> MJD {
        MJD::from_date(
            self.year(),
            self.month(),
            self.day_of_month(),
            self.hour(),
            self.minute(),
            self.seconds(),
        )
    }

    /// Converts the UTC time into a date and time
    pub fn to_date(self) -> (u16, u8, u8, u8, u8, f64) {
        (
            self.year(),
            self.month(),
            self.day_of_month(),
            self.hour(),
            self.minute(),
            self.seconds(),
        )
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

    fn to_gps_internal(self, utc_params: Option<&UtcParams>) -> GpsTime {
        let is_lse = self.second_int >= 60;
        let mjd = self.to_mjd();

        let mut gps = utc_params.map_or_else(|| mjd.to_gps_hardcoded(), |p| mjd.to_gps(p));

        // During a leap second event the MJD is wrong by a second, so remove the
        // erroneous second here
        if is_lse {
            gps -= Duration::from_secs(1);
        }

        assert!(gps.is_valid());
        gps
    }

    /// Converts the UTC time into GPS time
    ///
    /// # Panics
    ///
    /// This function will panic if the [`UtcTime`] does not represent a valid
    /// GPS time.
    pub fn to_gps(self, utc_params: &UtcParams) -> GpsTime {
        self.to_gps_internal(Some(utc_params))
    }

    /// Converts the UTC time into GPS time using the hardcoded list of leap
    /// seconds.
    ///
    /// # Panics
    ///
    /// This function will panic if the [`UtcTime`] does not represent a valid
    /// GPS time.
    ///
    /// # ⚠️  🦘  ⏱  ⚠️  - Leap Seconds
    /// The hard coded list of leap seconds will get out of date, it is
    /// preferable to use [`UtcTime::to_gps()`] with the newest set of UTC parameters
    pub fn to_gps_hardcoded(self) -> GpsTime {
        self.to_gps_internal(None)
    }

    /// Converts the UTC time into a fractional year
    ///
    /// # Notes
    ///
    /// A fractional year is a decimal representation of the date. For example
    /// January 1, 2025 has a fractional year value of $2025.0$, while January
    /// 30, 2025 is 30 days into the year so has a fractional year value of
    /// approximately $2025.082$ ($30 \div 365 \approx 0.082$).
    pub fn to_fractional_year(&self) -> f64 {
        let year = self.year() as f64;
        let days = self.day_of_year() as f64;
        let hours = self.hour() as f64;
        let minutes = self.minute() as f64;
        let seconds = self.seconds();
        let total_days = days
            + hours / consts::DAY_HOURS as f64
            + (minutes / consts::MINUTE_SECS as f64 + seconds) / consts::DAY_SECS as f64;

        if is_leap_year(self.year()) {
            year + total_days / consts::LEAP_YEAR_DAYS as f64
        } else {
            year + total_days / consts::YEAR_DAYS as f64
        }
    }

    pub(super) fn add_second(&mut self) {
        self.second_int += 1;
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

/**
 * Start times of UTC leap second events given in GPS time {wn, tow, gps-utc}
 * The leap second event lasts for one second from the start time, and after
 * that the new offset is in effect.
 */
pub(super) const UTC_LEAPS: [(GpsTime, f64); 18] = [
    (GpsTime::new_unchecked(77, 259200.), 1.),  /* 01-07-1981 */
    (GpsTime::new_unchecked(129, 345601.), 2.), /* 01-07-1982 */
    (GpsTime::new_unchecked(181, 432002.), 3.), /* 01-07-1983 */
    (GpsTime::new_unchecked(286, 86403.), 4.),  /* 01-07-1985 */
    (GpsTime::new_unchecked(416, 432004.), 5.), /* 01-01-1988 */
    (GpsTime::new_unchecked(521, 86405.), 6.),  /* 01-01-1990 */
    (GpsTime::new_unchecked(573, 172806.), 7.), /* 01-01-1991 */
    (GpsTime::new_unchecked(651, 259207.), 8.), /* 01-07-1992 */
    (GpsTime::new_unchecked(703, 345608.), 9.), /* 01-07-1993 */
    (GpsTime::new_unchecked(755, 432009.), 10.), /* 01-07-1994 */
    (GpsTime::new_unchecked(834, 86410.), 11.), /* 01-01-1996 */
    (GpsTime::new_unchecked(912, 172811.), 12.), /* 01-07-1997 */
    (GpsTime::new_unchecked(990, 432012.), 13.), /* 01-01-1999 */
    (GpsTime::new_unchecked(1356, 13.), 14.),   /* 01-01-2006 */
    (GpsTime::new_unchecked(1512, 345614.), 15.), /* 01-01-2009 */
    (GpsTime::new_unchecked(1695, 15.), 16.),   /* 01-07-2012 */
    (GpsTime::new_unchecked(1851, 259216.), 17.), /* 01-07-2015 */
    (GpsTime::new_unchecked(1930, 17.), 18.),   /* 01-01-2017 */
];

#[cfg(test)]
mod tests {
    use super::*;

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
            let d_utc = test_case.t.gps_utc_offset_hardcoded();
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
                259218.0 + 1e-12 * (6.0 * consts::WEEK_SECS as f64 + 259218.0),
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
                259218.0 - 1e-12 * (6.0 * consts::WEEK_SECS as f64 + 259218.0),
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
                test_case.t.gps_utc_offset(params)
            } else {
                test_case.t.gps_utc_offset_hardcoded()
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
                u: UtcExpectation::new(1981, 7, 1, 00, 00, 00.0),
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
                u: UtcExpectation::new(2017, 1, 1, 00, 00, 00.0),
                p: None,
            },
            /* Jan 8 2017 */
            TestCase {
                t: GpsTime::new_unchecked(1931, 17.0),
                u: UtcExpectation::new(2017, 1, 7, 23, 59, 59.0),
                p: None,
            },
            TestCase {
                t: GpsTime::new_unchecked(1931, 17.5),
                u: UtcExpectation::new(2017, 1, 7, 23, 59, 59.5),
                p: None,
            },
            TestCase {
                t: GpsTime::new_unchecked(1931, 18.0 - 6e-11),
                u: UtcExpectation::new(2017, 1, 7, 23, 59, 59.0 + 1.0 - 6e-11),
                p: None,
            },
            TestCase {
                t: GpsTime::new_unchecked(1931, 18.0 - 5e-11),
                u: UtcExpectation::new(2017, 1, 8, 00, 00, 00.0),
                p: None,
            },
            TestCase {
                t: GpsTime::new_unchecked(1931, 18.0),
                u: UtcExpectation::new(2017, 1, 8, 00, 00, 00.0),
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
                u: UtcExpectation::new(2020, 1, 1, 00, 00, 00.0),
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
                u: UtcExpectation::new(2020, 1, 1, 00, 00, 00.0),
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
                u: UtcExpectation::new(2020, 1, 1, 00, 00, 00.0),
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
                u: UtcExpectation::new(2020, 1, 1, 00, 00, 00.0),
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

            assert_eq!(
                u.year(),
                expected.year,
                "u.year: {}, expected.year: {}, tow: {}",
                u.year(),
                expected.year,
                test_case.t.tow()
            );
            assert_eq!(
                u.month(),
                expected.month,
                "u.month: {}, expected.month: {}, tow: {}",
                u.month(),
                expected.month,
                test_case.t.tow()
            );
            assert_eq!(
                u.day_of_month(),
                expected.day,
                "u.day_of_month: {}, expected.day: {}, tow: {}",
                u.day_of_month(),
                expected.day,
                test_case.t.tow()
            );
            assert_eq!(
                u.hour(),
                expected.hour,
                "u.hour: {}, expected.hour: {}, tow: {}",
                u.hour(),
                expected.hour,
                test_case.t.tow()
            );
            assert_eq!(
                u.minute(),
                expected.minute,
                "u.minute: {}, expected.minute: {}, tow: {}",
                u.minute(),
                expected.minute,
                test_case.t.tow()
            );
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
}
