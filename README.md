# swiftnav

`swiftnav` is a crate that implements GNSS utility functions for use by
software-defined GNSS receivers or software requiring GNSS functionality. It is
intended to be as portable as possible and has limited dependencies.

`swiftnav` does not provide any functionality for communicating with Swift
Navigation receivers.  See [libsbp](https://github.com/swift-nav/libsbp) to
communicate with receivers using Swift Binary Protocol (SBP).

## Publishing a new release

1. Make sure your local `master` is up to date and the working tree is clean:
```
git checkout master
git pull
```
2. Run the checks (format, lint, and tests) to confirm everything is green:
```
just
```
3. Bump the `version` field in [`swiftnav/Cargo.toml`](./swiftnav/Cargo.toml) following [semantic versioning](https://semver.org/). Also run `cargo check` to update the `Cargo.lock` file.
4. Commit the above changes and open a PR. For example, to bump to version 0.12.1, run:
```
git checkout -b release/0.12.1
git commit -am "chore: Release swiftnav version 0.12.1"
git push -u origin release/0.12.1
```
Get the PR approved and merged.
5. After the PR is merged, check out the updated `master` and publish the `swiftnav` crate. This requires a crates.io API token with publish rights, so run `cargo login` first. You can use your Github account used for work at Swift to sync with crates.io. 
```
git checkout master && git pull
cargo publish -p swiftnav
```
6. Tag the release and push the tag using [`cargo release`](https://github.com/crate-ci/cargo-release). Run the follow commands from the root of this repository:
```
cargo release tag --execute
cargo release push --execute
```
Omit `--execute` on either command to do a dry run first.

## License

This crate is distributed under the terms of the LGPLv3, full details are
available in the [LICENSE](./LICENSE) file.
