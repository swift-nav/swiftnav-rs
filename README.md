# swiftnav

`swiftnav` is a crate that implements GNSS utility functions for use by
software-defined GNSS receivers or software requiring GNSS functionality. It is
intended to be as portable as possible and has limited dependencies.

`swiftnav` does not provide any functionality for communicating with Swift
Navigation receivers.  See [libsbp](https://github.com/swift-nav/libsbp) to
communicate with receivers using Swift Binary Protocol (SBP).

# swiftnav-sys

`swiftnav-sys` is a crate which builds and exposes Rust FFI bindings for the
`libswiftnav` C library.

# Publishing a new release

Releases are done against the master branch.  Use the
[`cargo-release`](https://github.com/sunng87/cargo-release) tool.  First
release the `swiftnav-sys` crate:

```
cd swiftnav-sys
cargo release <major|minor|patch>

# If things look good
cargo release <major|minor|patch> --execute
```

Then release the `swiftnav` crate:

```
cd swiftnav
cargo release <major|minor|patch>

# If things look good
cargo release <major|minor|patch> --execute
```

# License
This crate is distributed under the terms of the LGPLv3, full details are
available in the [LICENSE](./LICENSE) file.
