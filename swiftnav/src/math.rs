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
pub(crate) const fn compile_time_sqrt(s: f64) -> f64 {
    let mut x = s;
    let mut y = 0.0;
    let mut z;
    let mut i = 0;
    while y != x {
        y = x;
        z = s / y;
        x = (y + z) / 2.0;
        i += 1;
    }
    if i > 100 {
        panic!("SLOW_SQRT failed to converge");
    }
    x
}
