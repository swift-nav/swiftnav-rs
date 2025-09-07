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
//! Transform coordinates between geodetic reference frames using time-dependent Helmert transformations.
//! Supports both global frames (ITRF series) and regional frames (ETRF, NAD83, etc.) with runtime
//! parameter loading.
//!
//! # Key Concepts
//!
//! - **Reference frames** define coordinate systems for Earth positioning
//! - **Crustal motion** causes positions to change over time, requiring velocity tracking
//! - **Transformations** use 15-parameter Helmert models with time dependencies
//! - **Path finding** automatically chains transformations between frames
//!
//! # Core Types
//!
//! - [`ReferenceFrame`] - Enumeration of supported coordinate systems
//! - [`TransformationRepository`] - Manages transformation parameters and pathfinding
//! - [`Coordinate`] - Position with reference frame, epoch, and optional velocity
//! - [`TimeDependentHelmertParams`] - 15-parameter transformation model
//!
//! # Basic Usage
//!
//! ```
//! use swiftnav::{
//!     coords::{Coordinate, ECEF},
//!     reference_frame::{TransformationRepository, ReferenceFrame},
//!     time::UtcTime
//! };
//!
//! let repo = TransformationRepository::from_builtin();
//! let epoch = UtcTime::from_parts(2020, 3, 15, 0, 0, 0.).to_gps_hardcoded();
//!
//! // Create coordinate with position and velocity
//! let itrf_coord = Coordinate::with_velocity(
//!     ReferenceFrame::ITRF2014,
//!     ECEF::new(-2703764.0, -4261273.0, 3887158.0),
//!     ECEF::new(-0.221, 0.254, 0.122),
//!     epoch
//! );
//!
//! // Transform to different reference frame
//! let nad83_coord = repo.transform(itrf_coord, &ReferenceFrame::NAD83_2011)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! # Custom Transformations
//!
//! ```
//! # use swiftnav::reference_frame::*;
//! let mut repo = TransformationRepository::new();
//!
//! let custom_transform = Transformation {
//!     from: ReferenceFrame::ITRF2020,
//!     to: ReferenceFrame::Other("LOCAL_FRAME".to_string()),
//!     params: TimeDependentHelmertParams { /* ... */ # tx: 0.0, tx_dot: 0.0, ty: 0.0, ty_dot: 0.0, tz: 0.0, tz_dot: 0.0, s: 0.0, s_dot: 0.0, rx: 0.0, rx_dot: 0.0, ry: 0.0, ry_dot: 0.0, rz: 0.0, rz_dot: 0.0, epoch: 2020.0 }
//! };
//!
//! repo.add_transformation(custom_transform);
//! ```

use crate::coords::{Coordinate, ECEF};
use nalgebra::{Matrix3, Vector3};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt,
};
use strum::{Display, EnumIter, EnumString};

mod params;

/// Geodetic reference frame identifiers
///
/// Enumerates well-known global and regional reference frames with support
/// for custom frames via the [`Other`] variant.
///
/// # Examples
/// ```
/// # use swiftnav::reference_frame::ReferenceFrame;
/// # use std::str::FromStr;
/// // Use predefined frames
/// let itrf = ReferenceFrame::ITRF2020;
/// let nad83 = ReferenceFrame::NAD83_2011;
///
/// // Parse from string
/// let parsed: ReferenceFrame = "ITRF2014".parse()?;
/// let custom: ReferenceFrame = "MY_LOCAL_FRAME".parse()?;
///
/// // Custom frames
/// let local = ReferenceFrame::Other("SITE_FRAME_2023".to_string());
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// [`Other`]: ReferenceFrame::Other
#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    EnumString,
    Display,
    EnumIter,
    Hash,
    Serialize,
    Deserialize,
)]
#[strum(serialize_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
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
    /// Custom reference frame with user-defined name
    #[strum(transparent, default)]
    #[serde(untagged)]
    Other(String),
}

impl PartialEq<&ReferenceFrame> for ReferenceFrame {
    fn eq(&self, other: &&ReferenceFrame) -> bool {
        self == *other
    }
}

impl PartialEq<ReferenceFrame> for &ReferenceFrame {
    fn eq(&self, other: &ReferenceFrame) -> bool {
        *self == other
    }
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
/// # Parameter Units
///
/// Input parameters are stored in standard geodetic units:
/// - **Translation** (`tx`, `ty`, `tz`): millimeters (mm)  
/// - **Translation rates** (`tx_dot`, `ty_dot`, `tz_dot`): mm/year
/// - **Scale** (`s`): parts per billion (ppb)
/// - **Scale rate** (`s_dot`): ppb/year  
/// - **Rotation** (`rx`, `ry`, `rz`): milliarcseconds (mas)
/// - **Rotation rates** (`rx_dot`, `ry_dot`, `rz_dot`): mas/year
/// - **Reference epoch** (`epoch`): decimal years
///
/// These units are automatically converted to SI units during computation.
///
/// # Sign Convention
///
/// There are several sign conventions in use for the rotation
/// parameters in Helmert transformations. In this implementation
/// we follow the IERS conventions, which is opposite of the original
/// formulation of the Helmert transformation.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Serialize, Deserialize)]
pub struct TimeDependentHelmertParams {
    t: Vector3<f64>,
    t_dot: Vector3<f64>,
    #[serde(alias = "scale", alias = "d")]
    s: f64,
    #[serde(alias = "scale_dot", alias = "d_dot")]
    s_dot: f64,
    r: Vector3<f64>,
    r_dot: Vector3<f64>,
    epoch: f64,
}

impl TimeDependentHelmertParams {
    /// Scale factor for translation parameters (converts mm to m)
    const TRANSLATE_SCALE: f64 = 1.0e-3;
    /// Scale factor for scale parameters (converts ppb to fractional scale)
    const SCALE_SCALE: f64 = 1.0e-9;
    /// Scale factor for rotation parameters (converts mas to radians)
    const ROTATE_SCALE: f64 = (std::f64::consts::PI / 180.0) * (0.001 / 3600.0);

    /// Reverses the transformation. Since this is a linear transformation we simply negate all terms
    pub fn invert(mut self) -> Self {
        self.t *= -1.0;
        self.t_dot *= -1.0;
        self.s *= -1.0;
        self.s_dot *= -1.0;
        self.r *= -1.0;
        self.r_dot *= -1.0;

        self
    }

    /// Apply the transformation to a position at a specific epoch
    #[must_use]
    pub fn transform_position(&self, position: &ECEF, epoch: f64) -> ECEF {
        let dt = epoch - self.epoch;
        let t = (self.t + self.t_dot * dt) * Self::TRANSLATE_SCALE;
        let s = (self.s + self.s_dot * dt) * Self::SCALE_SCALE;
        let r = (self.r + self.r_dot * dt) * Self::ROTATE_SCALE;

        let m = Self::make_rotation_matrix(s, r);

        (position.as_vector() + t + m * position.as_vector()).into()
    }

    /// Apply the transformation to a velocity at a specific position
    #[must_use]
    pub fn transform_velocity(&self, velocity: &ECEF, position: &ECEF) -> ECEF {
        let t = self.t_dot * Self::TRANSLATE_SCALE;
        let s = self.s_dot * Self::SCALE_SCALE;
        let r = self.r_dot * Self::ROTATE_SCALE;

        let m = Self::make_rotation_matrix(s, r);

        (velocity.as_vector() + t + m * position.as_vector()).into()
    }

    #[must_use]
    fn make_rotation_matrix(s: f64, r: Vector3<f64>) -> Matrix3<f64> {
        Matrix3::new(s, -r.z, r.y, r.z, s, -r.x, -r.y, r.x, s)
    }

    /// Apply the complete Helmert transformation to position and velocity
    ///
    /// Combines position and velocity transformations into a single operation.
    /// The velocity transformation uses the rate terms from the Helmert parameters.
    ///
    /// # Arguments
    /// * `position` - Position to transform
    /// * `velocity` - Optional velocity to transform (None preserves None)
    /// * `epoch` - Time epoch for position transformation
    ///
    /// # Returns
    /// Tuple of transformed (position, velocity)
    pub fn transform(
        &self,
        position: &ECEF,
        velocity: Option<&ECEF>,
        epoch: f64,
    ) -> (ECEF, Option<ECEF>) {
        let position = self.transform_position(position, epoch);
        let velocity = velocity.map(|v| self.transform_velocity(v, &position));

        (position, velocity)
    }
}

/// A transformation from one reference frame to another.
#[derive(Debug, PartialEq, PartialOrd, Clone, Serialize, Deserialize)]
pub struct Transformation {
    #[serde(alias = "source", alias = "source_name")]
    pub from: ReferenceFrame,
    #[serde(alias = "destination", alias = "destination_name")]
    pub to: ReferenceFrame,
    pub params: TimeDependentHelmertParams,
}

impl Transformation {
    /// Transform the given coordinate, producing a new coordinate.
    ///
    /// Reference frame transformations do not change the epoch of the
    /// coordinate.
    ///
    /// # Errors
    ///
    /// [`TransformationNotFound`] Is returned as an error if there is a mismatch between
    /// the coordinate reference frame and the [`Transformation::from`] field.
    #[must_use]
    pub fn transform(&self, coord: Coordinate) -> Result<Coordinate, TransformationNotFound> {
        if coord.reference_frame() != self.from {
            return Err(TransformationNotFound(
                self.from.clone(),
                coord.reference_frame().clone(),
            ));
        }

        let (new_position, new_velocity) = self.params.transform(
            &coord.position(),
            coord.velocity().as_ref(),
            coord.epoch().to_fractional_year_hardcoded(),
        );

        Ok(Coordinate::new(
            self.to.clone(),
            new_position,
            new_velocity,
            coord.epoch(),
        ))
    }

    /// Reverse the transformation
    #[must_use]
    pub fn invert(mut self) -> Self {
        std::mem::swap(&mut self.from, &mut self.to);
        self.params = self.params.invert();
        self
    }
}

/// Error indicating that no transformation was found between two reference frames
///
/// This error is returned when trying to find a transformation between two reference frames
/// and no transformation is found.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct TransformationNotFound(ReferenceFrame, ReferenceFrame);

impl fmt::Display for TransformationNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No transformation found from {} to {}", self.0, self.1)
    }
}

impl std::error::Error for TransformationNotFound {}

type TransformationGraph =
    HashMap<ReferenceFrame, HashMap<ReferenceFrame, TimeDependentHelmertParams>>;

/// A repository for managing reference frame transformations
///
/// This struct allows for loading the builtin transformations
/// as well as adding additional transformations from other sources.
#[derive(Debug, Clone)]
pub struct TransformationRepository {
    transformations: TransformationGraph,
}

impl TransformationRepository {
    /// Create an empty transformation repository
    pub fn new() -> Self {
        Self {
            transformations: TransformationGraph::new(),
        }
    }

    /// Create a repository from a list of transformations
    ///
    /// # Note
    ///
    /// If there are duplicated transformations in the list, the
    /// last one in the list will take priority
    pub fn from_transformations<T: IntoIterator<Item = Transformation>>(
        transformations: T,
    ) -> Self {
        let mut repo = Self::new();
        repo.extend(transformations);
        repo
    }

    /// Create a repository with the builtin transformations
    pub fn from_builtin() -> Self {
        Self::from_transformations(builtin_transformations())
    }

    /// Add a transformation to the repository
    ///
    /// This will rebuild the internal graph to include the new transformation.
    pub fn add_transformation(&mut self, transformation: Transformation) {
        let from = transformation.from;
        let to = transformation.to;
        let params = transformation.params;
        let inverted_params = params.invert();

        self.transformations
            .entry(from.clone())
            .or_default()
            .extend([(to.clone(), params)]);

        // Add inverted parameters as well
        self.transformations
            .entry(to)
            .or_default()
            .extend([(from, inverted_params)]);
    }

    /// Transform a [`Coordinate`] to a new reference frame
    ///
    /// This function finds the shortest series of transformations from the coordinate's
    /// initial reference frame to the requestest one, then sequentially applies
    /// those transformations to get the new position and velocity. The epoch of the
    /// coordinate is not modified in this process.
    ///
    /// # Errors
    ///
    /// [`TransformationNotFound`] is returned as an error if no path from the
    /// coordinate's reference frame to the requested reference frame could be found
    /// in the repository.
    pub fn transform(
        &self,
        coord: Coordinate,
        to: &ReferenceFrame,
    ) -> Result<Coordinate, TransformationNotFound> {
        let epoch = coord.epoch().to_fractional_year_hardcoded();

        let accumulate_transformations = |(position, velocity): (ECEF, Option<ECEF>),
                                          params: &TimeDependentHelmertParams|
         -> (ECEF, Option<ECEF>) {
            params.transform(&position, velocity.as_ref(), epoch)
        };

        let (position, velocity) = self
            .get_shortest_path(coord.reference_frame(), to)?
            .into_iter()
            .fold(
                (coord.position(), coord.velocity()),
                accumulate_transformations,
            );

        Ok(Coordinate::new(
            to.clone(),
            position,
            velocity,
            coord.epoch(),
        ))
    }

    /// Find the shortest transformation path between reference frames
    ///
    /// Uses breadth-first search to find the minimal sequence of transformations
    /// needed to convert between reference frames. The algorithm automatically
    /// chains transformations when no direct path exists.
    ///
    /// Returns an empty vector if source and destination frames are identical.
    ///
    /// # Examples
    /// ```
    /// # use swiftnav::reference_frame::*;
    /// let repo = TransformationRepository::from_builtin();
    ///
    /// // Direct transformation if available
    /// let path = repo.get_shortest_path(&ReferenceFrame::ITRF2020, &ReferenceFrame::ITRF2014)?;
    ///
    /// // Multi-hop transformation when needed
    /// let path = repo.get_shortest_path(&ReferenceFrame::ITRF2020, &ReferenceFrame::ETRF2000)?;
    /// assert!(path.len() >= 1);
    /// # Ok::<(), TransformationNotFound>(())
    /// ```
    ///
    /// # Errors
    ///
    /// [`TransformationNotFound`] if no transformation path exists between the frames
    fn get_shortest_path(
        &self,
        from: &ReferenceFrame,
        to: &ReferenceFrame,
    ) -> Result<Vec<&TimeDependentHelmertParams>, TransformationNotFound> {
        if from == to {
            return Ok(Vec::new());
        }

        let mut visited: HashSet<&ReferenceFrame> = HashSet::new();
        let mut queue: VecDeque<(&ReferenceFrame, Vec<&TimeDependentHelmertParams>)> =
            VecDeque::new();
        queue.push_back((from, Vec::new()));

        while let Some((current_frame, path)) = queue.pop_front() {
            if current_frame == to {
                return Ok(path);
            }

            if let Some(neighbors) = self.transformations.get(current_frame) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor.0) {
                        visited.insert(neighbor.0);
                        let mut new_path = path.clone();
                        new_path.push(neighbor.1);
                        queue.push_back((neighbor.0, new_path));
                    }
                }
            }
        }

        Err(TransformationNotFound(from.clone(), to.clone()))
    }

    /// Get the number of transformations stored in the repository
    pub fn count(&self) -> usize {
        self.transformations
            .values()
            .map(|neighbors| neighbors.len())
            .sum()
    }
}

impl Default for TransformationRepository {
    fn default() -> Self {
        Self::from_builtin()
    }
}

impl Extend<Transformation> for TransformationRepository {
    fn extend<T: IntoIterator<Item = Transformation>>(&mut self, iter: T) {
        iter.into_iter()
            .for_each(|transformation| self.add_transformation(transformation));
    }
}

/// Get the builtin transformation parameters
///
/// Returns a Vec of all pre-defined transformations between common reference frames
/// including ITRF, ETRF, NAD83, and WGS84 series. These transformations are sourced
/// from authoritative geodetic organizations.
///
/// Use this to initialize a [`TransformationRepository`] or for serialization.
///
/// # Example
/// ```
/// # use swiftnav::reference_frame::*;
/// let transformations = builtin_transformations();
/// let repo = TransformationRepository::from_transformations(transformations);
/// ```
pub fn builtin_transformations() -> Vec<Transformation> {
    params::TRANSFORMATIONS.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_eq::assert_float_eq;
    use params::TRANSFORMATIONS;
    use std::str::FromStr;

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
            t: Vector3::new(
                1.0 / TimeDependentHelmertParams::TRANSLATE_SCALE,
                2.0 / TimeDependentHelmertParams::TRANSLATE_SCALE,
                3.0 / TimeDependentHelmertParams::TRANSLATE_SCALE,
            ),
            t_dot: Vector3::new(
                0.1 / TimeDependentHelmertParams::TRANSLATE_SCALE,
                0.2 / TimeDependentHelmertParams::TRANSLATE_SCALE,
                0.3 / TimeDependentHelmertParams::TRANSLATE_SCALE,
            ),
            s: 0.0,
            s_dot: 0.0,
            r: Vector3::new(0.0, 0.0, 0.0),
            r_dot: Vector3::new(0.0, 0.0, 0.0),
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
            t: Vector3::new(0.0, 0.0, 0.0),
            t_dot: Vector3::new(0.0, 0.0, 0.0),
            s: 1.0 / TimeDependentHelmertParams::SCALE_SCALE,
            s_dot: 0.1 / TimeDependentHelmertParams::SCALE_SCALE,
            r: Vector3::new(90.0, 0.0, 0.0),
            r_dot: Vector3::new(0.0, 0.0, 0.0),
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
            t: Vector3::new(0.0, 0.0, 0.0),
            t_dot: Vector3::new(0.0, 0.0, 0.0),
            s: 0.0,
            s_dot: 0.0,
            r: Vector3::new(
                1.0 / TimeDependentHelmertParams::ROTATE_SCALE,
                2.0 / TimeDependentHelmertParams::ROTATE_SCALE,
                3.0 / TimeDependentHelmertParams::ROTATE_SCALE,
            ),
            r_dot: Vector3::new(
                0.1 / TimeDependentHelmertParams::ROTATE_SCALE,
                0.2 / TimeDependentHelmertParams::ROTATE_SCALE,
                0.3 / TimeDependentHelmertParams::ROTATE_SCALE,
            ),
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
            t: Vector3::new(
                1.0 / TimeDependentHelmertParams::TRANSLATE_SCALE,
                2.0 / TimeDependentHelmertParams::TRANSLATE_SCALE,
                3.0 / TimeDependentHelmertParams::TRANSLATE_SCALE,
            ),
            t_dot: Vector3::new(
                0.1 / TimeDependentHelmertParams::TRANSLATE_SCALE,
                0.2 / TimeDependentHelmertParams::TRANSLATE_SCALE,
                0.3 / TimeDependentHelmertParams::TRANSLATE_SCALE,
            ),
            s: 0.0,
            s_dot: 0.0,
            r: Vector3::new(0.0, 0.0, 0.0),
            r_dot: Vector3::new(0.0, 0.0, 0.0),
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
            t: Vector3::new(0.0, 0.0, 0.0),
            t_dot: Vector3::new(0.0, 0.0, 0.0),
            s: 1.0 / TimeDependentHelmertParams::SCALE_SCALE,
            s_dot: 0.1 / TimeDependentHelmertParams::SCALE_SCALE,
            r: Vector3::new(90.0, 0.0, 0.0),
            r_dot: Vector3::new(0.0, 0.0, 0.0),
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
            t: Vector3::new(0.0, 0.0, 0.0),
            t_dot: Vector3::new(0.0, 0.0, 0.0),
            s: 0.0,
            s_dot: 0.0,
            r: Vector3::new(
                1.0 / TimeDependentHelmertParams::ROTATE_SCALE,
                2.0 / TimeDependentHelmertParams::ROTATE_SCALE,
                3.0 / TimeDependentHelmertParams::ROTATE_SCALE,
            ),
            r_dot: Vector3::new(
                0.1 / TimeDependentHelmertParams::ROTATE_SCALE,
                0.2 / TimeDependentHelmertParams::ROTATE_SCALE,
                0.3 / TimeDependentHelmertParams::ROTATE_SCALE,
            ),
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
            t: Vector3::new(1.0, 2.0, 3.0),
            t_dot: Vector3::new(0.1, 0.2, 0.3),
            s: 4.0,
            s_dot: 0.4,
            r: Vector3::new(5.0, 6.0, 7.0),
            r_dot: Vector3::new(0.5, 0.6, 0.7),
            epoch: 2010.0,
        }
        .invert();
        assert_float_eq!(params.t.x, -1.0, abs_all <= 1e-4);
        assert_float_eq!(params.t_dot.x, -0.1, abs_all <= 1e-4);
        assert_float_eq!(params.t.y, -2.0, abs_all <= 1e-4);
        assert_float_eq!(params.t_dot.y, -0.2, abs_all <= 1e-4);
        assert_float_eq!(params.t.z, -3.0, abs_all <= 1e-4);
        assert_float_eq!(params.t_dot.z, -0.3, abs_all <= 1e-4);

        assert_float_eq!(params.s, -4.0, abs_all <= 1e-4);
        assert_float_eq!(params.s_dot, -0.4, abs_all <= 1e-4);
        assert_float_eq!(params.r.x, -5.0, abs_all <= 1e-4);
        assert_float_eq!(params.r_dot.x, -0.5, abs_all <= 1e-4);
        assert_float_eq!(params.r.y, -6.0, abs_all <= 1e-4);
        assert_float_eq!(params.r_dot.y, -0.6, abs_all <= 1e-4);
        assert_float_eq!(params.r.z, -7.0, abs_all <= 1e-4);
        assert_float_eq!(params.r_dot.z, -0.7, abs_all <= 1e-4);
        assert_float_eq!(params.epoch, 2010.0, abs_all <= 1e-4);
        let params = params.invert();
        assert_float_eq!(params.t.x, 1.0, abs_all <= 1e-4);
        assert_float_eq!(params.t_dot.x, 0.1, abs_all <= 1e-4);
        assert_float_eq!(params.t.y, 2.0, abs_all <= 1e-4);
        assert_float_eq!(params.t_dot.y, 0.2, abs_all <= 1e-4);
        assert_float_eq!(params.t.z, 3.0, abs_all <= 1e-4);
        assert_float_eq!(params.t_dot.z, 0.3, abs_all <= 1e-4);

        assert_float_eq!(params.s, 4.0, abs_all <= 1e-4);
        assert_float_eq!(params.s_dot, 0.4, abs_all <= 1e-4);
        assert_float_eq!(params.r.x, 5.0, abs_all <= 1e-4);
        assert_float_eq!(params.r_dot.x, 0.5, abs_all <= 1e-4);
        assert_float_eq!(params.r.y, 6.0, abs_all <= 1e-4);
        assert_float_eq!(params.r_dot.y, 0.6, abs_all <= 1e-4);
        assert_float_eq!(params.r.z, 7.0, abs_all <= 1e-4);
        assert_float_eq!(params.r_dot.z, 0.7, abs_all <= 1e-4);
        assert_float_eq!(params.epoch, 2010.0, abs_all <= 1e-4);
    }

    #[test]
    fn itrf2020_to_etrf2000_shortest_path() {
        let from = ReferenceFrame::ITRF2020;
        let to = ReferenceFrame::ETRF2000;

        // Make sure there isn't a direct path
        assert!(!TRANSFORMATIONS.iter().any(|t| t.from == from && t.to == to));

        let graph: TransformationRepository = TransformationRepository::from_builtin();
        let path = graph.get_shortest_path(&from, &to);
        assert!(path.is_ok());
        // Make sure that the path is correct. N.B. this may change if more transformations
        // are added in the future
        let path = path.unwrap();
        assert_eq!(path.len(), 2);
    }

    #[test]
    fn transformation_repository_empty() {
        let repo = TransformationRepository::new();
        assert_eq!(repo.count(), 0);

        let result = repo.get_shortest_path(&ReferenceFrame::ITRF2020, &ReferenceFrame::ITRF2014);
        assert!(result.is_err());
    }

    #[test]
    fn transformation_repository_from_builtin() {
        let repo = TransformationRepository::from_builtin();
        assert_eq!(repo.count(), TRANSFORMATIONS.len() * 2); // Also cound inverted transformations

        // Test path finding
        let path = repo.get_shortest_path(&ReferenceFrame::ITRF2020, &ReferenceFrame::ETRF2000);
        assert!(path.is_ok());
    }

    #[test]
    fn transformation_repository_add_transformation() {
        let mut repo = TransformationRepository::new();

        // Create a simple transformation for testing
        let transformation = Transformation {
            from: ReferenceFrame::ITRF2020,
            to: ReferenceFrame::ITRF2014,
            params: TimeDependentHelmertParams {
                tx: 1.0,
                tx_dot: 0.0,
                ty: 2.0,
                ty_dot: 0.0,
                tz: 3.0,
                tz_dot: 0.0,
                s: 0.0,
                s_dot: 0.0,
                rx: 0.0,
                rx_dot: 0.0,
                ry: 0.0,
                ry_dot: 0.0,
                rz: 0.0,
                rz_dot: 0.0,
                epoch: 2015.0,
            },
        };

        let params = transformation.params.clone();
        repo.add_transformation(transformation);
        assert_eq!(repo.count(), 2);

        let result = repo.get_shortest_path(&ReferenceFrame::ITRF2020, &ReferenceFrame::ITRF2014);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![&params]);

        // Test reverse transformation
        let params = params.invert();
        let reverse_result =
            repo.get_shortest_path(&ReferenceFrame::ITRF2014, &ReferenceFrame::ITRF2020);
        assert!(reverse_result.is_ok());
        assert_eq!(reverse_result.unwrap(), vec![&params]);
    }

    #[test]
    fn transformation_repository_from_transformations() {
        let transformations = vec![
            Transformation {
                from: ReferenceFrame::ITRF2020,
                to: ReferenceFrame::ITRF2014,
                params: TimeDependentHelmertParams {
                    tx: 1.0,
                    tx_dot: 0.0,
                    ty: 2.0,
                    ty_dot: 0.0,
                    tz: 3.0,
                    tz_dot: 0.0,
                    s: 0.0,
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
                from: ReferenceFrame::ITRF2014,
                to: ReferenceFrame::ITRF2000,
                params: TimeDependentHelmertParams {
                    tx: 4.0,
                    tx_dot: 0.0,
                    ty: 5.0,
                    ty_dot: 0.0,
                    tz: 6.0,
                    tz_dot: 0.0,
                    s: 0.0,
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
        ];

        let repo = TransformationRepository::from_transformations(transformations.clone());
        assert_eq!(repo.count(), 4);

        // Test direct transformation
        let result = repo.get_shortest_path(&ReferenceFrame::ITRF2020, &ReferenceFrame::ITRF2014);
        assert!(result.is_ok());

        // Test multi-step path
        let path = repo.get_shortest_path(&ReferenceFrame::ITRF2020, &ReferenceFrame::ITRF2000);
        assert!(path.is_ok());
        let path = path.unwrap();
        assert_eq!(path.len(), 2); // Two transformations: ITRF2020->ITRF2014 and ITRF2014->ITRF2000
        assert_eq!(
            path,
            vec![&transformations[0].params, &transformations[1].params]
        );
    }

    #[test]
    fn custom_reference_frame_creation() {
        let custom_frame = ReferenceFrame::Other("MyLocalFrame".to_string());
        assert_eq!(custom_frame.to_string(), "MyLocalFrame");
    }

    #[test]
    fn custom_reference_frame_from_str() {
        let custom_frame: ReferenceFrame = "UnknownFrame".parse().unwrap();
        assert_eq!(
            custom_frame,
            ReferenceFrame::Other("UnknownFrame".to_string())
        );
        assert_eq!(custom_frame.to_string(), "UnknownFrame");
    }

    #[test]
    fn known_reference_frame_from_str() {
        let itrf_frame: ReferenceFrame = "ITRF2020".parse().unwrap();
        assert_eq!(itrf_frame, ReferenceFrame::ITRF2020);

        let nad83_frame: ReferenceFrame = "NAD83(2011)".parse().unwrap();
        assert_eq!(nad83_frame, ReferenceFrame::NAD83_2011);

        let nad83_frame2: ReferenceFrame = "NAD83_2011".parse().unwrap();
        assert_eq!(nad83_frame2, ReferenceFrame::NAD83_2011);
    }

    #[test]
    fn custom_transformation() {
        let transformation = Transformation {
            from: ReferenceFrame::ITRF2020,
            to: ReferenceFrame::Other("LocalFrame".to_string()),
            params: TimeDependentHelmertParams {
                tx: 1.0,
                tx_dot: 0.0,
                ty: 2.0,
                ty_dot: 0.0,
                tz: 3.0,
                tz_dot: 0.0,
                s: 0.0,
                s_dot: 0.0,
                rx: 0.0,
                rx_dot: 0.0,
                ry: 0.0,
                ry_dot: 0.0,
                rz: 0.0,
                rz_dot: 0.0,
                epoch: 2020.0,
            },
        };

        assert_eq!(transformation.from, ReferenceFrame::ITRF2020);
        assert_eq!(
            transformation.to,
            ReferenceFrame::Other("LocalFrame".to_string())
        );
    }

    #[test]
    fn custom_transformation_repository() {
        let mut repo = TransformationRepository::new();
        let transformation = Transformation {
            from: ReferenceFrame::Other("Frame1".to_string()),
            to: ReferenceFrame::Other("Frame2".to_string()),
            params: TimeDependentHelmertParams {
                tx: 1.0,
                tx_dot: 0.0,
                ty: 2.0,
                ty_dot: 0.0,
                tz: 3.0,
                tz_dot: 0.0,
                s: 0.0,
                s_dot: 0.0,
                rx: 0.0,
                rx_dot: 0.0,
                ry: 0.0,
                ry_dot: 0.0,
                rz: 0.0,
                rz_dot: 0.0,
                epoch: 2020.0,
            },
        };

        repo.add_transformation(transformation);

        let result = repo.get_shortest_path(
            &ReferenceFrame::Other("Frame1".to_string()),
            &ReferenceFrame::Other("Frame2".to_string()),
        );
        assert!(result.is_ok());

        let reverse_result = repo.get_shortest_path(
            &ReferenceFrame::Other("Frame2".to_string()),
            &ReferenceFrame::Other("Frame1".to_string()),
        );
        assert!(reverse_result.is_ok());
    }

    #[cfg(test)]
    mod serialization_tests {
        use super::*;

        #[test]
        fn test_serde_roundtrip() {
            let original_transformations = builtin_transformations();

            // Serialize to JSON
            let json =
                serde_json::to_string(&original_transformations).expect("Failed to serialize");

            // Deserialize from JSON
            let deserialized_transformations: Vec<Transformation> =
                serde_json::from_str(&json).expect("Failed to deserialize");

            // Compare
            assert_eq!(
                original_transformations.len(),
                deserialized_transformations.len()
            );
            for (orig, deser) in original_transformations
                .iter()
                .zip(deserialized_transformations.iter())
            {
                assert_eq!(orig, deser);
            }
        }

        #[test]
        fn test_reference_frame_serde() {
            let frame = ReferenceFrame::ITRF2020;
            let json = serde_json::to_string(&frame).expect("Failed to serialize");
            let deserialized: ReferenceFrame =
                serde_json::from_str(&json).expect("Failed to deserialize");
            assert_eq!(frame, deserialized);
        }

        #[test]
        fn test_custom_reference_frame_serde() {
            let custom_frame = ReferenceFrame::Other("MyCustomFrame".to_string());
            let json = serde_json::to_string(&custom_frame).expect("Failed to serialize");
            let deserialized: ReferenceFrame =
                serde_json::from_str(&json).expect("Failed to deserialize");
            assert_eq!(custom_frame, deserialized);
            assert_eq!(json, "\"MyCustomFrame\"");
        }

        #[test]
        fn test_custom_transformation_serde() {
            let transformation = Transformation {
                from: ReferenceFrame::Other("FrameA".to_string()),
                to: ReferenceFrame::Other("FrameB".to_string()),
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
            };

            let json = serde_json::to_string(&transformation).expect("Failed to serialize");
            let deserialized: Transformation =
                serde_json::from_str(&json).expect("Failed to deserialize");
            assert_eq!(transformation, deserialized);
        }

        #[test]
        fn test_transformation_serde() {
            let transformation = Transformation {
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
            };

            let json = serde_json::to_string(&transformation).expect("Failed to serialize");
            let deserialized: Transformation =
                serde_json::from_str(&json).expect("Failed to deserialize");
            assert_eq!(transformation, deserialized);
        }

        #[test]
        fn test_serde_aliases() {
            // Test "source" and "destination"
            let json = serde_json::json!({
                "source": "ITRF2020",
                "destination": "ITRF2014",
                "params": {
                    "tx": -1.4,
                    "tx_dot": 0.0,
                    "ty": -0.9,
                    "ty_dot": -0.1,
                    "tz": 1.4,
                    "tz_dot": 0.2,
                    "s": -0.42,
                    "s_dot": 0.0,
                    "rx": 0.0,
                    "rx_dot": 0.0,
                    "ry": 0.0,
                    "ry_dot": 0.0,
                    "rz": 0.0,
                    "rz_dot": 0.0,
                    "epoch": 2015.0,
                }
            });

            let deserialized: Transformation = serde_json::from_value(json).unwrap();
            assert_eq!(deserialized.from, ReferenceFrame::ITRF2020);
            assert_eq!(deserialized.to, ReferenceFrame::ITRF2014);

            // Test "source_name" and destination_name"
            let json = serde_json::json!({
                "source_name": "ITRF2020",
                "destination_name": "ITRF2014",
                "params": {
                    "tx": -1.4,
                    "tx_dot": 0.0,
                    "ty": -0.9,
                    "ty_dot": -0.1,
                    "tz": 1.4,
                    "tz_dot": 0.2,
                    "s": -0.42,
                    "s_dot": 0.0,
                    "rx": 0.0,
                    "rx_dot": 0.0,
                    "ry": 0.0,
                    "ry_dot": 0.0,
                    "rz": 0.0,
                    "rz_dot": 0.0,
                    "epoch": 2015.0,
                }
            });

            let deserialized: Transformation = serde_json::from_value(json).unwrap();
            assert_eq!(deserialized.from, ReferenceFrame::ITRF2020);
            assert_eq!(deserialized.to, ReferenceFrame::ITRF2014);
        }
    }
}
