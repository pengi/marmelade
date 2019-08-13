extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

mod serialization;
mod trap_handler;

use proc_macro::TokenStream;

#[proc_macro_derive(SerialRead, attributes(length_start, length_end, align, pad))]
pub fn serial_read(input: TokenStream) -> TokenStream {
    serialization::serial_read(input)
}


#[proc_macro_attribute]
pub fn trap_handlers(attr: TokenStream, item: TokenStream) -> TokenStream {
    trap_handler::trap_handlers(attr, item)
}