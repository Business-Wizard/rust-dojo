#

## Tests

### private/public

private/unit tests: create a test module in the same file as the code.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

public api tests: create a test module in a separate file.

### test code utils

tests
└── common
    └── mod.rs

then in the test modules needing, use the common module.

```rust
mod common;
```

### Running tests

Run tests for a single module file:

```sh
cargo test --test test_file_name
```

Run tests for single crate:

```sh
cargo test -p crate_name
```

## Cargo

### Workspace

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

## References; Mutation

Interior mutability: a design pattern in Rust that allows you to mutate data even when there are shared references to that data.

Two typical categories:
get mutable ref via a shared ref: Mutex, RefCell
mutate a value via a shared ref: UnsafeCell
