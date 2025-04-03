use proc_macro2::Span;
use syn::meta::ParseNestedMeta;
use syn::parse::Parse;
use syn::{Attribute, Error, Field, ItemEnum, ItemStruct, Lit, LitStr, Meta, Result, Variant};

use crate::opts::{ContainerOpts, FieldOpts, Modifier, OptionSet, VariantOpts};

fn parse_modifier(meta: ParseNestedMeta) -> Result<Modifier> {
    if meta.path.is_ident("rename") {
        let value = meta.value()?;
        let name: LitStr = value.parse()?;

        return Ok(Modifier::Rename {
            serialize_name: Some(name.value()),
            deserialize_name: Some(name.value()),
        });
    }

    if meta.path.is_ident("rename_all") {
        return Ok(Modifier::RenameAll {
            serialize_case: None,
            deserialize_case: None,
        });
    }

    if meta.path.is_ident("rename_all_fields") {
        return Ok(Modifier::RenameAllFields {
            serialize_case: None,
            deserialize_case: None,
        });
    }

    if meta.path.is_ident("deny_unknown_fields") {
        return Ok(Modifier::DenyUnknownFields);
    }

    if meta.path.is_ident("tag") {
        //return Ok(Modifier::);
        todo!()
    }

    if meta.path.is_ident("untagged") {
        return Ok(Modifier::Untagged);
    }

    if meta.path.is_ident("bound") {
        return Ok(Modifier::Bound {
            serialize: None,
            deserialize: None,
        });
    }

    if meta.path.is_ident("default") {
        return Ok(Modifier::Default { item: None });
    }

    if meta.path.is_ident("remote") {
        return Ok(Modifier::Remote {
            item: String::new(),
        });
    }

    if meta.path.is_ident("transparent") {
        return Ok(Modifier::Transparent);
    }

    if meta.path.is_ident("from") {
        return Ok(Modifier::From {
            item: String::new(),
        });
    }

    if meta.path.is_ident("try_from") {
        return Ok(Modifier::TryFrom {
            item: String::new(),
        });
    }

    if meta.path.is_ident("into") {
        return Ok(Modifier::Into {
            item: String::new(),
        });
    }

    if meta.path.is_ident("crate") {
        return Ok(Modifier::Crate {
            path: String::new(),
        });
    }

    if meta.path.is_ident("expecting") {
        return Ok(Modifier::Expecting {
            expectation: String::new(),
        });
    }

    if meta.path.is_ident("variant_identifier") {
        return Ok(Modifier::VariantIdentifier);
    }

    if meta.path.is_ident("field_identifier") {
        return Ok(Modifier::FieldIdentifier);
    }

    if meta.path.is_ident("alias") {
        return Ok(Modifier::Alias {
            name: String::new(),
        });
    }

    if meta.path.is_ident("skip") {
        return Ok(Modifier::Skip);
    }

    if meta.path.is_ident("skip_serializing") {
        return Ok(Modifier::SkipSerializing);
    }

    if meta.path.is_ident("skip_deserializing") {
        return Ok(Modifier::SkipDeserializing);
    }

    if meta.path.is_ident("serialize_with") {
        return Ok(Modifier::SerializeWith { imp: String::new() });
    }

    if meta.path.is_ident("deserialize_with") {
        return Ok(Modifier::DeserializeWith { imp: String::new() });
    }

    if meta.path.is_ident("with") {
        return Ok(Modifier::With { imp: String::new() });
    }

    if meta.path.is_ident("borrow") {
        return Ok(Modifier::Borrow { li: None });
    }

    if meta.path.is_ident("other") {
        return Ok(Modifier::Other);
    }

    if meta.path.is_ident("flatten") {
        return Ok(Modifier::Flatten);
    }

    if meta.path.is_ident("skip_serializing_if") {
        return Ok(Modifier::SkipSerializingIf { imp: String::new() });
    }

    if meta.path.is_ident("getter") {
        return Ok(Modifier::Getter {
            item: String::new(),
        });
    }

    Err(Error::new(Span::call_site(), "unsupported attribute"))
}

fn attr_modifiers(attrs: &[Attribute]) -> Result<Vec<Modifier>> {
    let mut modifiers = Vec::new();

    for attr in attrs {
        if !attr.path().is_ident("serde") {
            continue;
        }

        attr.parse_nested_meta(|meta| {
            modifiers.push(parse_modifier(meta)?);
            Ok(())
        })?;
    }

    Ok(modifiers)
}

pub fn attr_struct_opts(item: &ItemStruct) -> Result<ContainerOpts> {
    let mut opts = ContainerOpts::default();
    let modifiers = attr_modifiers(&item.attrs)?;
    opts.apply_modifiers(&modifiers).unwrap();
    Ok(opts)
}

pub fn attr_enum_opts(item: &ItemEnum) -> Result<ContainerOpts> {
    let mut opts = ContainerOpts::default();
    let modifiers = attr_modifiers(&item.attrs)?;
    opts.apply_modifiers(&modifiers).unwrap();
    Ok(opts)
}

pub fn attr_variant_opts(variant: &Variant) -> Result<VariantOpts> {
    let mut opts = VariantOpts::default();
    let modifiers = attr_modifiers(&variant.attrs)?;
    opts.apply_modifiers(&modifiers).unwrap();
    Ok(opts)
}

pub fn attr_field_opts(field: &Field) -> Result<FieldOpts> {
    let mut opts = FieldOpts::default();
    let modifiers = attr_modifiers(&field.attrs)?;
    opts.apply_modifiers(&modifiers).unwrap();
    Ok(opts)
}

pub fn name_of_field(field: &Field, opts: &FieldOpts) -> String {
    opts.rename
        .clone()
        .unwrap_or_else(|| field.ident.as_ref().unwrap().to_string())
}

pub fn name_of_variant(variant: &Variant, opts: &VariantOpts) -> String {
    opts.rename
        .clone()
        .unwrap_or_else(|| variant.ident.to_string())
}
