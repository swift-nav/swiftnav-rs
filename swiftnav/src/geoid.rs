use crate::coords::LLHRadians;

/// List of potential Geoid models used
///
/// Currently only one model is compiled into the code at a time
pub enum GeoidModel {
    /// The EGM2008 geoid model, down sampled to 1 degree resolution
    Egm2008_1Deg,
    /// The EGM2008 geoid model, down sampled to 15 arc minute resolution
    Egm2008_15Min,
}

/// Get the offset of the geoid from the WGS84 ellipsoid
///
/// Only the latitude and longitude of the given position is taken into
/// account, the height is ignored.
///
/// To get the convert a height above the WGS84 ellipsoid to a height above the
/// geoid subtract the geoid offset from the height above the WGS84 ellipsoid
pub fn get_geoid_offset<T: Into<LLHRadians>>(pos: T) -> f32 {
    let pos: LLHRadians = pos.into();

    unsafe { swiftnav_sys::get_geoid_offset(pos.latitude(), pos.longitude()) }
}

/// Gets the geoid model compiled into the Swiftnav crate
pub fn get_geoid_model() -> GeoidModel {
    GeoidModel::Egm2008_1Deg
}
