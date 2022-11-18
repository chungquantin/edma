use quote::quote;
use syn::{parse_quote, Data, DataStruct, DeriveInput, Fields, FieldsNamed, Stmt};

pub fn derive(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
	match &input.data {
		Data::Struct(DataStruct {
			fields: Fields::Named(FieldsNamed {
				named,
				..
			}),
			..
		}) => {
			let ident = &input.ident;

			let (m_reads, m_names) = map_properties(named);

			Ok(quote! {

				impl std::convert::TryFrom<gremlin_client::GValue> for #ident {
					type Error = gremlin_client::GremlinError;
					fn try_from(result : gremlin_client::GValue) -> gremlin_client::GremlinResult<Self>{


						match result {
							gremlin_client::GValue::Map(map) => {
								#(#m_reads)*

								Ok(#ident {
									#(#m_names),*
								})
							}
							 _ => {
								Err(gremlin_client::GremlinError::Cast(String::from("Cannot")))
							}
						}


					}
				}
			})
		}
		_ => Err(syn::Error::new_spanned(input, "Only structs are supported for Repo derive")),
	}
}

pub fn derive_map(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
	match &input.data {
		Data::Struct(DataStruct {
			fields: Fields::Named(FieldsNamed {
				named,
				..
			}),
			..
		}) => {
			let ident = &input.ident;

			let (m_reads, m_names) = map_properties(named);

			Ok(quote! {

				impl std::convert::TryFrom<gremlin_client::Map> for #ident {
					type Error = gremlin_client::GremlinError;
					fn try_from(map : gremlin_client::Map) -> gremlin_client::GremlinResult<Self>{
						#(#m_reads)*

						Ok(#ident {
							#(#m_names),*
						})
					}
				}
			})
		}
		_ => Err(syn::Error::new_spanned(input, "Only structs are supported for Repo derive")),
	}
}

fn map_properties(
	named: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
) -> (Vec<Stmt>, Vec<&Option<proc_macro2::Ident>>) {
	let reads = named
		.iter()
		.filter_map(|field| -> Option<Stmt> {
			let id = &field.ident.as_ref()?;
			let id_s = id.to_string();
			let ty = &field.ty;

			Some(parse_quote!(
				let #id: #ty = map.try_get(#id_s)?;
			))
		})
		.collect();

	let names = named.iter().map(|field| &field.ident).collect();

	(reads, names)
}
