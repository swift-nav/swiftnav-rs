use crate::{c_bindings, time::GpsTime};

///  Calculate tropospheric delay using UNM3m model.
///
/// References:
///   -# UNB Neutral Atmosphere Models: Development and Performance. R Leandro,
///      M Santos, and R B Langley
///
/// \param t_gps GPS time at which to calculate tropospheric delay [gps_time]
/// \param lat Latitude of the receiver [rad]
/// \param h Orthometric height of the receiver (height above the geoid) [m]
/// \param el Elevation of the satellite [rad]
///
/// \return Tropospheric delay distance [m]
pub fn calc_delay(t: &GpsTime, lat: f64, h: f64, el: f64) -> f64 {
    unsafe { c_bindings::calc_troposphere(t.c_ptr(), lat, h, el) }
}
