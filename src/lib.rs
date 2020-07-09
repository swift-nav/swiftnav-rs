//! Libswiftnav (LSN) is a library that implements GNSS utility functions for
//! use by software-defined GNSS receivers or software requiring GNSS
//! functionality.
//!
//! LSN does not provide any functionality for communicating with Swift
//! Navigation receivers. See [libsbp](https://github.com/swift-nav/libsbp) to
//! communicate with receivers using Swift Binary Protocol (SBP).

mod c_bindings;
pub mod coords;
pub mod signal;
pub mod time;
