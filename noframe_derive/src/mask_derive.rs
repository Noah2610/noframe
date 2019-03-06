use std::collections::HashMap;

use proc_macro::TokenStream;
use syn::{ self, DeriveInput, Data, DataStruct, Ident, Type };
use quote::quote;

pub fn impl_mask(ast: &DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;
    let field_point;
    let field_size;
    let field_origin;

    if let Data::Struct(data) = &ast.data {
        field_point  = ident_for(data, "Point")
            .expect(&format!("Struct {} must have a field of type Point", struct_name));
        field_size   = ident_for(data, "Size")
            .expect(&format!("Struct {} must have a field of type Size", struct_name));
        field_origin = ident_for(data, "Origin")
            .expect(&format!("Struct {} must have a field of type Origin", struct_name));
    } else {
        panic!("#[derive(Mask)] can only be used on structs");
    };

    let gen = quote! {
        impl Mask for #struct_name {
            fn point(&self) -> &Point {
                &self.#field_point
            }
            fn point_mut(&mut self) -> &mut Point {
                &mut self.#field_point
            }
            fn size(&self) -> &Size {
                &self.#field_size
            }
            fn origin(&self) -> &Origin {
                &self.#field_origin
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
