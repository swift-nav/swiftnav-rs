# swiftnav-rs

`swiftnav-rs` is a crate that implements GNSS utility functions for use by
software-defined GNSS receivers or software requiring GNSS functionality. It is
intended to be as portable as possible and has limited dependencies.

`swiftnav-rs` does not provide any functionality for communicating with Swift
Navigation receivers.  See [libsbp](https://github.com/swift-nav/libsbp) to
communicate with receivers using Swift Binary Protocol (SBP).

## Publishing a new release

Use the [`cargo-release`](https://github.com/sunng87/cargo-release) tool:

```
cargo release --skip-publish
```

## License
This crate is distributed under the terms of the LGPLv3, full details are
available in the [LICENSE](./LICENSE) file.
