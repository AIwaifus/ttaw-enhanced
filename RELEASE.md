# Release Process

- leave the Cargo.toml package version alone :)
- trigger the `tag and publish crate [manual]` action and specify the release increment (major, minor, or patch)
  - once the action completes, the Cargo.toml will be updated and the crate will be live on crates.io
- create a release (with title, notes, thanks, e