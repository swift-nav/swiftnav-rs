#[derive(Copy, Clone)]
pub struct GpsTime(crate::gps_time_t);

impl GpsTime {
    pub fn new_unchecked(wn: i16, tow: f64) -> GpsTime {
        GpsTime(crate::gps_time_t{wn, tow})
    }

    pub fn new(wn: i16, tow: f64) -> Option<GpsTime> {
        let time = GpsTime::new_unchecked(wn, tow);

        if time.is_valid() {
            Some(time)
        } else {
            None
        }
    }

    pub fn is_valid(&self) -> bool {
        unsafe { crate::gps_time_valid(&self.0) }
    }

    pub fn add_duration(&mut self, duration: &std::time::Duration) {
        unsafe { crate::add_secs(&mut self.0, duration.as_secs_f64()); }
    }

    pub fn subtract_duration(&mut self, duration: &std::time::Duration) {
        unsafe { crate::add_secs(&mut self.0, -duration.as_secs_f64()); }
    }
}

impl std::fmt::Debug for GpsTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GpsTime")
            .field("WN", unsafe{&self.0.wn})
            .field("TOW", unsafe{&self.0.tow})
            .finish()
    }
}

impl PartialEq for GpsTime {
    fn eq(&self, other: &Self) -> bool {
        let diff = unsafe { crate::gpsdifftime(&self.0, &other.0) };

        return diff.abs() < crate::FLOAT_EQUALITY_EPS;
    }
}

impl PartialOrd for GpsTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let diff = unsafe {crate::gpsdifftime(&self.0, &other.0) };

        if diff.abs() < crate::FLOAT_EQUALITY_EPS {
            Some(std::cmp::Ordering::Equal)
        } else if diff > 0.0 {
            Some(std::cmp::Ordering::Greater)
        } else {
            Some(std::cmp::Ordering::Less)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GpsTime;

    #[test]
    fn validity() {
        assert!(GpsTime::new(0, 0.0).is_some());
        assert!(GpsTime::new(-1, -1.0).is_none());
        assert!(GpsTime::new(-1, -1.0).is_none());
        assert!(GpsTime::new(12, crate::WEEK_SECS as f64).is_none());
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
        let mut t1 = GpsTime::new(0, 0.0).unwrap();
        let t2 = GpsTime::new(0, 1.001).unwrap();

        t1.add_duration(&std::time::Duration::new(1, 1000000));
        assert_eq!(t1, t2);
    }

    #[test]
    fn subtract_duration() {
        let mut t1 = GpsTime::new(0, 1.001).unwrap();
        let t2 = GpsTime::new(0, 0.0).unwrap();

        t1.subtract_duration(&std::time::Duration::new(1, 1000000));
        assert_eq!(t1, t2);

        t1.subtract_duration(&std::time::Duration::new(1, 1000000));
        assert!(!t1.is_valid());
    }
}
