// Copyright (c) 2025 Swift Navigation Inc.
// Contact: Swift Navigation <dev@swiftnav.com>
//
// This source is subject to the license found in the file 'LICENSE' which must
// be be distributed together with this source. All other rights reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND,
// EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A PARTICULAR PURPOSE.

/// We define a `const` max function since [`std::cmp::max`] isn't `const`
pub(crate) const fn compile_time_max_u16(a: u16, b: u16) -> u16 {
    if b < a {
        a
    } else {
        b
    }
}

/// Computes the square root of a given number at compile time using the Newton-Raphson method.
///
/// # Parameters
///
/// - `s`: A `f64` value representing the number for which the square root is to be calculated.
///
/// # Returns
///
/// - A `f64` value representing the square root of the input number.
///
/// # Panics
///
/// - This function will panic if the computation does not converge within 100 iterations.
///
/// # Notes
///
/// - This function is marked as `const`, allowing it to be evaluated at compile time.
/// - The algorithm iteratively refines the approximation of the square root until the result stabilizes.
#[allow(clippy::many_single_char_names)] // This is pure math, single character names are used in the source material
pub(crate) const fn compile_time_sqrt(s: f64) -> f64 {
    let mut x = s;
    let mut y = 0.0_f64;
    let mut z;
    let mut i = 0;
    while y.to_bits() != x.to_bits() {
        y = x;
        z = s / y;
        x = f64::midpoint(y, z);
        i += 1;
    }
    assert!(i <= 100, "SLOW_SQRT failed to converge");
    x
}

/// Calculate the rotation matrix for rotating between an [`crate::coords::ECEF`] and [`crate::coords::NED`] frames
#[must_use]
pub(crate) fn ecef2ned_matrix(llh: crate::coords::LLHRadians) -> nalgebra::Matrix3<f64> {
    let sin_lat = llh.latitude().sin();
    let cos_lat = llh.latitude().cos();
    let sin_lon = llh.longitude().sin();
    let cos_lon = llh.longitude().cos();

    nalgebra::Matrix3::new(
        -sin_lat * cos_lon,
        -sin_lat * sin_lon,
        cos_lat,
        -sin_lon,
        cos_lon,
        0.0,
        -cos_lat * cos_lon,
        -cos_lat * sin_lon,
        -sin_lat,
    )
}
