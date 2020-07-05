extern crate bindgen;

use cmake::Config;
use std::env;
use std::path::PathBuf;

fn main() {
    let dst = Config::new("third-party/libswiftnav/").build();

    println!("cargo:rustc-link-search=native={}/lib/", dst.display());
    println!("cargo:rustc-link-lib=static=swiftnav");

    let include_args = vec![
        "-isystem".to_string(),
        format!("{}/include/", dst.display()),
    ]
    .into_iter();
    let bindings = bindgen::Builder::default()
        .clang_args(include_args)
        // The input header we would like to generate
        // bindings for.
        .header(format!("{}/include/swiftnav/signal.h", dst.display()))
        .header(format!("{}/include/swiftnav/gnss_time.h", dst.display()))
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .blacklist_type("u8")
        .blacklist_type("u16")
        .blacklist_type("u32")
        .whitelist_type("gps_time_t")
        .whitelist_function("gpsdifftime")
        .whitelist_function("gps_time_valid")
        .whitelist_function("add_secs")
        .whitelist_var("FLOAT_EQUALITY_EPS")
        .whitelist_var("MINUTE_SECS")
        .whitelist_var("HOUR_SECS")
        .whitelist_var("DAY_SECS")
        .whitelist_var("WEEK_SECS")
        .whitelist_type("constellation_t")
        .whitelist_type("code_t")
        .whitelist_type("gnss_signal_t")
        .whitelist_function("is_gps")
        .whitelist_function("is_sbas")
        .whitelist_function("is_glo")
        .whitelist_function("is_bds2")
        .whitelist_function("is_gal")
        .whitelist_function("is_qzss")
        .whitelist_function("sid_to_constellation")
        .whitelist_function("code_to_constellation")
        .whitelist_function("constellation_to_sat_count")
        .whitelist_function("code_to_sig_count")
        .whitelist_function("code_to_chip_count")
        .whitelist_function("code_to_chip_rate")
        .whitelist_function("sid_to_carr_freq")
        .whitelist_function("code_string_to_enum")
        .whitelist_function("code_to_string")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
