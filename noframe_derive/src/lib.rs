//! This crate provides a procedural macro for generating boilerplate code
//! for `noframe::geo::Mask`, `#[derive(Mask)]`.
// TODO
//! _(I plan to write some more procedural macros in this crate in the future.)_

extern crate proc_macro;

mod mask_derive;

use proc_macro::TokenStream;

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
    mask_derive::impl_mask(&ast)
}
