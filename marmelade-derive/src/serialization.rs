
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
        let mut reader = quote!{SerialRead::read(rdr#attr_mod)?,};
        let len = if let syn::Expr::Lit(lit) = &arr.len {
            match &lit.lit {
                syn::Lit::Int(i) => i.value(),
                _ => unimplemented!()
            }
        } else {
            unimplemented!();
        };
        for _ in 1..len {
            reader.extend(quote!{SerialRead::read(rdr)?,});
        }
        quote! {[#reader]}
    } else {
        quote! {SerialRead::read(rdr#attr_mod)?}
    }
}

pub fn serial_read(input: TokenStream) -> TokenStream {
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
                        Ok(#name {
                            #ftoks
                        })
                    }
                },
                fields @ syn::Fields::Unnamed(_) => {
                    let mut ftoks = quote! {};
                    for f in fields.iter() {
                        let reader = reader_for_field(f);
                        ftoks.extend(quote! { #reader, });
                    }
                    quote! {
                        Ok(#name (
                            #ftoks
                        ))
                    }
                },
                syn::Fields::Unit => quote!{Ok(#name)}
            }
        },
        _ => unimplemented!()
    };

    let gen = quote! {
        impl SerialRead for #name {
            fn read( rdr : &mut SerialReadStorage ) -> std::io::Result<#name> {
                #read_func
            }
        }
    };
    // println!("{}", gen);
    // println!("");
    // println!("");
    gen.into()
}