#

## Cargo

### Workspace

Run tests for single crate:

```sh
cargo test -p crate_name
```

### documentation comments

View code docs with `cargo doc --open`.

why: demonstrate how to use your code, api.
what: generates html from documentation comments.
how:

- three slashes `///` for code that follows the comment.
- `//!` to document the overall module/crate.

### build profiles

why: set optimization levels for different build profiles.
how:

Defaults:

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```
