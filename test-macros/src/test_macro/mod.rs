mod attributes;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::ItemFn;

pub fn handle(attr: TokenStream, item: ItemFn) -> Result<TokenStream, String> {
    let attributes = attributes::parse(attr)?;
    let setup = attributes.setup;
    let teardown = attributes.teardown;
    let setup = Ident::new(&setup, Span::call_site());
    let teardown = Ident::new(&teardown, Span::call_site());
    let fn_name = &item.sig.ident;

    Ok(quote! {
        #[test]
        fn #fn_name() {
            #item
            testing_utils::tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    testing_utils::test(
                        #setup,
                        #teardown,
                        #fn_name,
                    ).await;
                })
        }
    }
    .into())
}
