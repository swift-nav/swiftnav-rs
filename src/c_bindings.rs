// Include the C bindings
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl Copy for gnss_signal_t {}

impl Clone for gnss_signal_t {
    fn clone(&self) -> Self {
        *self
    }
}
