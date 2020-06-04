use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

use build_info_common::BuildInfo;

mod eval;
use eval::{Eval, Value};
mod syntax;
mod types;

pub fn format(input: TokenStream, _build_info: BuildInfo) -> TokenStream {
	let syntax = parse_macro_input!(input as syntax::Syntax);
	let values: Vec<Value> = syntax.args.iter().map(|v| v.eval()).collect();

	let str = format!("{:#?}", values);

	let output = quote!(#str);

	// println!("{}", output.to_string());
	output.into()
}
