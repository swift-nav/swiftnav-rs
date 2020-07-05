// void llhrad2deg(const double llh_rad[3], double llh_deg[3]);
// void llhdeg2rad(const double llh_deg[3], double llh_rad[3]);
// void wgsllh2ecef(const double llh[3], double ecef[3]);
// void wgsecef2llh(const double ecef[3], double llh[3]);
// void wgsecef2ned(const double ecef[3], const double ref_ecef[3], double ned[3]);
// void wgsecef2ned_d(const double ecef[3], const double ref_ecef[3], double ned[3]);
// void wgsned2ecef(const double ned[3], const double ref_ecef[3], double ecef[3]);
// void wgsned2ecef_d(const double ned[3], const double ref_ecef[3], double ecef[3]);
// void wgsecef2azel(const double ecef[3], const double ref_ecef[3], double* azimuth, double* elevation);

use crate::c_bindings;
use std::marker::PhantomData;

pub trait Angle { }

#[derive(Copy, Clone, Debug)]
pub struct Degrees { }
impl Angle for Degrees { }

#[derive(Copy, Clone, Debug)]
pub struct Radians { }
impl Angle for Radians { }

#[derive(Copy, Clone, Debug)]
pub struct LLH<T: Angle>([f64; 3], PhantomData<T>);

impl<T: Angle> LLH<T> {
    pub fn new(lat: f64, lon: f64, height: f64) -> LLH<T> {
        LLH([lat, lon, height], PhantomData)
    }

    pub fn from_array(array: &[f64; 3]) -> LLH<T> {
        LLH(*array, PhantomData)
    }

    pub fn as_ptr(&self) -> *const f64 {
        self.0.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut f64 {
        self.0.as_mut_ptr()
    }

    pub fn latitude(&self) -> f64 {
        self.0[0]
    }

    pub fn longitude(&self) -> f64 {
        self.0[1]
    }

    pub fn height(&self) -> f64 {
        self.0[2]
    }
}

impl LLH<Radians> {
    pub fn to_degrees(&self) -> LLH<Degrees> {
        let mut deg = LLH::<Degrees>::from_array(&[0.0; 3]);
        unsafe { c_bindings::llhrad2deg(self.as_ptr(), deg.as_mut_ptr()) };
        deg
    }

    pub fn to_ecef(&self) -> ECEF {
        let mut ecef = ECEF::from_array(&[0.0; 3]);
        unsafe { c_bindings::wgsllh2ecef(self.as_ptr(), ecef.as_mut_ptr()) };
        ecef
    }
}

impl LLH<Degrees> {
    pub fn to_radians(&self) -> LLH<Radians> {
        let mut rad = LLH::<Radians>::from_array(&[0.0; 3]);
        unsafe { c_bindings::llhdeg2rad(self.as_ptr(), rad.as_mut_ptr()) };
        rad
    }
}

impl<T: Angle> AsRef<[f64; 3]> for LLH<T> {
    fn as_ref(&self) -> &[f64; 3] {
        &self.0
    }
}

impl<T: Angle> AsMut<[f64; 3]> for LLH<T> {
    fn as_mut(&mut self) -> &mut [f64; 3] {
        &mut self.0
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ECEF([f64; 3]);

impl ECEF {
    pub fn new(x: f64, y: f64, z: f64) -> ECEF {
        ECEF([x, y, z])
    }

    pub fn from_array(array: &[f64; 3]) -> ECEF {
        ECEF(*array)
    }

    pub fn as_ptr(&self) -> *const f64 {
        self.0.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut f64 {
        self.0.as_mut_ptr()
    }

    pub fn x(&self) -> f64 {
        self.0[0]
    }

    pub fn y(&self) -> f64 {
        self.0[1]
    }

    pub fn z(&self) -> f64 {
        self.0[2]
    }

    pub fn to_llh(&self) -> LLH<Radians> {
        let mut llh = LLH::<Radians>::from_array(&[0.0; 3]);
        unsafe { c_bindings::wgsecef2llh(self.as_ptr(), llh.as_mut_ptr()) };
        llh
    }

    pub fn to_azel(&self, ref_point: &ECEF) -> AzimuthElevation {
        let mut azel = AzimuthElevation::new(0.0, 0.0);
        unsafe { c_bindings::wgsecef2azel(self.as_ptr(), ref_point.as_ptr(), &mut azel.az, &mut azel.el) };
        azel
    }
}


#[derive(Copy, Clone, Debug)]
pub struct AzimuthElevation {
    pub az: f64,
    pub el: f64,
}

impl AzimuthElevation {
    pub fn new(az: f64, el: f64) -> AzimuthElevation {
        AzimuthElevation{az, el}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const D2R: f64 = std::f64::consts::PI / 180.0;
    /* Maximum allowable error in quantities with units of length (in meters). */
    const MAX_DIST_ERROR_M: f64 = 1e-6;
    /* Maximum allowable error in quantities with units of angle (in sec of arc).
     * 1 second of arc on the equator is ~31 meters. */
    const MAX_ANGLE_ERROR_DEF: f64 = 1e-7;
    const MAX_ANGLE_ERROR_RAD: f64 = MAX_ANGLE_ERROR_DEF * D2R;

    #[test]
    fn llhrad2deg() {
        let zeros = LLH::<Radians>::from_array(&[0.0; 3]);

        let deg = zeros.to_degrees();
        assert_eq!(0.0, deg.latitude());
        assert_eq!(0.0, deg.longitude());
        assert_eq!(0.0, deg.height());

        let swift_home = LLH::<Degrees>::from_array(&[37.779804, -122.391751, 60.0]);
        let rads = swift_home.to_radians();

        assert!((rads.latitude() - 0.659381970558).abs() < MAX_ANGLE_ERROR_RAD);
        assert!((rads.longitude() + 2.136139032231).abs() < MAX_ANGLE_ERROR_RAD);
        assert!(rads.height() == swift_home.height());
    }

    const LLH_VALUES: [LLH<Radians>; 10] = [
        LLH::<Radians>([0.0, 0.0, 0.0], PhantomData),          /* On the Equator and Prime Meridian. */
        LLH::<Radians>([0.0, 180.0 * D2R, 0.0], PhantomData),  /* On the Equator. */
        LLH::<Radians>([0.0, 90.0 * D2R, 0.0], PhantomData),   /* On the Equator. */
        LLH::<Radians>([0.0, -90.0 * D2R, 0.0], PhantomData),  /* On the Equator. */
        LLH::<Radians>([90.0 * D2R, 0.0, 0.0], PhantomData),   /* North pole. */
        LLH::<Radians>([-90.0 * D2R, 0.0, 0.0], PhantomData),  /* South pole. */
        LLH::<Radians>([90.0 * D2R, 0.0, 22.0], PhantomData),  /* 22m above the north pole. */
        LLH::<Radians>([-90.0 * D2R, 0.0, 22.0], PhantomData), /* 22m above the south pole. */
        LLH::<Radians>([0.0, 0.0, 22.0], PhantomData),         /* 22m above the Equator and Prime Meridian. */
        LLH::<Radians>([0.0, 180.0 * D2R, 22.0], PhantomData), /* 22m above the Equator. */
    ];

    /* Semi-major axis. */
    const EARTH_A: f64 = 6378137.0;
    /* Semi-minor axis. */
    const EARTH_B: f64 = 6356752.31424517929553985595703125;

    const ECEF_VALUES: [ECEF; 10] = [
        ECEF([EARTH_A, 0.0, 0.0]),
        ECEF([-EARTH_A, 0.0, 0.0]),
        ECEF([0.0, EARTH_A, 0.0]),
        ECEF([0.0, -EARTH_A, 0.0]),
        ECEF([0.0, 0.0, EARTH_B]),
        ECEF([0.0, 0.0, -EARTH_B]),
        ECEF([0.0, 0.0, (EARTH_B + 22.0)]),
        ECEF([0.0, 0.0, -(EARTH_B + 22.0)]),
        ECEF([(22.0 + EARTH_A), 0.0, 0.0]),
        ECEF([-(22.0 + EARTH_A), 0.0, 0.0]),
    ];

    #[test]
    fn wgsllh2ecef() {
        for (llh_input, expected_ecef) in LLH_VALUES.iter().zip(ECEF_VALUES.iter()) {
            let ecef = llh_input.to_ecef();

            assert!(!ecef.x().is_nan());
            assert!(!ecef.y().is_nan());
            assert!(!ecef.z().is_nan());

            let x_err = ecef.x() - expected_ecef.x();
            assert!(x_err.abs() < MAX_DIST_ERROR_M);

            let y_err = ecef.y() - expected_ecef.y();
            assert!(y_err.abs() < MAX_DIST_ERROR_M);

            let z_err = ecef.z() - expected_ecef.z();
            assert!(z_err.abs() < MAX_DIST_ERROR_M);
        }
    }

    #[test]
    fn wgsecef2llh() {
        for (ecef_input, expected_llh) in ECEF_VALUES.iter().zip(LLH_VALUES.iter()) {
            let llh = ecef_input.to_llh();

            assert!(!llh.latitude().is_nan());
            assert!(!llh.longitude().is_nan());
            assert!(!llh.height().is_nan());

            let lat_err = llh.latitude() - expected_llh.latitude();
            assert!(lat_err.abs() < MAX_ANGLE_ERROR_RAD);

            let lon_err = llh.longitude() - expected_llh.longitude();
            assert!(lon_err.abs() < MAX_ANGLE_ERROR_RAD);

            let height_err = llh.height() - expected_llh.height();
            assert!(height_err.abs() < MAX_DIST_ERROR_M);
        }
    }
}