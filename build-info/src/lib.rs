/*!
Begin by adding `build-info` as a `[dependency]` and `build-info-build` as a `[build-dependency]` to your [`Cargo.toml`](https://github.com/danielschemmel/build-info/tree/master/sample/Cargo.toml).
Please make sure that both dependencies use the same version!

If it does not already exist, add a [`build.rs`](https://github.com/danielschemmel/build-info/tree/master/sample/build.rs) to your project's root, where you call `build_info_build::build_script()`.
This will collect build information at compile time.

Then, either use the `build_info!` macro to add a function that returns version information at runtime:
```rust,ignore
build_info::build_info!(fn version);
```
or use `build_info::format!` to generate a string at compile time:
```rust,ignore
// sample output: "{sample v0.0.13 built with rustc 1.45.0-nightly (4bd32c980 2020-05-29) at 2020-05-30 11:22:46Z}"
build_info::format!("{{{.crate_info.name} v{.crate_info.version} built with {.compiler} at {.timestamp}}}")
```

You can also check out the [sample](https://github.com/danielschemmel/build-info/tree/master/sample/) project that shows both variants.

# Features
The ´build-info` crate has the following features:

- `runtime` (enabled by default): Enables the use of `build_info::build_info!` and provides the prerequisite types.
- `serde` (disabled by default): Adds serde support to the objects returned by `build_info::build_info!`. Implies the `runtime` feature.
- `nested` (disabled by default): Enables [`proc-macro-nested`](https://crates.io/crates/proc-macro-nested) for `build_info::format!`, which lets it be nested inside another proc-macro invocation in some cases.
*/

#![forbid(unsafe_code)]

use proc_macro_hack::proc_macro_hack;

#[cfg(feature = "runtime")]
pub use build_info_common::{
	BuildInfo, CompilerChannel, CompilerInfo, CrateInfo, DateTime, GitInfo, Utc, Version, VersionControl,
};

/// Used internally by the function that is generated by `build_info::build_info!`
#[doc(hidden)]
#[cfg(feature = "runtime")]
pub use build_info_common::nanos_to_utc;

/// Used internally by the function that is generated by `build_info::build_info!`
#[doc(hidden)]
#[cfg(feature = "runtime")]
pub use lazy_static::lazy_static;

#[cfg(feature = "runtime")]
pub use build_info_proc::build_info;

/**
Generates a string at compile-time that includes build information.

This function-like macro takes a single string-literal as its argument, on which it performs string interpolation with the current build information.
To do so, pass the path to a member inside curly braces, as if indexing an object of type `BuildInfo`. For example, `build_info::format!("Built at {.timestamp}")` might return `"Built at 2020-05-28 20:09:40.379213639Z".`

Use `?` to unwrap `Option`s, the results are stringified automatically.

Literal curly braces can be printed by doubling them up: `build_info::format!("{{}}") // yields "{}"`.
*/
#[cfg_attr(not(feature = "nested"), proc_macro_hack)]
#[cfg_attr(feature = "nested", proc_macro_hack(support_nested))]
pub use build_info_proc::format;
#[cfg_attr(not(feature = "nested"), proc_macro_hack)]
#[cfg_attr(feature = "nested", proc_macro_hack(support_nested))]
pub use build_info_proc::format2;
