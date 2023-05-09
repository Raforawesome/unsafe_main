use proc_macro::TokenStream;
use syn::ItemFn;

#[proc_macro_attribute]
pub fn unsafe_main(attr: TokenStream, item: TokenStream) -> TokenStream {
    if !attr.is_empty() {
        panic!("Expected no input to procedural macro 'unsafe_main'")
    }

    let ast: ItemFn = syn::parse(item).unwrap();
    impl_umain(&ast)
}

fn impl_umain(ast: &ItemFn) -> TokenStream {
    let body = &ast.block;
    quote::quote! {
        pub fn main() {
            unsafe { unsafe_main_helper_function() }
        }
        unsafe fn unsafe_main_helper_function() #body
    }
    .into()
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
