//! `swiftnav-rs` is a library that implements GNSS utility functions to perform
//! position estimations. The data used by `swiftnav-rs` typically comes from GNSS
//! receiver chips as raw observation and ephemeris data. `swiftnav-rs` is more of
//! a "bring your own algorithm" library, it provides a bunch of functionality that
//! is useful when processing raw GNSS data, but it provides only limited position
//! estimation capabilities. Each module encompasses a single set of functionality,
//! and they are meant to be pretty self-explanatory for developers familiar with
//! GNSS processing.
//!
//! GNSS systems are used to estimate the location of the receiver by determining
//! the distance between the receiver and several satellites. The satellites send
//! out precisely timed periodic messages and the receiver measures the delay
//! of those messages. Knowing the location of the satellites at the time of
//! transmission and the delays of the messages the receiver is able to determine
//! the location of itself in relation to the satellites.
//!
//! `swiftnav-rs` does not provide any functionality for communicating with
//! receivers made by Swift Navigation, or any manufacturer.
//! [libsbp](https://github.com/swift-nav/libsbp) is the library to use if you
//! want to communicate with receivers using Swift Binary Protocol (SBP).
//!
//! ## Time
//! Time is a very important aspect of GNSS. `swiftnav-rs` defaults to representing
//! all times as GPS times. It provides the ability to manipulate GPS time stamps,
//! as well as means to convert a GPS time stamp into various other time bases
//! (GLONASS time, UTC, MJD).
//!
//! ## Coordinates
//! Several different coordinate types have representations and the ability to
//! convert between them. Earth centered earth fixed (ECEF), Latitude longitude and
//! height (both in radians and degrees), and Azimuth and elevation coordinates are
//! available.
//!
//! ## Ephemeris
//! Decoding and evaluation of broadcast ephemeris for all major GNSS constellations
//! is made available. You are able to calculate the satellite position at a
//! particular point in time in several different coordinates.
//!
//! ## Troposphere and Ionosphere
//! Two major sources of signal error in GNSS are the troposphere and ionosphere.
//! `swiftnav-rs` provides the ability to decode and use the broadcast Klobuchar
//! ionosphere model. An implementation of the UNM3m troposphere model is also
//! provided.
//!
//! ## Single epoch position solver
//! A simple least squares position solver is also included. This allows you to
//! get an approximate position with GNSS measurements from a single point in time.
//! It uses a least squares algorith, so no state is maintained between solves.
//! This can be used to seed your own position estimation algorithm with a rough
//! starting location.

mod c_bindings;
pub mod coords;
pub mod edc;
pub mod ephemeris;
pub mod ionosphere;
pub mod navmeas;
pub mod signal;
pub mod solver;
pub mod time;
pub mod troposphere;
