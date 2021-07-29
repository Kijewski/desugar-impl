// Copyright (c) 2021 René Kijewski <rene.[SURNAME]@fu-berlin.de>
// All rights reserved.
//
// This software and the accompanying materials are made available under
// the terms of the ISC License which is available in the project root as LICENSE-ISC, AND/OR
// the terms of the MIT License which is available in the project root as LICENSE-MIT, AND/OR
// the terms of the Apache License, Version 2.0 which is available in the project root as LICENSE-APACHE.
//
// You have to accept AT LEAST one of the aforementioned licenses to use, copy, modify, and/or distribute this software.
// At your will you may redistribute the software under the terms of only one, two, or all three of the aforementioned licenses.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! ## `impl Trait` not allowed outside of function and method return types
//!
//! **… but it is now!**
//!
//! This library gives you one macro, and one macro only: [`#[desugar_impl]`][macro@desugar_impl].
//!
//! Annotate any struct, enum, or union with [`#[desugar_impl]`][macro@desugar_impl]
//! to allow the use of `field_name: impl SomeTrait` in their declaration. E.g.
//!
//! ```
//! #[desugar_impl::desugar_impl]
//! struct Test {
//!     a: impl Clone + PartialOrd,
//!     b: impl Clone + PartialOrd,
//!     c: impl Copy,
//! }
//! ```
//!
//! desugars to
//!
//! ```
//! struct Test<Ty1, Ty2, Ty3>
//! where
//!     Ty1: Clone + PartialOrd,
//!     Ty2: Clone + PartialOrd,
//!     Ty3: Copy,
//! {
//!     a: Ty1,
//!     b: Ty2,
//!     c: Ty3,
//! }
//! ```
//!
//! You can still place any `#[derive(…)]` macros just below `#[desugar_impl]`, and they'll see
//! the desugared code.

use std::iter::FromIterator;

use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::{Colon, Where};
use syn::{
    parse_macro_input, Data, DataEnum, DataStruct, DataUnion, DeriveInput, Field, Fields,
    FieldsNamed, FieldsUnnamed, GenericParam, Ident, Path, PathArguments, PathSegment,
    PredicateType, Type, TypeImplTrait, TypeParam, TypePath, WhereClause, WherePredicate,
};

/// Desugar `impl Trait` fields in a struct, enum, or union declaration.
///
/// Please see the library documentation for an explanation: [desugar_impl](index.html).
#[proc_macro_attribute]
pub fn desugar_impl(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as DeriveInput);
    let mut ty_index = 1;

    let ast_generics = &mut ast.generics;
    let ast_data = &mut ast.data;

    let mut convert_fields = |fields: &mut Punctuated<_, _>| {
        for Field { ty, .. } in fields {
            if let Type::ImplTrait(TypeImplTrait { bounds, .. }) = ty {
                let type_ident = format!("Ty{}", ty_index);
                ty_index += 1;
                let type_ident = Ident::new(&type_ident, Span::call_site().into());
                let type_path = Type::Path(TypePath {
                    qself: None,
                    path: Path {
                        leading_colon: None,
                        segments: Punctuated::from_iter([PathSegment {
                            ident: type_ident.clone(),
                            arguments: PathArguments::None,
                        }]),
                    },
                });

                let predicate = WherePredicate::Type(PredicateType {
                    lifetimes: None,
                    bounded_ty: type_path.clone(),
                    colon_token: Colon::default(),
                    bounds: bounds.clone(),
                });
                match &mut ast_generics.where_clause {
                    Some(where_clause) => {
                        where_clause.predicates.push(predicate);
                    }
                    where_clause @ None => {
                        *where_clause = Some(WhereClause {
                            where_token: Where::default(),
                            predicates: Punctuated::from_iter([predicate]),
                        });
                    }
                }

                ast_generics.params.push(GenericParam::Type(TypeParam {
                    attrs: Vec::new(),
                    ident: type_ident,
                    colon_token: None,
                    bounds: Default::default(),
                    eq_token: None,
                    default: None,
                }));

                *ty = type_path;
            }
        }
    };

    let mut convert_some_fields = |fields: &mut Fields| match fields {
        Fields::Named(FieldsNamed { named: fields, .. })
        | Fields::Unnamed(FieldsUnnamed {
            unnamed: fields, ..
        }) => {
            convert_fields(fields);
        }
        Fields::Unit => {}
    };

    match ast_data {
        Data::Struct(DataStruct { fields, .. }) => {
            convert_some_fields(fields);
        }
        Data::Union(DataUnion {
            fields: FieldsNamed { named: fields, .. },
            ..
        }) => {
            convert_fields(fields);
        }
        Data::Enum(DataEnum { variants, .. }) => {
            for variant in variants {
                convert_some_fields(&mut variant.fields);
            }
        }
    }

    TokenStream::from(quote! { #ast })
}
