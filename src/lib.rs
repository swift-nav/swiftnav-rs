//! GNSS systems are used to estimate the location of the receiver by determining
//! the distance between the receiver and several satelites. The satellites send
//! out precisely timed periodic messages and the receiver measures the delay
//! of those messages. Knowing the location of the satellites at the time of
//! transmission and the delays of the messages the receiver is able to determine
//! the location of itself in relation to the satellites.
//!
//! Libswiftnav is a library that implements GNSS utility functions to perform
//! position estimations. The data used by libswiftnav typically comes from GNSS
//! reciever chips as raw observation and ephemeris data.
//!
//! Libswiftnav does not provide any functionality for communicating with Swift
//! Navigation or any other receivers. See [libsbp](https://github.com/swift-nav/libsbp)
//! to communicate with receivers using Swift Binary Protocol (SBP).
//!

mod c_bindings;
pub mod coords;
pub mod edc;
pub mod ephemeris;
pub mod ionosphere;
pub mod navmeas;
pub mod signal;
pub mod time;
pub mod troposphere;
