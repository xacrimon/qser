use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    Data, DataEnum, DataStruct, DeriveInput, Error, Fields, FieldsNamed, Ident, Result, parse_quote,
};

use crate::{attr, bound};

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
    match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => derive_struct(&input, fields),
        Data::Enum(enumeration) => derive_enum(&input, enumeration),
        _ => Err(Error::new(
            Span::call_site(),
            "currently only structs with named fields are supported",
        )),
    }
}

fn derive_struct(input: &DeriveInput, fields: &FieldsNamed) -> Result<TokenStream> {
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let dummy = Ident::new(
        &format!("_IMPL_MINISERIALIZE_FOR_{}", ident),
        Span::call_site(),
    );

    let fieldname = &fields.named.iter().map(|f| &f.ident).collect::<Vec<_>>();
    let fieldstr = fields
        .named
        .iter()
        .map(|field| {
            let opts = attr::attr_field_opts(field)?;
            let name = attr::name_of_field(field, &opts);
            Ok(name)
        })
        .collect::<Result<Vec<_>>>()?;
    let index = 0usize..;

    let wrapper_generics = bound::with_lifetime_bound(&input.generics, "'__a");
    let (wrapper_impl_generics, wrapper_ty_generics, _) = wrapper_generics.split_for_impl();
    let bound = parse_quote!(qser::Serialize);
    let bounded_where_clause = bound::where_clause_with_bound(&input.generics, bound);

    Ok(quote! {
        #[allow(non_upper_case_globals)]
        #[allow(non_local_definitions)]
        const #dummy: () = {
            impl #impl_generics qser::Serialize for #ident #ty_generics #bounded_where_clause {
                fn begin(&self) -> qser::ser::Fragment {
                    qser::ser::Fragment::Map(std::boxed::Box::new(__Map {
                        data: self,
                        state: 0,
                    }))
                }
            }

            struct __Map #wrapper_impl_generics #where_clause {
                data: &'__a #ident #ty_generics,
                state: usize,
            }

            impl #wrapper_impl_generics qser::ser::Map for __Map #wrapper_ty_generics #bounded_where_clause {
                fn next(&mut self) -> std::option::Option<(std::borrow::Cow<str>, &dyn qser::Serialize)> {
                    let __state = self.state;
                    self.state = __state + 1;
                    match __state {
                        #(
                            #index => std::option::Option::Some((
                                std::borrow::Cow::Borrowed(#fieldstr),
                                &self.data.#fieldname,
                            )),
                        )*
                        _ => std::option::Option::None,
                    }
                }
            }
        };
    })
}

fn derive_enum(input: &DeriveInput, enumeration: &DataEnum) -> Result<TokenStream> {
    if input.generics.lt_token.is_some() || input.generics.where_clause.is_some() {
        return Err(Error::new(
            Span::call_site(),
            "Enums with generics are not supported",
        ));
    }

    let ident = &input.ident;
    let dummy = Ident::new(
        &format!("_IMPL_MINISERIALIZE_FOR_{}", ident),
        Span::call_site(),
    );

    let var_idents = enumeration
        .variants
        .iter()
        .map(|variant| match variant.fields {
            Fields::Unit => Ok(&variant.ident),
            _ => Err(Error::new_spanned(
                variant,
                "Invalid variant: only simple enum variants without fields are supported",
            )),
        })
        .collect::<Result<Vec<_>>>()?;
    let names = enumeration
        .variants
        .iter()
        .map(|variant| {
            let opts = attr::attr_variant_opts(variant)?;
            let name = attr::name_of_variant(variant, &opts);
            Ok(name)
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(quote! {
        #[allow(non_upper_case_globals)]
        const #dummy: () = {
            impl qser::Serialize for #ident {
                fn begin(&self) -> qser::ser::Fragment {
                    match self {
                        #(
                            #ident::#var_idents => {
                                qser::ser::Fragment::Str(std::borrow::Cow::Borrowed(#names))
                            }
                        )*
                    }
                }
            }
        };
    })
}
