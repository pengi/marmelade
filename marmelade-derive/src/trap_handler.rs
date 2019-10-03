
use proc_macro::{
    TokenStream
};

use syn::{
    Attribute,
    ItemImpl,
    ImplItem,
    Expr,
    Ident,
    export::Span
};

fn take_attributes(attrs: &mut Vec<Attribute>, name: &str) -> Vec<Attribute> {
    let path : syn::Path = syn::parse_str(name).unwrap();
    let mut found_attrs : Vec<Attribute> = vec![];
    for idx in (0..attrs.len()).rev() {
        if path == attrs[idx].path {
            found_attrs.push(attrs.remove(idx));
        }
    }
    found_attrs
}

#[derive(Debug)]
struct TrapHandlerRef {
    trap: Box<Expr>,
    name: Ident,
    num_args: usize
}

pub fn trap_handlers(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast: ItemImpl = syn::parse(item).unwrap();

    let name = &ast.self_ty;

    let mut methods : Vec<TrapHandlerRef> = vec![];

    for item in ast.items.iter_mut() {
        if let ImplItem::Method(method) = item {
            let trap_attributes = take_attributes(&mut method.attrs, "trap");
            for fnattr in trap_attributes {
                let trap : syn::ExprParen = syn::parse(fnattr.tts.into()).unwrap();

                // TODO: Verify (&mut self, cpu: &mut impl CPU) parameters

                methods.push(TrapHandlerRef {
                    trap: trap.expr,
                    name: method.sig.ident.clone(),
                    num_args: method.sig.decl.inputs.len()-2
                });
            }
        }
    }

    let method_impl = methods.iter().map(|method| {
        let trap = &method.trap;
        let methodname = &method.name;
        let args : Vec<Ident> = (0..method.num_args).map(
            |i| Ident::new(format!("arg_{}", i).as_str(), Span::call_site())
            ).collect();
        let args2 = args.clone();
        quote!{
            #trap => {
                #(let #args = Stackable::stack_pop(core);)*
                if let Some(result) = self.#methodname(cpu #(, #args2)*) {
                    result.stack_replace(core);
                    Some(())
                } else {
                    None
                }
            }
        }
    });

    let traphandlertrait = quote! {
        impl #name {
            fn trap_invoke(&mut self, cpu: &mut CPU, core: &mut CPUCore, ir: u16, _pc: u32) -> Option<()> {
                match ir {
                    #(#method_impl,)*
                    _ => None
                }
            }
        }
    };

    (quote! {
        #ast
        #traphandlertrait
    }).into()
}