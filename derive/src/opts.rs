use std::default::Default;

use anyhow::{Error, Result};

// https://serde.rs/attributes.html
enum Modifier {
    // #[serde(rename = "name")]
    Rename {
        serialize_name: Option<String>,
        deserialize_name: Option<String>,
    },

    // #[serde(rename_all = "...")]
    RenameAll {
        serialize_case: Option<String>,
        deserialize_case: Option<String>,
    },

    // #[serde(rename_all_fields = "...")]
    RenameAllFields {
        serialize_case: Option<String>,
        deserialize_case: Option<String>,
    },

    // #[serde(deny_unknown_fields)]
    DenyUnknownFields,

    // #[serde(tag = "type")]
    TagInternal {
        field: String,
    },

    // #[serde(tag = "t", content = "c")]
    TagAdjacent {
        tag: String,
        content: String,
    },

    // #[serde(untagged)]
    Untagged,

    // #[serde(bound = "T: MyTrait")]
    // #[serde(bound(serialize = "T: MySerTrait"))]
    // #[serde(bound(deserialize = "T: MyDeTrait"))]
    // #[serde(bound(serialize = "T: MySerTrait", deserialize = "T: MyDeTrait"))]
    Bound {
        serialize: Option<String>,
        deserialize: Option<String>,
    },

    // #[serde(default)]
    // #[serde(default = "path")]
    Default {
        item: Option<String>,
    },

    // #[serde(remote = "...")]
    Remote {
        item: String,
    },

    // #[serde(transparent)]
    Transparent,

    // #[serde(from = "FromType")]
    From {
        item: String,
    },

    // #[serde(try_from = "FromType")]
    TryFrom {
        item: String,
    },

    // #[serde(into = "IntoType")]
    Into {
        item: String,
    },

    // #[serde(crate = "...")]
    Crate {
        path: String,
    },

    // #[serde(expecting = "...")]
    Expecting {
        expectation: String,
    },

    // #[serde(variant_identifier)]
    VariantIdentifier,

    // #[serde(field_identifier)]
    FieldIdentifier,

    // #[serde(alias = "name")]
    Alias {
        name: String,
    },

    // #[serde(skip)]
    Skip,

    // #[serde(skip_serializing)]
    SkipSerializing,

    // #[serde(skip_deserializing)]
    SkipDeserializing,

    // #[serde(serialize_with = "path")]
    SerializeWith {
        imp: String,
    },

    // #[serde(deserialize_with = "path")]
    DeserializeWith {
        imp: String,
    },

    // #[serde(with = "module")]
    With {
        imp: String,
    },

    // #[serde(borrow)]
    // #[serde(borrow = "'a + 'b + ...")]
    Borrow {
        li: Option<String>,
    },

    // #[serde(other)]
    Other,

    // #[serde(skip_serializing_if = "path")]
    SkipSerializingIf {
        imp: String,
    },

    // #[serde(getter = "...")]
    Getter {
        item: String,
    },
}

// ----------------------------------------------------------

enum Case {
    Lowercase,
    Uppercase,
    PascalCase,
    CamelCase,
    SnakeCase,
    ScreamingSnakeCase,
    KebabCase,
    ScreamingKebabCase,
}

enum TagStyle {
    External,
    Internal { field: String },
    Adjacent { tag: String, content: String },
    Untagged,
}

struct DefaultValue {
    on: bool,
    path: Option<String>,
}

impl Default for DefaultValue {
    fn default() -> Self {
        Self {
            on: false,
            path: None,
        }
    }
}

struct Skip {
    serializing: bool,
    serializing_if: Option<String>,
    deserializing: bool,
}

impl Default for Skip {
    fn default() -> Self {
        Self {
            serializing: false,
            serializing_if: None,
            deserializing: false,
        }
    }
}

struct With {
    module: Option<String>,
    serialize_fn: Option<String>,
    deserialize_fn: Option<String>,
}

impl Default for With {
    fn default() -> Self {
        Self {
            module: None,
            serialize_fn: None,
            deserialize_fn: None,
        }
    }
}

// ----------------------------------------------------------

trait OptionSet {
    fn apply_modifiers(&mut self, modifiers: &[Modifier]) -> Result<()>;
}

// ----------------------------------------------------------

struct ContainerOpts {
    rename: Option<String>,
    rename_all: Option<Case>,
    tag_style: TagStyle,
    default: DefaultValue,
    remote: Option<String>,
    transparent: bool,
    from: Option<String>,
    try_from: Option<String>,
    into: Option<String>,
}

impl Default for ContainerOpts {
    fn default() -> Self {
        Self {
            rename: None,
            rename_all: None,
            tag_style: TagStyle::External,
            default: DefaultValue::default(),
            remote: None,
            transparent: false,
            from: None,
            try_from: None,
            into: None,
        }
    }
}

impl OptionSet for ContainerOpts {
    fn apply_modifiers(&mut self, modifiers: &[Modifier]) -> Result<()> {
        todo!()
    }
}

// ----------------------------------------------------------

struct VariantOpts {
    rename: Option<String>,
    rename_all: Option<Case>,
    skip: Skip,
    with: With,
    other: bool,
    untagged: bool,
}

impl Default for VariantOpts {
    fn default() -> Self {
        Self {
            rename: None,
            rename_all: None,
            skip: Skip::default(),
            with: With::default(),
            other: false,
            untagged: false,
        }
    }
}

impl OptionSet for VariantOpts {
    fn apply_modifiers(&mut self, modifiers: &[Modifier]) -> Result<()> {
        todo!()
    }
}

// ----------------------------------------------------------

struct FieldOpts {
    rename: Option<String>,
    default: DefaultValue,
    flatten: bool,
    skip: Skip,
    with: With,
}

impl Default for FieldOpts {
    fn default() -> Self {
        Self {
            rename: None,
            default: DefaultValue::default(),
            flatten: false,
            skip: Skip::default(),
            with: With::default(),
        }
    }
}

impl OptionSet for FieldOpts {
    fn apply_modifiers(&mut self, modifiers: &[Modifier]) -> Result<()> {
        todo!()
    }
}
