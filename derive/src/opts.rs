use anyhow::{Error, Result};

// https://serde.rs/attributes.html
enum Modifier {
    // #[serde(rename = "name")]
    Rename {
        name: String,
    },

    // #[serde(rename_all = "...")]
    RenameAll {
        case: String,
    },

    // #[serde(rename_all_fields = "...")]
    RenameAllFeilds {
        case: String,
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

enum TagStyle {
    External,
    Internal { field: String },
    Adjacent { tag: String, content: String },
    Untagged,
}

struct Default {
    on: bool,
    path: Option<String>,
}

struct Skip {
    serializing: bool,
    serializing_if: Option<String>,
    deserializing: bool,
}

struct With {
    module: Option<String>,
    serialize_fn: Option<String>,
    deserialize_fn: Option<String>,
}

struct ContainerOpts {
    rename: Option<String>,
    rename_all: Option<String>,
    tag_style: TagStyle,
    default: Default,
    remote: Option<String>,
    transparent: bool,
    from: Option<String>,
    try_from: Option<String>,
    into: Option<String>,
}

impl TryFrom<Vec<Modifier>> for ContainerOpts {
    type Error = Error;

    fn try_from(_modifiers: Vec<Modifier>) -> Result<Self> {
        todo!()
    }
}

struct VariantOpts {
    rename: Option<String>,
    rename_all: Option<String>,
    skip: Skip,
    with: With,
    other: bool,
    untagged: bool,
}

impl TryFrom<Vec<Modifier>> for VariantOpts {
    type Error = Error;

    fn try_from(_modifiers: Vec<Modifier>) -> Result<Self> {
        todo!()
    }
}

struct FieldOpts {
    rename: Option<String>,
    default: Default,
    flatten: bool,
    skip: Skip,
    with: With,
}

impl TryFrom<Vec<Modifier>> for FieldOpts {
    type Error = Error;

    fn try_from(_modifiers: Vec<Modifier>) -> Result<Self> {
        todo!()
    }
}
