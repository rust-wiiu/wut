use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, ItemFn, Meta, Token};

#[proc_macro_attribute]
pub fn wut_main(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    let args = parse_macro_input!(attr with Punctuated::<Meta, Token![,]>::parse_terminated)
        .into_iter()
        .collect::<Vec<_>>();

    let func_name = &input.sig.ident;
    let block = &input.block;

    // Parse the attributes and generate the corresponding code
    let mut custom_args = Vec::new();
    for arg in args {
        if let Meta::Path(path) = arg {
            if let Some(ident) = path.get_ident() {
                custom_args.push(quote! { wut::logger::#ident });
            }
        }
    }

    let expanded = quote! {
        #[no_mangle]
        pub extern "C" fn #func_name() {
            wut::process::init(#(#custom_args)|*);
            #block
            wut::process::deinit();
        }
    };

    TokenStream::from(expanded)
}
