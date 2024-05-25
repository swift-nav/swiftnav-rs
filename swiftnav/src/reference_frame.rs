use crate::coords::{Coordinate, ECEF};
use std::{convert::TryFrom, fmt, str::FromStr};

/// Reference Frames
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum ReferenceFrame {
    ITRF2008,
    ITRF2014,
    ITRF2020,
    ETRF2008,
    ETRF2014,
    ETRF2020,
    /// i.e. NAD83(2011)
    NAD83_2011,
    /// i.e. NAD83(CSRS) - Canadian Spatial Reference System
    #[allow(non_camel_case_types)]
    NAD83_CSRS,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct InvalidReferenceFrameName(String);

impl fmt::Display for InvalidReferenceFrameName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid reference frame name ({})", self.0)
    }
}

impl std::error::Error for InvalidReferenceFrameName {}

impl TryFrom<&str> for ReferenceFrame {
    type Error = InvalidReferenceFrameName;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "ITRF2008" => Ok(ReferenceFrame::ITRF2008),
            "ITRF2014" => Ok(ReferenceFrame::ITRF2014),
            "ITRF2020" => Ok(ReferenceFrame::ITRF2020),
            "ETRF2008" => Ok(ReferenceFrame::ETRF2008),
            "ETRF2014" => Ok(ReferenceFrame::ETRF2014),
            "ETRF2020" => Ok(ReferenceFrame::ETRF2020),
            "NAD83(2011)" => Ok(ReferenceFrame::NAD83_2011),
            "NAD83_2011" => Ok(ReferenceFrame::NAD83_2011),
            "NAD83(CSRS)" => Ok(ReferenceFrame::NAD83_CSRS),
            "NAD83_CSRS" => Ok(ReferenceFrame::NAD83_CSRS),
            _ => Err(InvalidReferenceFrameName(value.to_string())),
        }
    }
}

impl FromStr for ReferenceFrame {
    type Err = InvalidReferenceFrameName;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ReferenceFrame::try_from(s)
    }
}

impl From<ReferenceFrame> for &'static str {
    fn from(value: ReferenceFrame) -> &'static str {
        match value {
            ReferenceFrame::ITRF2008 => "ITRF2008",
            ReferenceFrame::ITRF2014 => "ITRF2014",
            ReferenceFrame::ITRF2020 => "ITRF2020",
            ReferenceFrame::ETRF2008 => "ETRF2008",
            ReferenceFrame::ETRF2014 => "ETRF2014",
            ReferenceFrame::ETRF2020 => "ETRF2020",
            ReferenceFrame::NAD83_2011 => "NAD83(2011)",
            ReferenceFrame::NAD83_CSRS => "NAD83(CSRS)",
        }
    }
}

impl fmt::Display for ReferenceFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}

/// 15-parameter Helmert transformation parameters
///
/// This transformation consists of a 3 dimensional translation,
/// 3 dimensional rotation, and a universal scaling. All terms,
/// except for the reference epoch, have a an additional time
/// dependent term. The rotations are typically very small, so
/// the small angle approximation is used.
///
/// There are several sign and scale conventions in use with
/// Helmert transformations. In this implementation we follow
/// the IERS conventions, meaning the translations are in
/// millimeters, the rotations are in milliarcseconds, and
/// the scaling is in parts per billion. We also follow the
/// IERS convention for the sign of the rotation terms.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct TimeDependentHelmertParams {
    tx: f64,
    tx_dot: f64,
    ty: f64,
    ty_dot: f64,
    tz: f64,
    tz_dot: f64,
    s: f64,
    s_dot: f64,
    rx: f64,
    rx_dot: f64,
    ry: f64,
    ry_dot: f64,
    rz: f64,
    rz_dot: f64,
    epoch: f64,
}

impl TimeDependentHelmertParams {
    const TRANSLATE_SCALE: f64 = 1.0e-3;
    const SCALE_SCALE: f64 = 1.0e-9;
    const ROTATE_SCALE: f64 = (std::f64::consts::PI / 180.0) * (0.001 / 3600.0);

    /// Reverses the transformation. Since this is a linear transformation we simply negate all terms
    fn invert(&mut self) {
        self.tx *= -1.0;
        self.tx_dot *= -1.0;
        self.ty *= -1.0;
        self.ty_dot *= -1.0;
        self.tz *= -1.0;
        self.tz_dot *= -1.0;
        self.s *= -1.0;
        self.s_dot *= -1.0;
        self.rx *= -1.0;
        self.rx_dot *= -1.0;
        self.ry *= -1.0;
        self.ry_dot *= -1.0;
        self.rz *= -1.0;
        self.rz_dot *= -1.0;
    }

    /// Apply the transformation on a position at a specific epoch
    fn transform_position(&self, position: &ECEF, epoch: f64) -> ECEF {
        let dt = epoch - self.epoch;
        let tx = (self.tx + self.tx_dot * dt) * Self::TRANSLATE_SCALE;
        let ty = (self.ty + self.ty_dot * dt) * Self::TRANSLATE_SCALE;
        let tz = (self.tz + self.tz_dot * dt) * Self::TRANSLATE_SCALE;
        let s = (self.s + self.s_dot * dt) * Self::SCALE_SCALE;
        let rx = (self.rx + self.rx_dot * dt) * Self::ROTATE_SCALE;
        let ry = (self.ry + self.ry_dot * dt) * Self::ROTATE_SCALE;
        let rz = (self.rz + self.rz_dot * dt) * Self::ROTATE_SCALE;

        let x = position.x() + tx + (s * position.x()) + (-rz * position.y()) + (ry * position.z());
        let y = position.y() + ty + (rz * position.x()) + (s * position.y()) + (-rx * position.z());
        let z = position.z() + tz + (-ry * position.x()) + (rx * position.y()) + (s * position.z());

        ECEF::new(x, y, z)
    }

    /// Apply the transformation on a velocity at a specific position
    fn transform_velocity(&self, velocity: &ECEF, position: &ECEF) -> ECEF {
        let tx = self.tx_dot * Self::TRANSLATE_SCALE;
        let ty = self.ty_dot * Self::TRANSLATE_SCALE;
        let tz = self.tz_dot * Self::TRANSLATE_SCALE;
        let s = self.s_dot * Self::SCALE_SCALE;
        let rx = self.rx_dot * Self::ROTATE_SCALE;
        let ry = self.ry_dot * Self::ROTATE_SCALE;
        let rz = self.rz_dot * Self::ROTATE_SCALE;

        let x = velocity.x() + tx + (s * position.x()) + (-rz * position.y()) + (ry * position.z());
        let y = velocity.y() + ty + (rz * position.x()) + (s * position.y()) + (-rx * position.z());
        let z = velocity.z() + tz + (-ry * position.x()) + (rx * position.y()) + (s * position.z());

        ECEF::new(x, y, z)
    }
}

/// Container for a complete transformation which includes
/// both the transformation parameters as well as the source
/// and destination reference frames
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Transformation {
    from: ReferenceFrame,
    to: ReferenceFrame,
    params: TimeDependentHelmertParams,
}

impl Transformation {
    /// Transform the given coordinate, producing a new coordinate.
    ///
    /// Reference frame transformations do not change the epoch of the
    /// coordinate.
    pub fn transform(&self, coord: &Coordinate) -> Coordinate {
        assert!(
            coord.reference_frame() == self.from,
            "Coordinate reference frame does not match transformation from reference frame"
        );

        let new_position = self.params.transform_position(
            &coord.position(),
            coord.epoch().to_fractional_year_hardcoded(),
        );
        let new_velocity = coord
            .velocity()
            .as_ref()
            .map(|velocity| self.params.transform_velocity(velocity, &coord.position()));
        Coordinate::new(self.to, new_position, new_velocity, coord.epoch())
    }

    /// Reverse the transformation
    pub fn invert(mut self) -> Self {
        std::mem::swap(&mut self.from, &mut self.to);
        self.params.invert();
        self
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct TransformationNotFound(ReferenceFrame, ReferenceFrame);

impl fmt::Display for TransformationNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No transformation found from {} to {}", self.0, self.1)
    }
}

impl std::error::Error for TransformationNotFound {}

/// Find a transformation from one reference frame to another
///
/// We currently only support a limited set of transformations.
/// If no transformation is found, `None` is returned.
pub fn get_transformation(
    from: ReferenceFrame,
    to: ReferenceFrame,
) -> Result<Transformation, TransformationNotFound> {
    TRANSFORMATIONS
        .iter()
        .find(|t| (t.from == from && t.to == to) || (t.from == to && t.to == from))
        .map(|t| {
            if t.from == from && t.to == to {
                t.clone()
            } else {
                t.clone().invert()
            }
        })
        .ok_or(TransformationNotFound(from, to))
}

const TRANSFORMATIONS: [Transformation; 9] = [
    Transformation {
        from: ReferenceFrame::ITRF2020,
        to: ReferenceFrame::ITRF2014,
        params: TimeDependentHelmertParams {
            tx: -1.4,
            tx_dot: 0.0,
            ty: -0.9,
            ty_dot: -0.1,
            tz: 1.4,
            tz_dot: 0.2,
            s: -0.42,
            s_dot: 0.0,
            rx: 0.0,
            rx_dot: 0.0,
            ry: 0.0,
            ry_dot: 0.0,
            rz: 0.0,
            rz_dot: 0.0,
            epoch: 2015.0,
        },
    },
    Transformation {
        from: ReferenceFrame::ITRF2020,
        to: ReferenceFrame::ITRF2008,
        params: TimeDependentHelmertParams {
            tx: 0.2,
            tx_dot: 0.0,
            ty: 1.0,
            ty_dot: -0.1,
            tz: 3.3,
            tz_dot: 0.1,
            s: -0.29,
            s_dot: 0.03,
            rx: 0.0,
            rx_dot: 0.0,
            ry: 0.0,
            ry_dot: 0.0,
            rz: 0.0,
            rz_dot: 0.0,
            epoch: 2015.0,
        },
    },
    Transformation {
        from: ReferenceFrame::ITRF2020,
        to: ReferenceFrame::ETRF2020,
        params: TimeDependentHelmertParams {
            tx: 0.0,
            tx_dot: 0.0,
            ty: 0.0,
            ty_dot: 0.0,
            tz: 0.0,
            tz_dot: 0.0,
            s: 0.0,
            s_dot: 0.0,
            rx: 0.0,
            rx_dot: 0.086,
            ry: 0.0,
            ry_dot: 0.519,
            rz: 0.0,
            rz_dot: -0.753,
            epoch: 1989.0,
        },
    },
    Transformation {
        from: ReferenceFrame::ITRF2020,
        to: ReferenceFrame::ETRF2014,
        params: TimeDependentHelmertParams {
            tx: -1.4,
            tx_dot: 0.0,
            ty: -0.9,
            ty_dot: -0.1,
            tz: 1.4,
            tz_dot: 0.2,
            s: -0.42,
            s_dot: 0.0,
            rx: 2.21,
            rx_dot: 0.085,
            ry: 13.806,
            ry_dot: 0.531,
            rz: -20.02,
            rz_dot: -0.77,
            epoch: 2015.0,
        },
    },
    Transformation {
        from: ReferenceFrame::ITRF2014,
        to: ReferenceFrame::NAD83_2011,
        params: TimeDependentHelmertParams {
            tx: 1005.30,
            tx_dot: 0.79,
            ty: -1909.21,
            ty_dot: -0.60,
            tz: -541.57,
            tz_dot: -1.44,
            s: 0.36891,
            s_dot: -0.07201,
            rx: -26.78138,
            rx_dot: -0.06667,
            ry: 0.42027,
            ry_dot: 0.75744,
            rz: -10.93206,
            rz_dot: 0.05133,
            epoch: 2010.0,
        },
    },
    Transformation {
        from: ReferenceFrame::ITRF2014,
        to: ReferenceFrame::ETRF2014,
        params: TimeDependentHelmertParams {
            tx: 0.0,
            tx_dot: 0.0,
            ty: 0.0,
            ty_dot: 0.0,
            tz: 0.0,
            tz_dot: 0.0,
            s: 0.0,
            s_dot: 0.0,
            rx: 0.0,
            rx_dot: 0.085,
            ry: 0.0,
            ry_dot: 0.531,
            rz: 0.0,
            rz_dot: -0.770,
            epoch: 1989.0,
        },
    },
    Transformation {
        from: ReferenceFrame::ITRF2008,
        to: ReferenceFrame::NAD83_CSRS,
        params: TimeDependentHelmertParams {
            tx: 1003.70,
            tx_dot: 0.79,
            ty: -1911.11,
            ty_dot: -0.60,
            tz: -543.97,
            tz_dot: -1.34,
            s: 0.38891,
            s_dot: -0.10201,
            rx: -26.78138,
            rx_dot: -0.06667,
            ry: 0.42027,
            ry_dot: 0.75744,
            rz: -10.93206,
            rz_dot: 0.05133,
            epoch: 2010.0,
        },
    },
    Transformation {
        from: ReferenceFrame::ITRF2014,
        to: ReferenceFrame::NAD83_CSRS,
        params: TimeDependentHelmertParams {
            tx: 1005.30,
            tx_dot: 0.79,
            ty: -1909.21,
            ty_dot: -0.60,
            tz: -541.57,
            tz_dot: -1.44,
            s: 0.36891,
            s_dot: -0.07201,
            rx: -26.78138,
            rx_dot: -0.06667,
            ry: 0.42027,
            ry_dot: 0.75744,
            rz: -10.93206,
            rz_dot: 0.05133,
            epoch: 2010.0,
        },
    },
    Transformation {
        from: ReferenceFrame::ITRF2020,
        to: ReferenceFrame::NAD83_CSRS,
        params: TimeDependentHelmertParams {
            tx: 1003.90,
            tx_dot: 0.79,
            ty: -1909.61,
            ty_dot: -0.70,
            tz: -541.17,
            tz_dot: -1.24,
            s: -0.05109,
            s_dot: -0.07201,
            rx: -26.78138,
            rx_dot: -0.06667,
            ry: 0.42027,
            ry_dot: 0.75744,
            rz: -10.93206,
            rz_dot: 0.05133,
            epoch: 2010.0,
        },
    },
];

#[cfg(test)]
mod tests {
    use float_eq::assert_float_eq;

    #[test]
    fn helmert_position_translations() {
        let params = super::TimeDependentHelmertParams {
            tx: 1.0 / super::TimeDependentHelmertParams::TRANSLATE_SCALE,
            tx_dot: 0.1 / super::TimeDependentHelmertParams::TRANSLATE_SCALE,
            ty: 2.0 / super::TimeDependentHelmertParams::TRANSLATE_SCALE,
            ty_dot: 0.2 / super::TimeDependentHelmertParams::TRANSLATE_SCALE,
            tz: 3.0 / super::TimeDependentHelmertParams::TRANSLATE_SCALE,
            tz_dot: 0.3 / super::TimeDependentHelmertParams::TRANSLATE_SCALE,
            s: 0.0,
            s_dot: 0.0,
            rx: 0.0,
            rx_dot: 0.0,
            ry: 0.0,
            ry_dot: 0.0,
            rz: 0.0,
            rz_dot: 0.0,
            epoch: 2010.0,
        };
        let initial_position = super::ECEF::default();

        let transformed_position = params.transform_position(&initial_position, 2010.0);
        assert_float_eq!(transformed_position.x(), 1.0, abs_all <= 1e-4);
        assert_float_eq!(transformed_position.y(), 2.0, abs_all <= 1e-4);
        assert_float_eq!(transformed_position.z(), 3.0, abs_all <= 1e-4);

        let transformed_position = params.transform_position(&initial_position, 2011.0);
        assert_float_eq!(transformed_position.x(), 1.1, abs_all <= 1e-4);
        assert_float_eq!(transformed_position.y(), 2.2, abs_all <= 1e-4);
        assert_float_eq!(transformed_position.z(), 3.3, abs_all <= 1e-4);
    }

    #[test]
    fn helmert_position_scaling() {
        let params = super::TimeDependentHelmertParams {
            tx: 0.0,
            tx_dot: 0.0,
            ty: 0.0,
            ty_dot: 0.0,
            tz: 0.0,
            tz_dot: 0.0,
            s: 1.0 / super::TimeDependentHelmertParams::SCALE_SCALE,
            s_dot: 0.1 / super::TimeDependentHelmertParams::SCALE_SCALE,
            rx: 90.0,
            rx_dot: 0.0,
            ry: 0.0,
            ry_dot: 0.0,
            rz: 0.0,
            rz_dot: 0.0,
            epoch: 2010.0,
        };
        let initial_position = super::ECEF::new(1., 2., 3.);

        let transformed_position = params.transform_position(&initial_position, 2010.0);
        assert_float_eq!(transformed_position.x(), 2.0, abs_all <= 1e-4);
        assert_float_eq!(transformed_position.y(), 4.0, abs_all <= 1e-4);
        assert_float_eq!(transformed_position.z(), 6.0, abs_all <= 1e-4);

        let transformed_position = params.transform_position(&initial_position, 2011.0);
        assert_float_eq!(transformed_position.x(), 2.1, abs_all <= 1e-4);
        assert_float_eq!(transformed_position.y(), 4.2, abs_all <= 1e-4);
        assert_float_eq!(transformed_position.z(), 6.3, abs_all <= 1e-4);
    }

    #[test]
    fn helmert_position_rotations() {
        let params = super::TimeDependentHelmertParams {
            tx: 0.0,
            tx_dot: 0.0,
            ty: 0.0,
            ty_dot: 0.0,
            tz: 0.0,
            tz_dot: 0.0,
            s: 0.0,
            s_dot: 0.0,
            rx: 1.0 / super::TimeDependentHelmertParams::ROTATE_SCALE,
            rx_dot: 0.1 / super::TimeDependentHelmertParams::ROTATE_SCALE,
            ry: 2.0 / super::TimeDependentHelmertParams::ROTATE_SCALE,
            ry_dot: 0.2 / super::TimeDependentHelmertParams::ROTATE_SCALE,
            rz: 3.0 / super::TimeDependentHelmertParams::ROTATE_SCALE,
            rz_dot: 0.3 / super::TimeDependentHelmertParams::ROTATE_SCALE,
            epoch: 2010.0,
        };
        let initial_position = super::ECEF::new(1.0, 1.0, 1.0);

        let transformed_position = params.transform_position(&initial_position, 2010.0);
        assert_float_eq!(transformed_position.x(), 0.0, abs_all <= 1e-4);
        assert_float_eq!(transformed_position.y(), 3.0, abs_all <= 1e-4);
        assert_float_eq!(transformed_position.z(), 0.0, abs_all <= 1e-4);

        let transformed_position = params.transform_position(&initial_position, 2011.0);
        assert_float_eq!(transformed_position.x(), -0.1, abs_all <= 1e-9);
        assert_float_eq!(transformed_position.y(), 3.2, abs_all <= 1e-9);
        assert_float_eq!(transformed_position.z(), -0.1, abs_all <= 1e-9);
    }

    #[test]
    fn helmert_velocity_translations() {
        let params = super::TimeDependentHelmertParams {
            tx: 1.0 / super::TimeDependentHelmertParams::TRANSLATE_SCALE,
            tx_dot: 0.1 / super::TimeDependentHelmertParams::TRANSLATE_SCALE,
            ty: 2.0 / super::TimeDependentHelmertParams::TRANSLATE_SCALE,
            ty_dot: 0.2 / super::TimeDependentHelmertParams::TRANSLATE_SCALE,
            tz: 3.0 / super::TimeDependentHelmertParams::TRANSLATE_SCALE,
            tz_dot: 0.3 / super::TimeDependentHelmertParams::TRANSLATE_SCALE,
            s: 0.0,
            s_dot: 0.0,
            rx: 0.0,
            rx_dot: 0.0,
            ry: 0.0,
            ry_dot: 0.0,
            rz: 0.0,
            rz_dot: 0.0,
            epoch: 2010.0,
        };
        let initial_velocity = super::ECEF::default();
        let position = super::ECEF::default();

        let transformed_velocity = params.transform_velocity(&initial_velocity, &position);
        assert_float_eq!(transformed_velocity.x(), 0.1, abs_all <= 1e-4);
        assert_float_eq!(transformed_velocity.y(), 0.2, abs_all <= 1e-4);
        assert_float_eq!(transformed_velocity.z(), 0.3, abs_all <= 1e-4);
    }

    #[test]
    fn helmert_velocity_scaling() {
        let params = super::TimeDependentHelmertParams {
            tx: 0.0,
            tx_dot: 0.0,
            ty: 0.0,
            ty_dot: 0.0,
            tz: 0.0,
            tz_dot: 0.0,
            s: 1.0 / super::TimeDependentHelmertParams::SCALE_SCALE,
            s_dot: 0.1 / super::TimeDependentHelmertParams::SCALE_SCALE,
            rx: 90.0,
            rx_dot: 0.0,
            ry: 0.0,
            ry_dot: 0.0,
            rz: 0.0,
            rz_dot: 0.0,
            epoch: 2010.0,
        };
        let initial_velocity = super::ECEF::default();
        let position = super::ECEF::new(1., 2., 3.);

        let transformed_velocity = params.transform_velocity(&initial_velocity, &position);
        assert_float_eq!(transformed_velocity.x(), 0.1, abs_all <= 1e-4);
        assert_float_eq!(transformed_velocity.y(), 0.2, abs_all <= 1e-4);
        assert_float_eq!(transformed_velocity.z(), 0.3, abs_all <= 1e-4);
    }

    #[test]
    fn helmert_velocity_rotations() {
        let params = super::TimeDependentHelmertParams {
            tx: 0.0,
            tx_dot: 0.0,
            ty: 0.0,
            ty_dot: 0.0,
            tz: 0.0,
            tz_dot: 0.0,
            s: 0.0,
            s_dot: 0.0,
            rx: 1.0 / super::TimeDependentHelmertParams::ROTATE_SCALE,
            rx_dot: 0.1 / super::TimeDependentHelmertParams::ROTATE_SCALE,
            ry: 2.0 / super::TimeDependentHelmertParams::ROTATE_SCALE,
            ry_dot: 0.2 / super::TimeDependentHelmertParams::ROTATE_SCALE,
            rz: 3.0 / super::TimeDependentHelmertParams::ROTATE_SCALE,
            rz_dot: 0.3 / super::TimeDependentHelmertParams::ROTATE_SCALE,
            epoch: 2010.0,
        };
        let initial_velocity = super::ECEF::default();
        let position = super::ECEF::new(4., 5., 6.);

        let transformed_velocity = params.transform_velocity(&initial_velocity, &position);
        assert_float_eq!(transformed_velocity.x(), -0.3, abs_all <= 1e-4);
        assert_float_eq!(transformed_velocity.y(), 0.6, abs_all <= 1e-4);
        assert_float_eq!(transformed_velocity.z(), -0.3, abs_all <= 1e-4);
    }

    #[test]
    fn helmert_invert() {
        let mut params = super::TimeDependentHelmertParams {
            tx: 1.0,
            tx_dot: 0.1,
            ty: 2.0,
            ty_dot: 0.2,
            tz: 3.0,
            tz_dot: 0.3,
            s: 4.0,
            s_dot: 0.4,
            rx: 5.0,
            rx_dot: 0.5,
            ry: 6.0,
            ry_dot: 0.6,
            rz: 7.0,
            rz_dot: 0.7,
            epoch: 2010.0,
        };
        params.invert();
        assert_float_eq!(params.tx, -1.0, abs_all <= 1e-4);
        assert_float_eq!(params.tx_dot, -0.1, abs_all <= 1e-4);
        assert_float_eq!(params.ty, -2.0, abs_all <= 1e-4);
        assert_float_eq!(params.ty_dot, -0.2, abs_all <= 1e-4);
        assert_float_eq!(params.tz, -3.0, abs_all <= 1e-4);
        assert_float_eq!(params.tz_dot, -0.3, abs_all <= 1e-4);
        assert_float_eq!(params.s, -4.0, abs_all <= 1e-4);
        assert_float_eq!(params.s_dot, -0.4, abs_all <= 1e-4);
        assert_float_eq!(params.rx, -5.0, abs_all <= 1e-4);
        assert_float_eq!(params.rx_dot, -0.5, abs_all <= 1e-4);
        assert_float_eq!(params.ry, -6.0, abs_all <= 1e-4);
        assert_float_eq!(params.ry_dot, -0.6, abs_all <= 1e-4);
        assert_float_eq!(params.rz, -7.0, abs_all <= 1e-4);
        assert_float_eq!(params.rz_dot, -0.7, abs_all <= 1e-4);
        assert_float_eq!(params.epoch, 2010.0, abs_all <= 1e-4);
    }
}
