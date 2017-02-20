#![recursion_limit="500"]
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(CSVParsable)]
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
        syn::Body::Enum(_) => panic!("#[derive(CSVParsable)] can only be used with structs"),
    };

	let mut header_parser = Vec::new();
	let mut row_parser = Vec::new();
    let mut idx : usize = 0;
	for i in fields {
        header_parser.push({
            let name = i.ident.clone().expect("Fields must be named");
            quote! {
                let mut index = None;
                for (i, col) in reader.headers().unwrap().iter().enumerate() {
                    if col == stringify!(#name) {
                        index = Some(i);
                    }
                }
                let index = index.expect(format!("Column \"{}\" not found", "").as_str());
                column_indices.push(index);
            }
        });


		row_parser.push({
            let name = i.ident.clone().expect("Fields must be named");
            let ty = i.ty.clone();
            quote! {
                #name : record[col_indices[#idx]].parse::<#ty>().unwrap()
            }
        });
        idx += 1;
	}

	let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    quote! {
        impl #impl_generics CSVParsable<#name> for #name #ty_generics #where_clause {
			fn parse_header<R: std::io::Read>(reader: &mut csv::Reader<R>) -> Vec<usize> {
                let mut column_indices = vec![];
                #(#header_parser)*
                column_indices
			}

            fn parse_row<R: std::io::Read>(
                records: &mut csv::StringRecords<R>, 
                col_indices: &Vec<usize>) -> Option<#name> {
                let record = records.next();
                match record {
                    Some(record) => {
                        let record = record.unwrap();
                        Some(#name { #(#row_parser),* })
                    }
                    None => None
                }
            }
        }
    }
}
