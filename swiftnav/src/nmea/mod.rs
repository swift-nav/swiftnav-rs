// Copyright (c) 2025 Swift Navigation Inc.
// Contact: Swift Navigation <dev@swiftnav.com>
//
// This source is subject to the license found in the file 'LICENSE' which must
// be be distributed together with this source. All other rights reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND,
// EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A PARTICULAR PURPOSE.

//! This module contains National Marine Electronics Association (NMEA) related structures and
//! formatting utilities. Notably, it contains (or eventually will contain) structures related to
//! NMEA sentences and parsing/serialization of those sentences.

mod checksum;
mod gga;
mod source;

pub use checksum::*;
pub use gga::*;
pub use source::*;
