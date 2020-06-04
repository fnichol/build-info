#![forbid(unsafe_code)]

// Use the `build_info!` macro to generate a function `crate::build_info` that returns on object with the data that
// is collected in the build script.
// This macro supports visibility-specifiers, like `build_info!(pub fn how_this_crate_was_built)`.
build_info::build_info!(fn build_info);

fn main() {
	// Most types have a sensible default for formatting. Just printing the BuildInfo type directly is quick and easy:
	println!("{}", build_info::format2!("{}", $.timestamp));
}
