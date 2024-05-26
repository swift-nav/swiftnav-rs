extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let mut cmake = cmake::Config::new("src/libswiftnav/");
    let out_dir = env::var("OUT_DIR").unwrap();

    if !cfg!(feature = "testcpp") {
        cmake
            .define("libswiftnav_ENABLE_TESTS", "OFF")
            .define("libswiftnav_ENABLE_TEST_LIBS", "OFF");
    }

    cmake
        .define("libswiftnav_ENABLE_CLANG_FORMAT", "OFF")
        .define("libswiftnav_ENABLE_CLANG_TIDY", "OFF")
        .define("CMAKE_INSTALL_PREFIX", out_dir);

    let dst = cmake.build();

    println!("cargo:rustc-link-search=native={}/lib/", dst.display());
    println!("cargo:rustc-link-search=native={}/lib64/", dst.display());
    println!("cargo:rustc-link-lib=static=swiftnav");

    let include_args = vec![
        "-isystem".to_string(),
        format!("{}/include/", dst.display()),
    ]
    .into_iter();
    let bindings = bindgen::Builder::default()
        .array_pointers_in_arguments(true)
        .clang_args(include_args)
        .derive_hash(true)
        .derive_partialord(true)
        .derive_ord(true)
        .derive_partialeq(true)
        .derive_eq(true)
        // The input header we would like to generate
        // bindings for.
        .header(format!("{}/include/swiftnav/signal.h", dst.display()))
        .header(format!("{}/include/swiftnav/gnss_time.h", dst.display()))
        .header(format!("{}/include/swiftnav/coord_system.h", dst.display()))
        .header(format!("{}/include/swiftnav/ionosphere.h", dst.display()))
        .header(format!("{}/include/swiftnav/geoid_model.h", dst.display()))
        .header(format!("{}/include/swiftnav/troposphere.h", dst.display()))
        .header(format!("{}/include/swiftnav/ephemeris.h", dst.display()))
        .header(format!("{}/include/swiftnav/edc.h", dst.display()))
        .header(format!("{}/include/swiftnav/nav_meas.h", dst.display()))
        .header(format!(
            "{}/include/swiftnav/single_epoch_solver.h",
            dst.display()
        ))
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .blocklist_type("u8")
        .blocklist_type("u16")
        .blocklist_type("u32")
        .allowlist_type("gps_time_t")
        .allowlist_type("glo_time_t")
        .allowlist_type("utc_params_t")
        .allowlist_type("utc_tm")
        .allowlist_function("gpsdifftime")
        .allowlist_function("gps_time_valid")
        .allowlist_function("add_secs")
        .allowlist_function("decode_utc_parameters")
        .allowlist_function("gps2utc")
        .allowlist_function("utc2gps")
        .allowlist_function("date2mjd")
        .allowlist_function("mjd2utc")
        .allowlist_function("utc2mjd")
        .allowlist_function("date2utc")
        .allowlist_function("get_gps_utc_offset")
        .allowlist_function("is_leap_second_event")
        .allowlist_function("round_to_epoch")
        .allowlist_function("floor_to_epoch")
        .allowlist_function("glo2gps")
        .allowlist_function("gps2glo")
        .allowlist_function("mjd2gps")
        .allowlist_var("FLOAT_EQUALITY_EPS")
        .allowlist_var("MINUTE_SECS")
        .allowlist_var("HOUR_SECS")
        .allowlist_var("DAY_SECS")
        .allowlist_var("WEEK_SECS")
        .allowlist_var("BDS_WEEK_TO_GPS_WEEK")
        .allowlist_var("BDS_SECOND_TO_GPS_SECOND")
        .allowlist_var("GAL_WEEK_TO_GPS_WEEK")
        .allowlist_var("GLO_EPOCH_WN")
        .allowlist_var("GLO_EPOCH_TOW")
        .allowlist_var("SID_STR_LEN_MAX")
        .allowlist_type("constellation_t")
        .allowlist_type("code_t")
        .allowlist_type("gnss_signal_t")
        .allowlist_function("is_gps")
        .allowlist_function("is_sbas")
        .allowlist_function("is_glo")
        .allowlist_function("is_bds2")
        .allowlist_function("is_gal")
        .allowlist_function("is_qzss")
        .allowlist_function("sid_to_constellation")
        .allowlist_function("sid_valid")
        .allowlist_function("code_to_constellation")
        .allowlist_function("constellation_to_sat_count")
        .allowlist_function("constellation_to_string")
        .allowlist_function("constellation_string_to_enum")
        .allowlist_function("code_to_sig_count")
        .allowlist_function("code_to_chip_count")
        .allowlist_function("code_to_chip_rate")
        .allowlist_function("sid_to_carr_freq")
        .allowlist_function("code_string_to_enum")
        .allowlist_function("code_to_string")
        .allowlist_function("sid_to_string")
        .allowlist_var("NUM_SATS_GPS")
        .allowlist_var("NUM_SATS_SBAS")
        .allowlist_var("NUM_SATS_GLO")
        .allowlist_var("NUM_SATS_BDS")
        .allowlist_var("NUM_SATS_GAL")
        .allowlist_var("NUM_SATS_QZS")
        .allowlist_var("GPS_FIRST_PRN")
        .allowlist_var("SBAS_FIRST_PRN")
        .allowlist_var("GLO_FIRST_PRN")
        .allowlist_var("BDS_FIRST_PRN")
        .allowlist_var("GAL_FIRST_PRN")
        .allowlist_var("QZS_FIRST_PRN")
        .allowlist_function("llhrad2deg")
        .allowlist_function("llhdeg2rad")
        .allowlist_function("wgsllh2ecef")
        .allowlist_function("wgsecef2llh")
        .allowlist_function("wgsecef2azel")
        .allowlist_function("wgsecef2ned")
        .allowlist_function("wgsned2ecef")
        .allowlist_function("wgsecef2azel")
        .allowlist_type("ionosphere_t")
        .allowlist_function("calc_ionosphere")
        .allowlist_function("decode_iono_parameters")
        .allowlist_function("calc_troposphere")
        .allowlist_type("ephemeris_t")
        .allowlist_function("calc_sat_state")
        .allowlist_function("calc_sat_az_el")
        .allowlist_function("calc_sat_doppler")
        .allowlist_function("get_ephemeris_status_t")
        .allowlist_function("ephemeris_valid_detailed")
        .allowlist_function("ephemeris_valid")
        .allowlist_function("ephemeris_equal")
        .allowlist_function("ephemeris_healthy")
        .allowlist_function("get_ephemeris_iod_or_iodcrc")
        .allowlist_function("decode_ephemeris")
        .allowlist_function("decode_bds_d1_ephemeris")
        .allowlist_function("decode_gal_ephemeris")
        .allowlist_function("crc24q")
        .allowlist_type("measurement_std_t")
        .allowlist_function("nav_meas_flags_valid")
        .allowlist_function("pseudorange_valid")
        .allowlist_function("encode_lock_time")
        .allowlist_function("decode_lock_time")
        .allowlist_var("NAV_MEAS_FLAG_CODE_VALID")
        .allowlist_var("NAV_MEAS_FLAG_MEAS_DOPPLER_VALID")
        .allowlist_var("NAV_MEAS_FLAG_CN0_VALID")
        .allowlist_type("obs_mask_config_t")
        .allowlist_function("sid_set_init")
        .allowlist_function("sid_set_get_sat_count")
        .allowlist_function("sid_set_get_sig_count")
        .allowlist_function("sid_set_contains")
        .allowlist_function("calc_PVT")
        .allowlist_var("pvt_err_msg")
        .allowlist_type("geoid_model_t")
        .allowlist_function("get_geoid_offset")
        .allowlist_function("get_geoid_model")
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
