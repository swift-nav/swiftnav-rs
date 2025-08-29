// Copyright (c) 2024 Swift Navigation Inc.
// Contact: Swift Navigation <dev@swiftnav.com>
//
// This source is subject to the license found in the file 'LICENSE' which must
// be be distributed together with this source. All other rights reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND,
// EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A PARTICULAR PURPOSE.
//! Geodetic reference frame transformations
//!
//! Geodetic reference frames define the coordinate system used to represent
//! positions on the Earth. Different reference frames are commonly used in
//! different regions of the world, and for different purposes. For example,
//! global reference frames, such as the International Terrestrial Reference
//! Frame (ITRF), are used for global positioning, while regional reference
//! frames, such as the European Terrestrial Reference Frame (ETRF), are used
//! for regional positioning. Due to the movement of the earth's crust apparently
//! fixed positions will move over time. Because of this it's important to note
//! only take note of a position, but also the time at which that position was
//! determined. In most regions of the earth the crust moves at a constant speed,
//! meaning that if you are able to determine the local velocity of the crust you
//! can easily determine what the position of a static point would have been in
//! the past. It is commong for regional reference frames to define a common reference
//! epoch that all positions should be transformed to, allowing the direct comparison
//! of positions even if they were determined at different times. Regional reference
//! frames also typically are defined to be "fixed" to a particular tectonic plate,
//! meaning the large majority of the velocity for points on that tectonic plate
//! are cancelled out. In contrast, global reference frames are not fixed to
//! any particular tectonic plate, so most places on earth will have a measurable
//! velocity. Global reference frames also typically do not have a common reference
//! epoch, so determining one's local velocity is important to be able to compare
//! positions or to transform a coordinate from a global reference frame to a regional
//! reference frame.
//!
//! This module provides several types and functions to help transform a set of coordinates
//! from one reference frame to another, and from one epoch to another. Several sets of
//! transformation parameters are included for converting between common reference frames.
//! To start out, you must have a [`Coordinate`] that you want to transform. This consists of a
//! position, an epoch, and a reference frame as well as an optional velocity. You then need to
//! get the [`Transformation`] object that describes the transformation from the reference
//! frame of the coordinate to the desired reference frame. You can then call the `transform`
//! method on the transformation object to get a new coordinate in the desired reference frame.
//! This transformation will change the position and velocity of the coordinate, but it does
//! not the change the epoch of the coordinate. If you need to change the epoch of the
//! coordinate you will need to use the [`Coordinate::adjust_epoch`](crate::coords::Coordinate::adjust_epoch)
//! method which uses the velocity of the coordinate to determine the position at the new epoch.
//!
//! # Example
//! ```
//! use swiftnav::{
//!     coords::{Coordinate, ECEF},
//!     reference_frame::{get_transformation, ReferenceFrame, TransformationNotFound},
//!     time::UtcTime
//! };
//!
//! let transformation = get_transformation(ReferenceFrame::ITRF2014, ReferenceFrame::NAD83_2011)
//!     .unwrap();
//!
//! let epoch_2020 = UtcTime::from_parts(2020, 3, 15, 0, 0, 0.).to_gps_hardcoded();
//! let itrf_coord = Coordinate::with_velocity(
//!     ReferenceFrame::ITRF2014, // The reference frame of the coordinate
//!     ECEF::new(-2703764.0, -4261273.0, 3887158.0), // The position of the coordinate
//!     ECEF::new(-0.221, 0.254, 0.122), // The velocity of the coordinate
//!     epoch_2020); // The epoch of the coordinate
//!
//! let epoch_2010 = UtcTime::from_parts(2010, 1, 1, 0, 0, 0.).to_gps_hardcoded();
//! let itrf_coord = itrf_coord.adjust_epoch(&epoch_2010); // Change the epoch of the coordinate
//!
//! let nad83_coord = transformation.transform(&itrf_coord);
//! // Alternatively, you can use the `transform_to` method on the coordinate itself
//! let nad83_coord: Result<Coordinate, TransformationNotFound> =
//!     itrf_coord.transform_to(ReferenceFrame::NAD83_2011);
//! ```
//!

use crate::coords::{Coordinate, ECEF};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt,
};
use strum::{Display, EnumIter, EnumString};

mod params;

/// Reference Frames
#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, EnumString, Display, EnumIter, Hash,
)]
#[strum(serialize_all = "UPPERCASE")]
pub enum ReferenceFrame {
    ITRF88,
    ITRF89,
    ITRF90,
    ITRF91,
    ITRF92,
    ITRF93,
    ITRF94,
    ITRF96,
    ITRF97,
    ITRF2000,
    ITRF2005,
    ITRF2008,
    ITRF2014,
    ITRF2020,
    ETRF89,
    ETRF90,
    ETRF91,
    ETRF92,
    ETRF93,
    ETRF94,
    ETRF96,
    ETRF97,
    ETRF2000,
    ETRF2005,
    ETRF2014,
    ETRF2020,
    /// i.e. NAD83(2011)
    #[strum(to_string = "NAD83(2011)", serialize = "NAD83_2011")]
    NAD83_2011,
    /// i.e. NAD83(CSRS) - Canadian Spatial Reference System
    #[allow(non_camel_case_types)]
    #[strum(to_string = "NAD83(CSRS)", serialize = "NAD83_CSRS")]
    NAD83_CSRS,
    #[allow(non_camel_case_types)]
    #[strum(to_string = "DREF91(R2016)", serialize = "DREF91_R2016")]
    DREF91_R2016,
    #[allow(non_camel_case_types)]
    #[strum(to_string = "WGS84(G1762)", serialize = "WGS84_G1762")]
    WGS84_G1762,
    #[allow(non_camel_case_types)]
    #[strum(to_string = "WGS84(G2139)", serialize = "WGS84_G2139")]
    WGS84_G2139,
    #[allow(non_camel_case_types)]
    #[strum(to_string = "WGS84(G2296)", serialize = "WGS84_G2296")]
    WGS84_G2296,
}

/// 15-parameter Helmert transformation parameters
///
/// This is an extension of the 7-parameter Helmert transformation
/// where each term has an additional time-dependent term. This
/// transformation consists of a 3 dimensional translation,
/// 3 dimensional rotation, and a universal scaling. The tranformation
/// takes the form of:
///
/// $$
///  \begin{bmatrix} X \\\\ Y \\\\ Z \end{bmatrix}\_{REF2} =
///  \begin{bmatrix} X \\\\ Y \\\\ Z \end{bmatrix}\_{REF1} +
///  \begin{bmatrix} \bar{t}_x \\\\ \bar{t}_y \\\\ \bar{t}_z \end{bmatrix} +
///  \begin{bmatrix}   \bar{s} & -\bar{r}_z & \bar{r}_y \\\\
///                    \bar{r}_z & \bar{s} & -\bar{r}_x \\\\
///                    -\bar{r}_y  & \bar{r}_x & \bar{s} \end{bmatrix}
///  \begin{bmatrix} X \\\\ Y \\\\ Z \end{bmatrix}\_{REF1}
/// $$
///
/// Where each $\bar{}$ parameter in the transformation is time
/// dependent and is defined to be:
///
/// $$ \bar{p}(t) = p + \dot{p}(t - \tau) $$
///
/// Where $p$ is the constant value, $\dot{p}$ is the rate of
/// change, and $\tau$ is the reference epoch.
///
/// There are several sign conventions in use for the rotation
/// parameters in Helmert transformations. In this implementation
/// we follow the IERS conventions, which is opposite of the original
/// formulation of the Helmert transformation.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct TimeDependentHelmertParams {
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
    pub fn invert(&mut self) {
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
    pub fn transform_position(&self, position: &ECEF, epoch: f64) -> ECEF {
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
    pub fn transform_velocity(&self, velocity: &ECEF, position: &ECEF) -> ECEF {
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

/// A transformation from one reference frame to another.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Transformation {
    pub from: ReferenceFrame,
    pub to: ReferenceFrame,
    pub params: TimeDependentHelmertParams,
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

/// Error indicating that no transformation was found between two reference frames
///
/// This error is returned when trying to find a transformation between two reference frames
/// and no transformation is found.
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
    params::TRANSFORMATIONS
        .iter()
        .find(|t| (t.from == from && t.to == to) || (t.from == to && t.to == from))
        .map(|t| {
            if t.from == from && t.to == to {
                *t
            } else {
                (*t).invert()
            }
        })
        .ok_or(TransformationNotFound(from, to))
}

/// A helper type for finding transformations between reference frames that require multiple steps
///
/// This object can be used to determine which calls to [`get_transformation`]
/// are needed when a single transformation does not exist between two reference frames.
pub struct TransformationGraph {
    graph: HashMap<ReferenceFrame, HashSet<ReferenceFrame>>,
}

impl TransformationGraph {
    /// Create a new transformation graph, fully populated with the known transformations
    pub fn new() -> Self {
        let mut graph = HashMap::new();
        for transformation in params::TRANSFORMATIONS.iter() {
            graph
                .entry(transformation.from)
                .or_insert_with(HashSet::new)
                .insert(transformation.to);
            graph
                .entry(transformation.to)
                .or_insert_with(HashSet::new)
                .insert(transformation.from);
        }
        TransformationGraph { graph }
    }

    /// Get the shortest path between two reference frames, if one exists
    ///
    /// This function will also search for reverse paths if no direct path is found.
    /// The search is performed breadth-first.
    pub fn get_shortest_path(
        &self,
        from: ReferenceFrame,
        to: ReferenceFrame,
    ) -> Option<Vec<ReferenceFrame>> {
        if from == to {
            return None;
        }

        let mut visited: HashSet<ReferenceFrame> = HashSet::new();
        let mut queue: VecDeque<(ReferenceFrame, Vec<ReferenceFrame>)> = VecDeque::new();
        queue.push_back((from, vec![from]));

        while let Some((current_frame, path)) = queue.pop_front() {
            if current_frame == to {
                return Some(path);
            }

            if let Some(neighbors) = self.graph.get(&current_frame) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        visited.insert(*neighbor);
                        let mut new_path = path.clone();
                        new_path.push(*neighbor);
                        queue.push_back((*neighbor, new_path));
                    }
                }
            }
        }
        None
    }
}

impl Default for TransformationGraph {
    fn default() -> Self {
        TransformationGraph::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_eq::assert_float_eq;
    use params::TRANSFORMATIONS;
    use std::str::FromStr;
    use strum::IntoEnumIterator;

    #[test]
    fn reference_frame_strings() {
        assert_eq!(ReferenceFrame::ITRF88.to_string(), "ITRF88");
        assert_eq!(
            ReferenceFrame::from_str("ITRF88"),
            Ok(ReferenceFrame::ITRF88)
        );
        assert_eq!(ReferenceFrame::ITRF89.to_string(), "ITRF89");
        assert_eq!(
            ReferenceFrame::from_str("ITRF89"),
            Ok(ReferenceFrame::ITRF89)
        );
        assert_eq!(ReferenceFrame::ITRF90.to_string(), "ITRF90");
        assert_eq!(
            ReferenceFrame::from_str("ITRF90"),
            Ok(ReferenceFrame::ITRF90)
        );
        assert_eq!(ReferenceFrame::ITRF91.to_string(), "ITRF91");
        assert_eq!(
            ReferenceFrame::from_str("ITRF91"),
            Ok(ReferenceFrame::ITRF91)
        );
        assert_eq!(ReferenceFrame::ITRF92.to_string(), "ITRF92");
        assert_eq!(
            ReferenceFrame::from_str("ITRF92"),
            Ok(ReferenceFrame::ITRF92)
        );
        assert_eq!(ReferenceFrame::ITRF93.to_string(), "ITRF93");
        assert_eq!(
            ReferenceFrame::from_str("ITRF93"),
            Ok(ReferenceFrame::ITRF93)
        );
        assert_eq!(ReferenceFrame::ITRF94.to_string(), "ITRF94");
        assert_eq!(
            ReferenceFrame::from_str("ITRF94"),
            Ok(ReferenceFrame::ITRF94)
        );
        assert_eq!(ReferenceFrame::ITRF96.to_string(), "ITRF96");
        assert_eq!(
            ReferenceFrame::from_str("ITRF96"),
            Ok(ReferenceFrame::ITRF96)
        );
        assert_eq!(ReferenceFrame::ITRF97.to_string(), "ITRF97");
        assert_eq!(
            ReferenceFrame::from_str("ITRF97"),
            Ok(ReferenceFrame::ITRF97)
        );
        assert_eq!(ReferenceFrame::ITRF2000.to_string(), "ITRF2000");
        assert_eq!(
            ReferenceFrame::from_str("ITRF2000"),
            Ok(ReferenceFrame::ITRF2000)
        );
        assert_eq!(ReferenceFrame::ITRF2005.to_string(), "ITRF2005");
        assert_eq!(
            ReferenceFrame::from_str("ITRF2005"),
            Ok(ReferenceFrame::ITRF2005)
        );
        assert_eq!(ReferenceFrame::ITRF2008.to_string(), "ITRF2008");
        assert_eq!(
            ReferenceFrame::from_str("ITRF2008"),
            Ok(ReferenceFrame::ITRF2008)
        );
        assert_eq!(ReferenceFrame::ITRF2014.to_string(), "ITRF2014");
        assert_eq!(
            ReferenceFrame::from_str("ITRF2014"),
            Ok(ReferenceFrame::ITRF2014)
        );
        assert_eq!(ReferenceFrame::ITRF2020.to_string(), "ITRF2020");
        assert_eq!(
            ReferenceFrame::from_str("ITRF2020"),
            Ok(ReferenceFrame::ITRF2020)
        );
        assert_eq!(ReferenceFrame::ETRF89.to_string(), "ETRF89");
        assert_eq!(
            ReferenceFrame::from_str("ETRF89"),
            Ok(ReferenceFrame::ETRF89)
        );
        assert_eq!(ReferenceFrame::ETRF90.to_string(), "ETRF90");
        assert_eq!(
            ReferenceFrame::from_str("ETRF90"),
            Ok(ReferenceFrame::ETRF90)
        );
        assert_eq!(ReferenceFrame::ETRF91.to_string(), "ETRF91");
        assert_eq!(
            ReferenceFrame::from_str("ETRF91"),
            Ok(ReferenceFrame::ETRF91)
        );
        assert_eq!(ReferenceFrame::ETRF92.to_string(), "ETRF92");
        assert_eq!(
            ReferenceFrame::from_str("ETRF92"),
            Ok(ReferenceFrame::ETRF92)
        );
        assert_eq!(ReferenceFrame::ETRF93.to_string(), "ETRF93");
        assert_eq!(
            ReferenceFrame::from_str("ETRF93"),
            Ok(ReferenceFrame::ETRF93)
        );
        assert_eq!(ReferenceFrame::ETRF94.to_string(), "ETRF94");
        assert_eq!(
            ReferenceFrame::from_str("ETRF94"),
            Ok(ReferenceFrame::ETRF94)
        );
        assert_eq!(ReferenceFrame::ETRF96.to_string(), "ETRF96");
        assert_eq!(
            ReferenceFrame::from_str("ETRF96"),
            Ok(ReferenceFrame::ETRF96)
        );
        assert_eq!(ReferenceFrame::ETRF97.to_string(), "ETRF97");
        assert_eq!(
            ReferenceFrame::from_str("ETRF97"),
            Ok(ReferenceFrame::ETRF97)
        );
        assert_eq!(ReferenceFrame::ETRF2000.to_string(), "ETRF2000");
        assert_eq!(
            ReferenceFrame::from_str("ETRF2000"),
            Ok(ReferenceFrame::ETRF2000)
        );
        assert_eq!(ReferenceFrame::ETRF2005.to_string(), "ETRF2005");
        assert_eq!(
            ReferenceFrame::from_str("ETRF2005"),
            Ok(ReferenceFrame::ETRF2005)
        );
        assert_eq!(ReferenceFrame::ETRF2014.to_string(), "ETRF2014");
        assert_eq!(
            ReferenceFrame::from_str("ETRF2014"),
            Ok(ReferenceFrame::ETRF2014)
        );
        assert_eq!(ReferenceFrame::ETRF2020.to_string(), "ETRF2020");
        assert_eq!(
            ReferenceFrame::from_str("ETRF2020"),
            Ok(ReferenceFrame::ETRF2020)
        );
        assert_eq!(ReferenceFrame::NAD83_2011.to_string(), "NAD83(2011)");
        assert_eq!(
            ReferenceFrame::from_str("NAD83_2011"),
            Ok(ReferenceFrame::NAD83_2011)
        );
        assert_eq!(
            ReferenceFrame::from_str("NAD83(2011)"),
            Ok(ReferenceFrame::NAD83_2011)
        );
        assert_eq!(ReferenceFrame::NAD83_CSRS.to_string(), "NAD83(CSRS)");
        assert_eq!(
            ReferenceFrame::from_str("NAD83_CSRS"),
            Ok(ReferenceFrame::NAD83_CSRS)
        );
        assert_eq!(
            ReferenceFrame::from_str("NAD83(CSRS)"),
            Ok(ReferenceFrame::NAD83_CSRS)
        );
    }

    #[test]
    fn helmert_position_translations() {
        let params = TimeDependentHelmertParams {
            tx: 1.0 / TimeDependentHelmertParams::TRANSLATE_SCALE,
            tx_dot: 0.1 / TimeDependentHelmertParams::TRANSLATE_SCALE,
            ty: 2.0 / TimeDependentHelmertParams::TRANSLATE_SCALE,
            ty_dot: 0.2 / TimeDependentHelmertParams::TRANSLATE_SCALE,
            tz: 3.0 / TimeDependentHelmertParams::TRANSLATE_SCALE,
            tz_dot: 0.3 / TimeDependentHelmertParams::TRANSLATE_SCALE,
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
        let initial_position = ECEF::default();

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
        let params = TimeDependentHelmertParams {
            tx: 0.0,
            tx_dot: 0.0,
            ty: 0.0,
            ty_dot: 0.0,
            tz: 0.0,
            tz_dot: 0.0,
            s: 1.0 / TimeDependentHelmertParams::SCALE_SCALE,
            s_dot: 0.1 / TimeDependentHelmertParams::SCALE_SCALE,
            rx: 90.0,
            rx_dot: 0.0,
            ry: 0.0,
            ry_dot: 0.0,
            rz: 0.0,
            rz_dot: 0.0,
            epoch: 2010.0,
        };
        let initial_position = ECEF::new(1., 2., 3.);

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
        let params = TimeDependentHelmertParams {
            tx: 0.0,
            tx_dot: 0.0,
            ty: 0.0,
            ty_dot: 0.0,
            tz: 0.0,
            tz_dot: 0.0,
            s: 0.0,
            s_dot: 0.0,
            rx: 1.0 / TimeDependentHelmertParams::ROTATE_SCALE,
            rx_dot: 0.1 / TimeDependentHelmertParams::ROTATE_SCALE,
            ry: 2.0 / TimeDependentHelmertParams::ROTATE_SCALE,
            ry_dot: 0.2 / TimeDependentHelmertParams::ROTATE_SCALE,
            rz: 3.0 / TimeDependentHelmertParams::ROTATE_SCALE,
            rz_dot: 0.3 / TimeDependentHelmertParams::ROTATE_SCALE,
            epoch: 2010.0,
        };
        let initial_position = ECEF::new(1.0, 1.0, 1.0);

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
        let params = TimeDependentHelmertParams {
            tx: 1.0 / TimeDependentHelmertParams::TRANSLATE_SCALE,
            tx_dot: 0.1 / TimeDependentHelmertParams::TRANSLATE_SCALE,
            ty: 2.0 / TimeDependentHelmertParams::TRANSLATE_SCALE,
            ty_dot: 0.2 / TimeDependentHelmertParams::TRANSLATE_SCALE,
            tz: 3.0 / TimeDependentHelmertParams::TRANSLATE_SCALE,
            tz_dot: 0.3 / TimeDependentHelmertParams::TRANSLATE_SCALE,
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
        let initial_velocity = ECEF::default();
        let position = ECEF::default();

        let transformed_velocity = params.transform_velocity(&initial_velocity, &position);
        assert_float_eq!(transformed_velocity.x(), 0.1, abs_all <= 1e-4);
        assert_float_eq!(transformed_velocity.y(), 0.2, abs_all <= 1e-4);
        assert_float_eq!(transformed_velocity.z(), 0.3, abs_all <= 1e-4);
    }

    #[test]
    fn helmert_velocity_scaling() {
        let params = TimeDependentHelmertParams {
            tx: 0.0,
            tx_dot: 0.0,
            ty: 0.0,
            ty_dot: 0.0,
            tz: 0.0,
            tz_dot: 0.0,
            s: 1.0 / TimeDependentHelmertParams::SCALE_SCALE,
            s_dot: 0.1 / TimeDependentHelmertParams::SCALE_SCALE,
            rx: 90.0,
            rx_dot: 0.0,
            ry: 0.0,
            ry_dot: 0.0,
            rz: 0.0,
            rz_dot: 0.0,
            epoch: 2010.0,
        };
        let initial_velocity = ECEF::default();
        let position = ECEF::new(1., 2., 3.);

        let transformed_velocity = params.transform_velocity(&initial_velocity, &position);
        assert_float_eq!(transformed_velocity.x(), 0.1, abs_all <= 1e-4);
        assert_float_eq!(transformed_velocity.y(), 0.2, abs_all <= 1e-4);
        assert_float_eq!(transformed_velocity.z(), 0.3, abs_all <= 1e-4);
    }

    #[test]
    fn helmert_velocity_rotations() {
        let params = TimeDependentHelmertParams {
            tx: 0.0,
            tx_dot: 0.0,
            ty: 0.0,
            ty_dot: 0.0,
            tz: 0.0,
            tz_dot: 0.0,
            s: 0.0,
            s_dot: 0.0,
            rx: 1.0 / TimeDependentHelmertParams::ROTATE_SCALE,
            rx_dot: 0.1 / TimeDependentHelmertParams::ROTATE_SCALE,
            ry: 2.0 / TimeDependentHelmertParams::ROTATE_SCALE,
            ry_dot: 0.2 / TimeDependentHelmertParams::ROTATE_SCALE,
            rz: 3.0 / TimeDependentHelmertParams::ROTATE_SCALE,
            rz_dot: 0.3 / TimeDependentHelmertParams::ROTATE_SCALE,
            epoch: 2010.0,
        };
        let initial_velocity = ECEF::default();
        let position = ECEF::new(4., 5., 6.);

        let transformed_velocity = params.transform_velocity(&initial_velocity, &position);
        assert_float_eq!(transformed_velocity.x(), -0.3, abs_all <= 1e-4);
        assert_float_eq!(transformed_velocity.y(), 0.6, abs_all <= 1e-4);
        assert_float_eq!(transformed_velocity.z(), -0.3, abs_all <= 1e-4);
    }

    #[test]
    fn helmert_invert() {
        let mut params = TimeDependentHelmertParams {
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
        params.invert();
        assert_float_eq!(params.tx, 1.0, abs_all <= 1e-4);
        assert_float_eq!(params.tx_dot, 0.1, abs_all <= 1e-4);
        assert_float_eq!(params.ty, 2.0, abs_all <= 1e-4);
        assert_float_eq!(params.ty_dot, 0.2, abs_all <= 1e-4);
        assert_float_eq!(params.tz, 3.0, abs_all <= 1e-4);
        assert_float_eq!(params.tz_dot, 0.3, abs_all <= 1e-4);
        assert_float_eq!(params.s, 4.0, abs_all <= 1e-4);
        assert_float_eq!(params.s_dot, 0.4, abs_all <= 1e-4);
        assert_float_eq!(params.rx, 5.0, abs_all <= 1e-4);
        assert_float_eq!(params.rx_dot, 0.5, abs_all <= 1e-4);
        assert_float_eq!(params.ry, 6.0, abs_all <= 1e-4);
        assert_float_eq!(params.ry_dot, 0.6, abs_all <= 1e-4);
        assert_float_eq!(params.rz, 7.0, abs_all <= 1e-4);
        assert_float_eq!(params.rz_dot, 0.7, abs_all <= 1e-4);
        assert_float_eq!(params.epoch, 2010.0, abs_all <= 1e-4);
    }

    #[test]
    fn itrf2020_to_etrf2000_shortest_path() {
        let from = ReferenceFrame::ITRF2020;
        let to = ReferenceFrame::ETRF2000;

        // Make sure there isn't a direct path
        assert!(!TRANSFORMATIONS.iter().any(|t| t.from == from && t.to == to));

        let graph = TransformationGraph::new();
        let path = graph.get_shortest_path(from, to);
        assert!(path.is_some());
        // Make sure that the path is correct. N.B. this may change if more transformations
        // are added in the future
        let path = path.unwrap();
        assert_eq!(path.len(), 3);
        assert_eq!(path[0], from);
        assert_eq!(path[1], ReferenceFrame::ITRF2000);
        assert_eq!(path[2], to);
    }

    #[test]
    fn fully_traversable_graph() {
        let graph = TransformationGraph::new();
        for from in ReferenceFrame::iter() {
            for to in ReferenceFrame::iter() {
                if from == to {
                    continue;
                }
                let path = graph.get_shortest_path(from, to);
                assert!(path.is_some(), "No path from {} to {}", from, to);
            }
        }
    }
}
