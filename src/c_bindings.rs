pub(crate) mod gnss_time {
    // Include the C bindings
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!(concat!(env!("OUT_DIR"), "/gnss_time_bindings.rs"));
}