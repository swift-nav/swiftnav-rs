// Copyright (c) 2020-2021 Swift Navigation Inc.
// Contact: Swift Navigation <dev@swiftnav.com>
//
// This source is subject to the license found in the file 'LICENSE' which must
// be be distributed together with this source. All other rights reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND,
// EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A PARTICULAR PURPOSE.
//! Geoid height calculation
//!
//! GNSS positions have a height which is usually referenced to a ellipsoid,
//! a smooth surface which very roughly approximates the mean sea level. This
//! ellipsoid can deviate by a large amount for the actual local mean sea level
//! due to the variations in the local gravitational field. A geoid model is
//! built to better approximate these variations in mean sea level, and can be
//! used to give a height relative to mean sea level which can be more helpful
//! to an end user.

use crate::coords::LLHRadians;

/// List of potential Geoid models used
///
/// Currently only one model is compiled into the code at a time
pub enum GeoidModel {
    /// The EGM96 geoid model
    Egm96,
    /// The EGM2008 geoid model
    Egm2008,
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
    let model = unsafe { swiftnav_sys::get_geoid_model() };

    match model {
        swiftnav_sys::geoid_model_t_GEOID_MODEL_EGM96 => GeoidModel::Egm96,
        swiftnav_sys::geoid_model_t_GEOID_MODEL_EGM2008 => GeoidModel::Egm2008,
        _ => unimplemented!("Unknown geoid model {}", model),
    }
}
