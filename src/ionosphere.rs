use crate::{c_bindings, time::GpsTime};

pub struct Ionosphere(c_bindings::ionosphere_t);

impl Ionosphere {
    pub fn new(
        t: GpsTime,
        a0: f64,
        a1: f64,
        a2: f64,
        a3: f64,
        b0: f64,
        b1: f64,
        b2: f64,
        b3: f64,
    ) -> Ionosphere {
        Ionosphere(c_bindings::ionosphere_t {
            toa: t.to_gps_time_t(),
            a0,
            a1,
            a2,
            a3,
            b0,
            b1,
            b2,
            b3,
        })
    }

    /// Decodes ionospheric parameters from GLS LNAV message subframe 4.
    ///
    /// The method decodes ionosphere data from GPS LNAV subframe 4 words 3-5.
    ///
    /// References:
    /// -# IS-GPS-200H, Section 20.3.3.5.1.7
    ///
    /// \param[in]  words    Subframe 4 page 18.
    pub fn decode_parameters(words: &[u32; 10]) -> Option<Ionosphere> {
        let mut iono = Ionosphere(c_bindings::ionosphere_t {
            toa: GpsTime::unknown(),
            a0: 0.0,
            a1: 0.0,
            a2: 0.0,
            a3: 0.0,
            b0: 0.0,
            b1: 0.0,
            b2: 0.0,
            b3: 0.0,
        });

        let success = unsafe { c_bindings::decode_iono_parameters(words.as_ptr(), &mut iono.0) };

        if success {
            Some(iono)
        } else {
            None
        }
    }

    /// Calculate ionospheric delay using Klobuchar model.
    ///
    /// References:
    ///   -# IS-GPS-200H, Section 20.3.3.5.2.5 and Figure 20-4
    ///
    /// \param t_gps GPS time at which to calculate the ionospheric delay
    /// \param lat_u Latitude of the receiver [rad]
    /// \param lon_u Longitude of the receiver [rad]
    /// \param a Azimuth of the satellite, clockwise positive from North [rad]
    /// \param e Elevation of the satellite [rad]
    /// \param i Ionosphere parameters struct from GPS NAV data
    ///
    /// \return Ionospheric delay distance for GPS L1 frequency [m]
    pub fn calc_delay(&self, t: &GpsTime, lat_u: f64, lon_u: f64, a: f64, e: f64) -> f64 {
        unsafe { c_bindings::calc_ionosphere(t.c_ptr(), lat_u, lon_u, a, e, &self.0) }
    }
}
