//! This crate provides a procedural macro for generating boilerplate code
//! for `noframe::geo::Mask`, `#[derive(Mask)]`.
// TODO
//! _(I plan to write some more procedural macros in this crate in the future.)_

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{ self, DeriveInput, Data, DataStruct, Ident, Type };
use quote::quote;

/// Use this procedural macro on structs to implement default methods depending on the struct's
/// fields. The struct must have fields of the following types, for this macro to work:
/// - `Point`
/// - `Size`
/// - `Origin`
///
/// The field names do not matter, although they must be named.
/// This macro will use the first occurence of a field of type `Point` for the Mask's point,
/// same goes for the other fields.
///
/// # Examples
/// ```
/// # extern crate noframe;
/// # use noframe_derive::Mask;
/// use noframe::geo::prelude::*;
///
/// #[derive(Mask)]
/// struct MyMask {
///     mask_point:  Point,
///     mask_size:   Size,
///     mask_origin: Origin,
/// }
///
/// let my_mask = MyMask {
///     mask_point:  Point::new(10.0, 5.0),
///     mask_size:   Size::new(40.0, 20.0),
///     mask_origin: Origin::Center,
/// };
///
/// assert_eq!(Point::new(10.0, 5.0), *my_mask.point());
/// assert_eq!(Size::new(40.0, 20.0), *my_mask.size());
/// assert_eq!(Origin::Center,        *my_mask.origin());
/// ```
#[proc_macro_derive(Mask)]
pub fn mask_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_mask(&ast)
}

fn impl_mask(ast: &DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;

    let point_ident;
    let size_ident;
    let origin_ident;

    if let Data::Struct(data) = &ast.data {
        point_ident = ident_for(data, "Point")
            .expect(&format!("Struct {} must have a field of type Point", struct_name));
        size_ident = ident_for(data, "Size")
            .expect(&format!("Struct {} must have a field of type Size", struct_name));
        origin_ident = ident_for(data, "Origin")
            .expect(&format!("Struct {} must have a field of type Origin", struct_name));
    } else {
        panic!("#[derive(Mask)] can only be used on structs");
    };

    let gen = quote! {
        impl Mask for #struct_name {
            fn point(&self) -> &Point {
                &self.#point_ident
            }
            fn point_mut(&mut self) -> &mut Point {
                &mut self.#point_ident
            }
            fn size(&self) -> &Size {
                &self.#size_ident
            }
            fn origin(&self) -> &Origin {
                &self.#origin_ident
            }
        }
    };
    gen.into()
}

fn ident_for(data: &DataStruct, type_name: &str) -> Option<Ident> {
    let type_name = type_name.to_string();
    data.fields.iter().find_map( |field| {
        if let Type::Path(ty) = &field.ty {
            if ty.path.segments[0].ident.to_string() == type_name {
                Some(field.ident.clone().expect("Struct must have named fields"))
            } else { None }
        } else { dbg!(&field.ty); None }
    })
}
