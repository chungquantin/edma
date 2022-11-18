use proc_macro::TokenStream;

mod value;

#[proc_macro_derive(FromGValue)]
pub fn derive_from_gvalue(input: TokenStream) -> TokenStream {
	let input = syn::parse_macro_input!(input as syn::DeriveInput);

	match value::derive(&input) {
		Ok(ts) => ts.into(),
		Err(e) => e.to_compile_error().into(),
	}
}

#[proc_macro_derive(FromGMap)]
pub fn derive_from_gmap(input: TokenStream) -> TokenStream {
	let input = syn::parse_macro_input!(input as syn::DeriveInput);

	match value::derive_map(&input) {
		Ok(ts) => ts.into(),
		Err(e) => e.to_compile_error().into(),
	}
}
