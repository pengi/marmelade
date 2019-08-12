extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

mod serialization;

use proc_macro::TokenStream;

#[proc_macro_derive(SerialRead, attributes(length_start, length_end, align, pad))]
pub fn serial_read(input: TokenStream) -> TokenStream {
    serialization::serial_read(input)
}