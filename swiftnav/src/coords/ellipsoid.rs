use super::{AzimuthElevation, LLHRadians, ECEF, NED};
use crate::math::compile_time_sqrt;
use nalgebra::Matrix3;

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

    /// Convert a [`LLHRadians`] to [`ECEF`]
    #[must_use]
    fn llh2ecef(llh: &LLHRadians) -> ECEF {
        let d = Self::E * (llh.latitude()).sin();
        let n = Self::A / (1. - d * d).sqrt();

        let x = (n + llh.height()) * llh.latitude().cos() * llh.longitude().cos();
        let y = (n + llh.height()) * llh.latitude().cos() * llh.longitude().sin();
        let z = ((1.0 - Self::E * Self::E) * n + llh.height()) * llh.latitude().sin();

        ECEF::new(x, y, z)
    }

    /// Convert an [`ECEF`] to [`LLHRadians`]
    #[must_use]
    fn ecef2llh(ecef: &ECEF) -> LLHRadians {
        // Distance from polar axis.
        let p = (ecef.x() * ecef.x() + ecef.y() * ecef.y()).sqrt();

        // Compute longitude first, this can be done exactly.
        let longitude = if p != 0.0 {
            ecef.y().atan2(ecef.x())
        } else {
            0.0
        };

        // If we are close to the pole then convergence is very slow, treat this is a
        // special case.
        if p < Self::A * 1e-16 {
            let latitude = std::f64::consts::FRAC_PI_2.copysign(ecef.z());
            let height = ecef.z().abs() - Self::B;
            return LLHRadians::new(latitude, longitude, height);
        }

        // Caluclate some other constants as defined in the Fukushima paper.
        let p_norm = p / Self::A;
        let e_c = (1. - Self::E * Self::E).sqrt();
        let z = ecef.z().abs() * e_c / Self::A;

        // Initial values for S and C correspond to a zero height solution.
        let mut s = z;
        let mut c = e_c * p_norm;

        // Neither S nor C can be negative on the first iteration so
        // starting prev = -1 will not cause and early exit.
        let mut prev_c = -1.0;
        let mut prev_s = -1.0;

        let mut a_n;
        let mut b_n;
        let mut d_n;
        let mut f_n;

        // Iterate a maximum of 10 times. This should be way more than enough for all
        // sane inputs
        for _ in 0..10 {
            // Calculate some intermediate variables used in the update step based on
            // the current state.
            a_n = (s * s + c * c).sqrt();
            d_n = z * a_n * a_n * a_n + Self::E * Self::E * s * s * s;
            f_n = p_norm * a_n * a_n * a_n - Self::E * Self::E * c * c * c;
            b_n = 1.5 * Self::E * s * c * c * (a_n * (p_norm * s - z * c) - Self::E * s * c);

            // Update step.
            s = d_n * f_n - b_n * s;
            c = f_n * f_n - b_n * c;

            // The original algorithm as presented in the paper by Fukushima has a
            // problem with numerical stability. S and C can grow very large or small
            // and over or underflow a double. In the paper this is acknowledged and
            // the proposed resolution is to non-dimensionalise the equations for S and
            // C. However, this does not completely solve the problem. The author caps
            // the solution to only a couple of iterations and in this period over or
            // underflow is unlikely but as we require a bit more precision and hence
            // more iterations so this is still a concern for us.
            //
            // As the only thing that is important is the ratio T = S/C, my solution is
            // to divide both S and C by either S or C. The scaling is chosen such that
            // one of S or C is scaled to unity whilst the other is scaled to a value
            // less than one. By dividing by the larger of S or C we ensure that we do
            // not divide by zero as only one of S or C should ever be zero.
            //
            // This incurs an extra division each iteration which the author was
            // explicityl trying to avoid and it may be that this solution is just
            // reverting back to the method of iterating on T directly, perhaps this
            // bears more thought?

            if s > c {
                c /= s;
                s = 1.0;
            } else {
                s /= c;
                c = 1.0;
            }

            // Check for convergence and exit early if we have converged.
            if (s - prev_s).abs() < 1e-16 && (c - prev_c).abs() < 1e-16 {
                break;
            }
            prev_s = s;
            prev_c = c;
        }

        a_n = (s * s + c * c).sqrt();
        let latitude = 1.0_f64.copysign(ecef.z()) * (s / (e_c * c)).atan();
        let height = (p * e_c * c + ecef.z().abs() * s - Self::A * e_c * a_n)
            / (e_c * e_c * c * c + s * s).sqrt();
        LLHRadians::new(latitude, longitude, height)
    }

    /// Rotate an [`ECEF`] vector into the a local [`NED`] vector
    #[must_use]
    fn ecef2ned(ecef: &ECEF, ref_ecef: &ECEF) -> NED {
        let m = ecef2ned_matrix(Self::ecef2llh(ref_ecef));
        NED::from_vector3(m * ecef.as_vector_ref())
    }

    /// Rotate a local [`NED`] vector into an [`ECEF`] vector
    #[must_use]
    fn ned2ecef(ned: &NED, ref_ecef: &ECEF) -> ECEF {
        let m = ecef2ned_matrix(Self::ecef2llh(ref_ecef));
        ECEF::from_vector3(m.transpose() * ned.as_vector_ref())
    }

    /// Calculate the local [`NED`] vector between two [`ECEF`] positions
    #[must_use]
    fn ecef2ned_d(ecef: &ECEF, ref_ecef: &ECEF) -> NED {
        let temp_vector = ecef - ref_ecef;
        Self::ecef2ned(&temp_vector, ref_ecef)
    }

    /// Calculate the [`AzimuthElevation`] from one [`ECEF`] position to another
    ///
    /// First the vector between the points is converted into the local North, East,
    /// Down frame of the reference point. Then we can directly calculate the
    /// azimuth and elevation.
    #[must_use]
    fn ecef2azel(ecef: &ECEF, ref_ecef: &ECEF) -> AzimuthElevation {
        /* Calculate the vector from the reference point in the local North, East,
         * Down frame of the reference point. */
        let ned = Self::ecef2ned_d(ecef, ref_ecef);

        let azimuth = ned.e().atan2(ned.n());
        /* atan2 returns angle in range [-pi, pi], usually azimuth is defined in the
         * range [0, 2pi]. */
        let azimuth = if azimuth < 0.0 {
            azimuth + 2.0 * std::f64::consts::PI
        } else {
            azimuth
        };

        let elevation = (-ned.d() / ned.as_vector_ref().norm()).asin();
        AzimuthElevation::new(azimuth, elevation)
    }
}

/// Calculate the rotation matrix for rotating between an [`ECEF`] and [`NED`] frames
#[must_use]
fn ecef2ned_matrix(llh: LLHRadians) -> Matrix3<f64> {
    let sin_lat = llh.latitude().sin();
    let cos_lat = llh.latitude().cos();
    let sin_lon = llh.longitude().sin();
    let cos_lon = llh.longitude().cos();

    Matrix3::new(
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
