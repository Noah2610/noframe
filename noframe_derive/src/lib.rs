extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{ self, DeriveInput };
use quote::quote;

#[proc_macro_derive(Mask)]
pub fn mask_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_mask(&ast)
}

fn impl_mask(ast: &DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;
    let gen = quote! {
        impl Mask for #struct_name {
            fn point(&self) -> &Point {
                &self.point
            }
            fn point_mut(&mut self) -> &mut Point {
                &mut self.point
            }
            fn size(&self) -> &Size {
                &self.size
            }
            fn origin(&self) -> &Origin {
                &self.origin
            }
        }
    };
    gen.into()
}
