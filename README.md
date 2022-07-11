# codegen-rs

Provides an builder API to assist in generating Rust code.

This is a fork of the [original codegen crate][origin].

[origin]: https://github.com/carllerche/codegen

## Installation

To use `codegen-rs`, add this to your `Cargo.toml`:

```toml
[dependencies]
codegen_rs = { git = "https://github.com/sayanarijit/codegen-rs" }
```

## Usage

1. Create a `Scope` instance.
2. Use the builder API to add elements to the scope.
3. Call `Scope::to_string()` to get the generated code.

For example:

```rust
use codegen_rs::Scope;

let mut scope = Scope::new();

let struct_ = scope.new_struct("Foo").derive("Debug");
struct_.field("one", "usize");
struct_.field("two", "String");

println!("{}", scope.to_string());
```

## License

This project is licensed under the [MIT license](LICENSE).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `codegen` by you, shall be licensed as MIT, without any
additional terms or conditions.
