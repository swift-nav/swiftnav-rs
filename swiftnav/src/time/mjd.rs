use std::time::Duration;
use crate::time::{consts, GpsTime, UtcTime, UtcParams};

/// Representation of modified julian dates (MJD)
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct MJD(f64);

impl MJD {
    /// Creates a modified julian date from a floating point representation
    ///
    /// # Panics
    ///
    /// Will panic if the given value is not finite
    pub fn from_f64(value: f64) -> Self {
        assert!(value.is_finite());
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
    
    pub(super) fn to_gps_internal(self, params: Option<&UtcParams>) -> GpsTime {
        let utc_days: f64 = self.0 - (consts::MJD_JAN_6_1980 as f64);
        
        let wn = (utc_days / consts::WEEK_DAYS as f64) as i16;
        let tow = (utc_days - wn as f64 * consts::WEEK_DAYS as f64) * (consts::DAY_SECS as f64);
        let utc_time = GpsTime::new_unchecked(wn, tow);

        let leap_secs = params.map_or_else(|| utc_time.utc_gps_offset_hardcoded(), |p| utc_time.utc_gps_offset(p));
        
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
    pub fn to_gps(self, utc_params: &UtcParams) -> GpsTime {
        self.to_gps_internal(Some(utc_params))
    }

    /// Converts the MJD into a [`GpsTime`] using a hard coded list of leap
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
    pub fn to_gps_hardcoded(self) -> GpsTime {
        self.to_gps_internal(None)
    }

    /// Converts the modified julian date into a UTC time
    pub fn to_utc(self) -> UtcTime {
        // utc_tm ret;
        let utc_days: f64 = self.0 - consts::MJD_JAN_6_1980 as f64;
        
        let wn = (utc_days / consts::WEEK_DAYS as f64) as i16;
        let tow = (utc_days - (wn as u32 * consts::WEEK_DAYS) as f64) * (consts::DAY_SECS as f64);
        let utc_time = GpsTime::new_unchecked(wn, tow);
        UtcTime::from_gps(utc_time)
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
    pub fn to_date(self) -> (u16, u8, u8, u8, u8, f64) {
        let j = (self.0 as i32) + 2400001 + 68569;
        let c = 4 * j / 146097;
        let j = j - (146097 * c + 3) / 4;
        let y = 4000 * (j + 1) / 1461001;
        let j = j - 1461 * y / 4 + 31;
        let m = 80 * j / 2447;
        let day: u8 = (j - 2447 * m / 80) as u8;
        let j = m / 11;
        let month: u8 = (m + 2 - (12 * j)) as u8;
        let year: u16 = (100 * (c - 49) + y + j) as u16;
        let frac_part = self.0.fract();
        let hour: u8 = (frac_part * consts::DAY_HOURS as f64) as u8;
        let min: u8 = ((frac_part - (hour as f64) / (consts::DAY_HOURS as f64)) * (consts::DAY_HOURS as f64) *
                    (consts::HOUR_MINUTES as f64)) as u8;
        let sec: f64 = (frac_part - (hour as f64) / (consts::DAY_HOURS as f64) -
                (min as f64) / (consts::DAY_HOURS as f64) / (consts::HOUR_MINUTES as f64)) *
                (consts::DAY_SECS as f64);
        (year, month, day, hour, min, sec)
    }
}

impl From<UtcTime> for MJD {
    fn from(utc: UtcTime) -> MJD {
        utc.to_mjd()
    }
}
