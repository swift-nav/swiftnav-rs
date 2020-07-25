//! Troposphere delay calculation
//!
//! Tropospheric delays are typically modeled with the Klobuchar model. The model
//! parameters are broadcast by the GPS constellation. A function to decode the
//! parameters from the raw subframe is provided.
//!
/// -----------
/// References:
///   * UNB Neutral Atmosphere Models: Development and Performance. R Leandro,
///      M Santos, and R B Langley
use crate::{c_bindings, time::GpsTime};

///  Calculate tropospheric delay using UNM3m model.
///
/// Requires the time of the delay, the latitude (rad) and height (m) of the
/// receiver, and the elevation of the satellite (rad)
pub fn calc_delay(t: &GpsTime, lat: f64, h: f64, el: f64) -> f64 {
    unsafe { c_bindings::calc_troposphere(t.c_ptr(), lat, h, el) }
}

#[cfg(test)]
mod tests {
    use crate::{
        time::{GpsTime, DAY},
        troposphere::calc_delay,
    };

    const D2R: f64 = std::f64::consts::PI / 180.0;

    #[test]
    fn calc_troposphere() {
        const d_tol: f64 = 1e-4;

        /* some tests against "true" values computed with UNB3M.f */
        /* http://www2.unb.ca/gge/Personnel/Santos/UNB_pack.pdf */

        let lat = 40.0 * D2R;
        let h = 1300.0;
        let doy = 32.5;
        let el = 45.0 * D2R;
        let d_true = 2.8567;

        /* GPS week 1669 starts on 1.1.2012, so easier to generate given doy */
        let t = GpsTime::new(1669, 0.).unwrap();
        let t = t + DAY.mul_f64(doy);

        let d_tropo = calc_delay(&t, lat, h, el);

        assert!(
            (d_tropo - d_true).abs() < d_tol,
            "Distance didn't match hardcoded correct values {:.5}. Saw: {:.5}",
            d_true,
            d_tropo
        );

        let lat = -10. * D2R;
        let h = 0.0;
        let doy = 180.5;
        let el = 20. * D2R;
        let d_true = 7.4942;

        let t = GpsTime::new(1669, 0.).unwrap();
        let t = t + DAY.mul_f64(doy);

        let d_tropo = calc_delay(&t, lat, h, el);

        assert!(
            (d_tropo - d_true).abs() < d_tol,
            "Distance didn't match hardcoded correct values {:.5}. Saw: {:.5}",
            d_true,
            d_tropo
        );

        let lat = 75. * D2R;
        let h = 0.0;
        let doy = 50.5;
        let el = 10. * D2R;
        let d_true = 12.9004;

        let t = GpsTime::new(1669, 0.).unwrap();
        let t = t + DAY.mul_f64(doy);

        let d_tropo = calc_delay(&t, lat, h, el);

        assert!(
            (d_tropo - d_true).abs() < d_tol,
            "Distance didn't match hardcoded correct values {:.5}. Saw: {:.5}",
            d_true,
            d_tropo
        );

        /* altitude sanity tests */
        let max_tropo_correction = 30.0;
        let h = -5000.;
        let d_tropo = calc_delay(&t, lat, h, el);

        assert!(
            d_tropo.abs() < max_tropo_correction,
            "Sanity test fail at altitude {:.5}. : Correction was {:.5}",
            h,
            d_tropo
        );

        let h = 12000.;
        let d_tropo = calc_delay(&t, lat, h, el);

        assert!(
            d_tropo.abs() < max_tropo_correction,
            "Sanity test fail at altitude {:.5}. : Correction was {:.5}",
            h,
            d_tropo
        );

        /* satellite elevation sanity tests */
        let h = 100.;
        let elevation_testcases: [f64; 6] = [1e-3, 1e-4, 1e-5, 0., -1e3, -0.1];
        let max_tropo_correction = 100.0;

        for el in elevation_testcases.iter() {
            let d_tropo = calc_delay(&t, lat, h, *el);
            assert!(
                d_tropo.abs() < max_tropo_correction,
                "Sanity test fail at satellite elevation {:.5}. : Correction was {:.5}",
                el,
                d_tropo
            );
        }
    }
}
