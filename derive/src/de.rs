use crate::{attr, bound};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    Data, DataEnum, DataStruct, DeriveInput, Error, Fields, FieldsNamed, Ident, Result, parse_quote,
};

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

pub fn derive_struct(input: &DeriveInput, fields: &FieldsNamed) -> Result<TokenStream> {
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let dummy = Ident::new(
        &format!("_IMPL_MINIDESERIALIZE_FOR_{}", ident),
        Span::call_site(),
    );

    let fieldname = fields.named.iter().map(|f| &f.ident).collect::<Vec<_>>();
    let fieldty = fields.named.iter().map(|f| &f.ty);
    let fieldstr = fields
        .named
        .iter()
        .map(attr::name_of_field)
        .collect::<Result<Vec<_>>>()?;

    let wrapper_generics = bound::with_lifetime_bound(&input.generics, "'__a");
    let (wrapper_impl_generics, wrapper_ty_generics, _) = wrapper_generics.split_for_impl();
    let bound = parse_quote!(qser::Deserialize);
    let bounded_where_clause = bound::where_clause_with_bound(&input.generics, bound);

    Ok(quote! {
        #[allow(non_upper_case_globals)]
        const #dummy: () = {
            #[repr(C)]
            struct __Visitor #impl_generics #where_clause {
                __out: qser::export::Option<#ident #ty_generics>,
            }

            impl #impl_generics qser::Deserialize for #ident #ty_generics #bounded_where_clause {
                fn begin(__out: &mut qser::export::Option<Self>) -> &mut dyn qser::de::Visitor {
                    unsafe {
                        &mut *{
                            __out
                            as *mut qser::export::Option<Self>
                            as *mut __Visitor #ty_generics
                        }
                    }
                }
            }

            impl #impl_generics qser::de::Visitor for __Visitor #ty_generics #bounded_where_clause {
                fn map(&mut self) -> qser::Result<qser::export::Box<dyn qser::de::Map + '_>> {
                    Ok(qser::export::Box::new(__State {
                        #(
                            #fieldname: qser::Deserialize::default(),
                        )*
                        __out: &mut self.__out,
                    }))
                }
            }

            struct __State #wrapper_impl_generics #where_clause {
                #(
                    #fieldname: qser::export::Option<#fieldty>,
                )*
                __out: &'__a mut qser::export::Option<#ident #ty_generics>,
            }

            impl #wrapper_impl_generics qser::de::Map for __State #wrapper_ty_generics #bounded_where_clause {
                fn key(&mut self, __k: &qser::export::str) -> qser::Result<&mut dyn qser::de::Visitor> {
                    match __k {
                        #(
                            #fieldstr => qser::export::Ok(qser::Deserialize::begin(&mut self.#fieldname)),
                        )*
                        _ => qser::export::Ok(qser::de::Visitor::ignore()),
                    }
                }

                fn finish(&mut self) -> qser::Result<()> {
                    #(
                        let #fieldname = self.#fieldname.take().ok_or(qser::Error)?;
                    )*
                    *self.__out = qser::export::Some(#ident {
                        #(
                            #fieldname,
                        )*
                    });
                    qser::export::Ok(())
                }
            }
        };
    })
}

pub fn derive_enum(input: &DeriveInput, enumeration: &DataEnum) -> Result<TokenStream> {
    if input.generics.lt_token.is_some() || input.generics.where_clause.is_some() {
        return Err(Error::new(
            Span::call_site(),
            "Enums with generics are not supported",
        ));
    }

    let ident = &input.ident;
    let dummy = Ident::new(
        &format!("_IMPL_MINIDESERIALIZE_FOR_{}", ident),
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
        .map(attr::name_of_variant)
        .collect::<Result<Vec<_>>>()?;

    Ok(quote! {
        #[allow(non_upper_case_globals)]
        const #dummy: () = {
            #[repr(C)]
            struct __Visitor {
                __out: qser::export::Option<#ident>,
            }

            impl qser::Deserialize for #ident {
                fn begin(__out: &mut qser::export::Option<Self>) -> &mut dyn qser::de::Visitor {
                    unsafe {
                        &mut *{
                            __out
                            as *mut qser::export::Option<Self>
                            as *mut __Visitor
                        }
                    }
                }
            }

            impl qser::de::Visitor for __Visitor {
                fn string(&mut self, s: &qser::export::str) -> qser::Result<()> {
                    let value = match s {
                        #( #names => #ident::#var_idents, )*
                        _ => { return qser::export::Err(qser::Error) },
                    };
                    self.__out = qser::export::Some(value);
                    qser::export::Ok(())
                }
            }
        };
    })
}
