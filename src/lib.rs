//! Libswiftnav (LSN) is a library that implements GNSS utility functions for
//! use by software-defined GNSS receivers or software requiring GNSS
//! functionality.
//!
//! LSN does not provide any functionality for communicating with Swift
//! Navigation receivers. See [libsbp](https://github.com/swift-nav/libsbp) to
//! communicate with receivers using Swift Binary Protocol (SBP).

mod c_bindings;
pub mod coords;
pub mod ephemeris;
pub mod ionosphere;
pub mod signal;
pub mod time;
pub mod troposphere;

#[derive(Copy, Clone, Debug, Default)]
pub struct Vec3([f64; 3]);

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3([x, y, z])
    }

    pub fn from_array(a: &[f64; 3]) -> Vec3 {
        Vec3(a.clone())
    }

    pub fn get_x(&self) -> f64 {
        self.0[0]
    }

    pub fn get_y(&self) -> f64 {
        self.0[1]
    }

    pub fn get_z(&self) -> f64 {
        self.0[2]
    }

    pub fn set_x(&mut self, new_x: f64) {
        self.0[0] = new_x;
    }

    pub fn set_y(&mut self, new_y: f64) {
        self.0[1] = new_y;
    }

    pub fn set_z(&mut self, new_z: f64) {
        self.0[2] = new_z;
    }

    pub fn as_array_ref(&self) -> &[f64; 3] {
        &self.0
    }

    pub fn as_mut_array_ref(&mut self) -> &mut [f64; 3] {
        &mut self.0
    }
}

impl AsRef<[f64; 3]> for Vec3 {
    fn as_ref(&self) -> &[f64; 3] {
        &self.0
    }
}

impl AsMut<[f64; 3]> for Vec3 {
    fn as_mut(&mut self) -> &mut [f64; 3] {
        &mut self.0
    }
}
