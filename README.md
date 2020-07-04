# swiftnav-rs

A rust wrapper around the C library `libswiftnav`, which provides general GNSS
related functionality. Only limited portion of the `libswiftnav` functionality
is currently wrapped. PRs adding functionality are welcome! Ideally the wrapper
should provide an idiomatic Rust interface while delegating as much work to the
underlying C implementation.