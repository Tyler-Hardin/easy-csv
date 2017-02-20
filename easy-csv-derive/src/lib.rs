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
    for (idx, field) in fields.iter().enumerate() {
        let name = field.ident.clone().expect("Fields must be named");
        header_parser.push({
            quote! {
                let mut index = None;
                let headers = try!(reader.headers());
                for (i, col) in headers.iter().enumerate() {
                    if col == stringify!(#name) {
                        index = Some(i);
                    }
                }
                let index = match index {
                    Some(index) => index,
                    None => {
                        return Err(::Error::MissingColumnError(
                            format!("Column \"{}\" not found", stringify!(#name))));
                    }
                };
                column_indices.push(index);
            }
        });


        row_parser.push({
            let ty = field.ty.clone();
            quote! {
                #name : match record[col_indices[#idx]].parse::<#ty>() {
                    Ok(v) => v,
                    Err(_) => {
                        return Some(Err(::Error::ParseError(
                            format!("Error parsing column \"{}\" on row {}",
                                stringify!(#name),
                                row))));
                    }
                }
            }
        });
    }

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    quote! {
        impl #impl_generics CSVParsable<#name> for #name #ty_generics #where_clause {
            fn parse_header<R: std::io::Read>(
                reader: &mut csv::Reader<R>) -> Result<Vec<usize>, ::Error> {
                let mut column_indices = vec![];
                #(#header_parser)*
                Ok(column_indices)
            }

            fn parse_row<R: std::io::Read>(
                records: &mut std::iter::Enumerate<csv::StringRecords<R>>,
                col_indices: &Vec<usize>) -> Option<Result<#name,::Error>> {
                let record = records.next();
                match record {
                    Some((row, record)) => {
                        match record {
                            Ok(record) => Some(Ok(#name { #(#row_parser),* })),
                            Err(e) => Some(Err(::Error::CSVError(e)))
                        }
                    }
                    None => None
                }
            }
        }
    }
}
