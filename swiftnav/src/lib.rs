// Copyright (c) 2020-2021 Swift Navigation Inc.
// Contact: Swift Navigation <dev@swiftnav.com>
//
// This source is subject to the license found in the file 'LICENSE' which must
// be be distributed together with this source. All other rights reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND,
// EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A PARTICULAR PURPOSE.
//! `swiftnav` is a library that implements GNSS utility functions to perform
//! position estimations. The data used by `swiftnav` typically comes from GNSS
//! receiver chips as raw observation and ephemeris data. `swiftnav` is more of
//! a "bring your own algorithm" library, it provides a bunch of functionality that
//! is useful when processing raw GNSS data. Each module encompasses a single set
//! of functionality, and they are meant to be pretty self-explanatory for
//! developers familiar with GNSS processing.
//!
//! GNSS systems are used to estimate the location of the receiver by determining
//! the distance between the receiver and several satellites. The satellites send
//! out precisely timed periodic messages and the receiver measures the delay
//! of those messages. Knowing the location of the satellites at the time of
//! transmission and the delays of the messages the receiver is able to determine
//! the location of itself in relation to the satellites.
//!
//! `swiftnav` does not provide any functionality for communicating with
//! receivers made by Swift Navigation, or any manufacturer.
//! [libsbp](https://github.com/swift-nav/libsbp) is the library to use if you
//! want to communicate with receivers using Swift Binary Protocol (SBP).
//!
//! ## [Signal](`signal`)
//! Types for identifying GNSS signals. Each satellite can send out multiple
//! signals, and each constellation of satellites support their own set of signals
//! and keeping track which is which is important.
//!
//! ## [Time](`time`)
//! Time is a very important aspect of GNSS. `swiftnav` defaults to representing
//! all times as GPS times. It provides the ability to manipulate GPS time stamps,
//! as well as means to convert a GPS time stamp into various other time bases
//! (GLONASS time, UTC, MJD).
//!
//! ## [Coordinates](`coords`)
//! Several different coordinate types have representations and the ability to
//! convert between them. Earth centered earth fixed (ECEF), Latitude longitude and
//! height (both in radians and degrees), and Azimuth and elevation coordinates are
//! available.
//!
//! ## [Checksums](`edc`)
//! Implementation of commonly used checksum algorithms used with GNSS data.
//!
//! ## [Geodetic Reference Frames](`reference_frame`)
//! Maps and GNSS tend to use their own reference frames (a.k.a datums), so it's
//! important to keep track which reference frame a position is in and be able to
//! transform positions in one reference frame into another so you can properly
//! compare positions.

pub mod coords;
pub mod edc;
mod math;
pub mod reference_frame;
pub mod signal;
pub mod time;
