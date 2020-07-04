pub struct GpsTime(crate::gps_time_t);

impl GpsTime {
    pub fn new(wn: i16, tow: f64) -> GpsTime {
        GpsTime(crate::gps_time_t{wn, tow})
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
    fn equality() {
        let t1 = GpsTime::new(10, 234.567);
        assert!(t1 == t1);

        let t2 = GpsTime::new(10, 234.5678);
        assert!(t1 != t2);
        assert!(t2 != t1);
    }

    #[test]
    fn ordering() {
        let t1 = GpsTime::new(10, 234.566);
        let t2 = GpsTime::new(10, 234.567);
        let t3 = GpsTime::new(10, 234.568);

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
}
