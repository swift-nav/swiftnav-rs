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
