use crate::math::compile_time_sqrt;

/// Ellipsoid
///
/// An ellipsoid can be defined in terms of the semi-major axis and a second
/// parameter, here we choose to use the inverse flattening term. The other
/// parameters are derived from these two values.
pub trait Ellipsoid {
    /// Semi-major axis of the Earth in meters.
    const A: f64;
    /// Inverse flattening of the Earth.
    const IF: f64;

    /// The flattening of the Earth.
    const F: f64 = 1.0 / Self::IF;
    /// Semi-minor axis of the Earth in meters.
    const B: f64 = Self::A * (1.0 - Self::F);
    /// Eccentricity of the Earth,  where e^2 = 2f - f^2
    const E: f64 = compile_time_sqrt(2.0 * Self::F - Self::F * Self::F);
}

/// WGS84 Parameters
///
/// Parameters defining the WGS84 ellipsoid. See <https://earth-info.nga.mil/?dir=wgs84&action=wgs84>
pub struct WGS84;

impl Ellipsoid for WGS84 {
    const A: f64 = 6_378_137.0;
    const IF: f64 = 298.257_223_563;
}

/// GRS80 Parameters
///
/// Parameters defining the GRS80 ellipsoid. The ellipsoid is defined in terms
/// of the semi-major axis and 3 physical constants making the inverse flattening
/// a derived value. Here we use the calulated value of the inverse flattening as
/// if it were a defining value. See <https://geoweb.mit.edu/~tah/12.221_2005/grs80_corr.pdf>
pub struct GRS80;

impl Ellipsoid for GRS80 {
    const A: f64 = 6_378_137.0;
    const IF: f64 = 298.257_222_100_882_7;
}
