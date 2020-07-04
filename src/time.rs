use crate::c_bindings::gnss_time as gnss_time_c;
use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::time::Duration;

pub const MINUTE: Duration = Duration::from_secs(gnss_time_c::MINUTE_SECS as u64);
pub const HOUR: Duration = Duration::from_secs(gnss_time_c::HOUR_SECS as u64);
pub const DAY: Duration = Duration::from_secs(gnss_time_c::DAY_SECS as u64);
pub const WEEK: Duration = Duration::from_secs(gnss_time_c::WEEK_SECS as u64);

#[derive(Copy, Clone)]
pub struct GpsTime(gnss_time_c::gps_time_t);

impl GpsTime {
    const JIFFY: f64 = gnss_time_c::FLOAT_EQUALITY_EPS;

    pub fn new_unchecked(wn: i16, tow: f64) -> GpsTime {
        GpsTime(gnss_time_c::gps_time_t { wn, tow })
    }

    pub fn new(wn: i16, tow: f64) -> Option<GpsTime> {
        let time = GpsTime::new_unchecked(wn, tow);

        if time.is_valid() {
            Some(time)
        } else {
            None
        }
    }

    pub fn wn(&self) -> i16 {
        self.0.wn
    }

    pub fn tow(&self) -> f64 {
        self.0.tow
    }

    pub fn is_valid(&self) -> bool {
        unsafe { gnss_time_c::gps_time_valid(&self.0) }
    }

    pub fn add_duration(&mut self, duration: &Duration) {
        unsafe {
            gnss_time_c::add_secs(&mut self.0, duration.as_secs_f64());
        }
    }

    pub fn subtract_duration(&mut self, duration: &Duration) {
        unsafe {
            gnss_time_c::add_secs(&mut self.0, -duration.as_secs_f64());
        }
    }

    pub fn diff(&self, other: &Self) -> f64 {
        unsafe { gnss_time_c::gpsdifftime(&self.0, &other.0) }
    }
}

impl fmt::Debug for GpsTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GpsTime")
            .field("WN", unsafe { &self.0.wn })
            .field("TOW", unsafe { &self.0.tow })
            .finish()
    }
}

impl PartialEq for GpsTime {
    fn eq(&self, other: &Self) -> bool {
        let diff_seconds = self.diff(other).abs();
        return diff_seconds < Self::JIFFY;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validity() {
        assert!(GpsTime::new(0, 0.0).is_some());
        assert!(GpsTime::new(-1, -1.0).is_none());
        assert!(GpsTime::new(-1, -1.0).is_none());
        assert!(GpsTime::new(12, WEEK.as_secs_f64()).is_none());
        assert!(GpsTime::new(12, std::f64::NAN).is_none());
        assert!(GpsTime::new(12, std::f64::INFINITY).is_none());
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
}
