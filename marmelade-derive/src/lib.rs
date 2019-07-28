extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::export::TokenStream2;

fn format_attrs(attrs: &Vec<syn::Attribute>) -> TokenStream2 {
    let mut attr_mod = quote!{};
    for attr in attrs {
        let attr_name = &attr.parse_meta().unwrap().name();
        let attr_toks = &attr.tts;

        attr_mod.extend( quote!{
            .#attr_name #attr_toks
        });
    }
    attr_mod
}

fn reader_for_field(f : &syn::Field) -> TokenStream2 {
    let attr_mod = format_attrs(&f.attrs);
    if let syn::Type::Array(arr) = &f.ty {
        let mut reader = quote!{FileReadable::read(rdr#attr_mod),};
        let len = if let syn::Expr::Lit(lit) = &arr.len {
            match &lit.lit {
                syn::Lit::Int(i) => i.value(),
                _ => unimplemented!()
            }
        } else {
            unimplemented!();
        };
        for _ in 1..len {
            reader.extend(quote!{FileReadable::read(rdr),});
        }
        quote! {[#reader]}
    } else {
        quote! {FileReadable::read(rdr#attr_mod)}
    }
}

#[proc_macro_derive(FileReadable, attributes(length_start, length_end))]
pub fn file_readable(input: TokenStream) -> TokenStream {
    let ast : syn::DeriveInput = syn::parse(input).unwrap();
    
    let name = &ast.ident;


    let read_func = match ast.data {
        syn::Data::Struct(data) => {
            match data.fields {
                fields @ syn::Fields::Named(_) => {
                    let mut ftoks = quote! {};
                    for f in fields.iter() {
                        let fid = &f.ident;
                        let reader = reader_for_field(f);
                        ftoks.extend(quote! {
                            #fid: #reader,
                        });
                    }
                    quote! {
                        #name {
                            #ftoks
                        }
                    }
                },
                fields @ syn::Fields::Unnamed(_) => {
                    let mut ftoks = quote! {};
                    for f in fields.iter() {
                        let reader = reader_for_field(f);
                        ftoks.extend(quote! { #reader, });
                    }
                    quote! {
                        #name (
                            #ftoks
                        )
                    }
                },
                syn::Fields::Unit => quote!{#name}
            }
        },
        _ => unimplemented!()
    };

    let gen = quote! {
        impl FileReadable for #name {
            fn read( rdr : &mut FileReader ) -> #name {
                #read_func
            }
        }
    };
    // println!("{}", gen);
    // println!("");
    // println!("");
    gen.into()
}