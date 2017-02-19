#[allow(non_snake_case)]
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::Ident;

#[proc_macro_derive(EasyCSV)]
pub fn easy_csv(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_macro_input(&s).unwrap();

    // Build the impl
    let gen = impl_easy_csv(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_easy_csv(ast: &syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;
    let fields = match ast.body {
        syn::Body::Struct(ref data) => data.fields(),
        syn::Body::Enum(_) => panic!("#[derive(EasyCSV)] can only be used with structs"),
    };

	let mut header_parser = Vec::new();
	let mut row_parser = Vec::new();
	for i in fields {
		header_parser.push({
			let name = i.ident.clone().expect("Fields must be named");
			let index_token = Ident::from(String::from(name.as_ref()) + "_index");
			quote! {
				let mut #index_token = None;
				for (i, col) in reader.headers().unwrap().iter().enumerate() {
					if col == stringify!(#name) {
						#index_token = Some(i);
					}
				}
				let #index_token = #index_token.expect(format!("Column \"{}\" not found", stringify!(#name)).as_str());
			}
		});

		row_parser.push({
            let name = i.ident.clone().expect("Fields must be named");
            let index_token = Ident::from(String::from(name.as_ref()) + "_index");
            let ty = i.ty.clone();
            quote! {
                #name : row[#index_token].parse::<#ty>().unwrap()
            }
        });

	}

	let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    quote! {
        impl #impl_generics ::EasyCSV<#name> for #name #ty_generics #where_clause {
            fn parse_csv<R : std::io::Read>(reader : &mut csv::Reader<R>) -> Vec<#name> {
				#(#header_parser;)*
				reader.records().map(|r| r.unwrap()).map(move |row| {
    				#name {
						#(#row_parser),*
					}
				}).collect()
            }
        }
    }
}

