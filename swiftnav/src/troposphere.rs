// Copyright (c) 2020-2021 Swift Navigation Inc.
// Contact: Swift Navigation <dev@swiftnav.com>
//
// This source is subject to the license found in the file 'LICENSE' which must
// be be distributed together with this source. All other rights reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND,
// EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A PARTICULAR PURPOSE.
//! Troposphere delay calculation
//!
//! Tropospheric delays are typically modeled with the UNM3m model. The model
//! parameters are hardcoded into the library, unlike the ionosphere model.
//!
//! # References
//! * UNB Neutral Atmosphere Models: Development and Performance. R Leandro,
//!   M Santos, and R B Langley

///  Calculate tropospheric delay using UNM3m model.
///
/// Requires the time of the delay, the latitude (rad) and height (m) of the
/// receiver, and the elevation of the satellite (rad)
pub fn calc_delay(doy: f64, lat: f64, h: f64, el: f64) -> f64 {
    unsafe { swiftnav_sys::calc_troposphere(doy, lat, h, el) }
}

#[cfg(test)]
mod tests {
    use crate::troposphere::calc_delay;

    const D2R: f64 = std::f64::consts::PI / 180.0;

    #[test]
    fn calc_troposphere() {
        const D_TOL: f64 = 1e-4;

        /* some tests against "true" values computed with UNB3M.f */
        /* http://www2.unb.ca/gge/Personnel/Santos/UNB_pack.pdf */

        let lat = 40.0 * D2R;
        let h = 1300.0;
        let doy = 32.5;
        let el = 45.0 * D2R;
        let d_true = 2.8567;

        let d_tropo = calc_delay(doy, lat, h, el);

        assert!(
            (d_tropo - d_true).abs() < D_TOL,
            "Distance didn't match hardcoded correct values {:.5}. Saw: {:.5}",
            d_true,
            d_tropo
        );

        let lat = -10. * D2R;
        let h = 0.0;
        let doy = 180.5;
        let el = 20. * D2R;
        let d_true = 7.4942;

        let d_tropo = calc_delay(doy, lat, h, el);

        assert!(
            (d_tropo - d_true).abs() < D_TOL,
            "Distance didn't match hardcoded correct values {:.5}. Saw: {:.5}",
            d_true,
            d_tropo
        );

        let lat = 75. * D2R;
        let h = 0.0;
        let doy = 50.5;
        let el = 10. * D2R;
        let d_true = 12.9007;

        let d_tropo = calc_delay(doy, lat, h, el);

        assert!(
            (d_tropo - d_true).abs() < D_TOL,
            "Distance didn't match hardcoded correct values {:.5}. Saw: {:.5}",
            d_true,
            d_tropo
        );

        /* altitude sanity tests */
        let max_tropo_correction = 30.0;
        let h = -5000.;
        let d_tropo = calc_delay(doy, lat, h, el);

        assert!(
            d_tropo.abs() < max_tropo_correction,
            "Sanity test fail at altitude {:.5}. : Correction was {:.5}",
            h,
            d_tropo
        );

        let h = 12000.;
        let d_tropo = calc_delay(doy, lat, h, el);

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
            let d_tropo = calc_delay(doy, lat, h, *el);
            assert!(
                d_tropo.abs() < max_tropo_correction,
                "Sanity test fail at satellite elevation {:.5}. : Correction was {:.5}",
                el,
                d_tropo
            );
        }
    }
}
