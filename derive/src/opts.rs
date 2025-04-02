use std::default::Default;
use std::mem;
use std::str::FromStr;

use anyhow::{Error, Result, anyhow, bail};

// https://serde.rs/attributes.html
pub enum Modifier {
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

    // #[serde(flatten)]
    Flatten,

    // #[serde(skip_serializing_if = "path")]
    SkipSerializingIf {
        imp: String,
    },

    // #[serde(getter = "...")]
    Getter {
        item: String,
    },
}

impl Modifier {
    fn kind_name(&self) -> String {
        format!("{:?}", mem::discriminant(self))
    }
}

// ----------------------------------------------------------

trait CompositeOpt {
    fn try_apply_modifier(&mut self, modifier: &Modifier) -> Result<bool>;
}

trait OptionSet {
    fn apply_modifiers(&mut self, modifiers: &[Modifier]) -> Result<()>;
}

// ----------------------------------------------------------

pub enum Case {
    Lowercase,
    Uppercase,
    PascalCase,
    CamelCase,
    SnakeCase,
    ScreamingSnakeCase,
    KebabCase,
    ScreamingKebabCase,
}

impl FromStr for Case {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "lowercase" => Ok(Case::Lowercase),
            "UPPERCASE" => Ok(Case::Uppercase),
            "PascalCase" => Ok(Case::PascalCase),
            "camelCase" => Ok(Case::CamelCase),
            "snake_case" => Ok(Case::SnakeCase),
            "SCREAMING_SNAKE_CASE" => Ok(Case::ScreamingSnakeCase),
            "kebab-case" => Ok(Case::KebabCase),
            "SCREAMING-KEBAB-CASE" => Ok(Case::ScreamingKebabCase),
            _ => bail!("invalid case: {}", s),
        }
    }
}

pub enum TagStyle {
    External,
    Internal { field: String },
    Adjacent { tag: String, content: String },
    Untagged,
}

impl CompositeOpt for TagStyle {
    fn try_apply_modifier(&mut self, modifier: &Modifier) -> Result<bool> {
        match modifier {
            Modifier::TagInternal { field } => {
                *self = TagStyle::Internal {
                    field: field.clone(),
                };
                Ok(true)
            }
            Modifier::TagAdjacent { tag, content } => {
                *self = TagStyle::Adjacent {
                    tag: tag.clone(),
                    content: content.clone(),
                };
                Ok(true)
            }
            Modifier::Untagged => {
                *self = TagStyle::Untagged;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

pub struct DefaultValue {
    pub on: bool,
    pub path: Option<String>,
}

impl Default for DefaultValue {
    fn default() -> Self {
        Self {
            on: false,
            path: None,
        }
    }
}

impl CompositeOpt for DefaultValue {
    fn try_apply_modifier(&mut self, modifier: &Modifier) -> Result<bool> {
        match modifier {
            Modifier::Default { item } => {
                self.on = true;
                self.path = item.clone();
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

pub struct Skip {
    pub serializing: bool,
    pub serializing_if: Option<String>,
    pub deserializing: bool,
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

impl CompositeOpt for Skip {
    fn try_apply_modifier(&mut self, modifier: &Modifier) -> Result<bool> {
        match modifier {
            Modifier::Skip => {
                self.serializing = true;
                self.deserializing = true;
                Ok(true)
            }
            Modifier::SkipSerializing => {
                self.serializing = true;
                Ok(true)
            }
            Modifier::SkipSerializingIf { imp } => {
                self.serializing = true;
                self.serializing_if = Some(imp.clone());
                Ok(true)
            }
            Modifier::SkipDeserializing => {
                self.deserializing = true;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

pub struct With {
    pub module: Option<String>,
    pub serialize_fn: Option<String>,
    pub deserialize_fn: Option<String>,
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

impl CompositeOpt for With {
    fn try_apply_modifier(&mut self, modifier: &Modifier) -> Result<bool> {
        match modifier {
            Modifier::With { imp } => {
                self.module = Some(imp.clone());
                Ok(true)
            }
            Modifier::SerializeWith { imp } => {
                self.serialize_fn = Some(imp.clone());
                Ok(true)
            }
            Modifier::DeserializeWith { imp } => {
                self.deserialize_fn = Some(imp.clone());
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

// ----------------------------------------------------------

fn bad_modifier(modifier: &Modifier) -> Error {
    anyhow!("bad modifier: {:?}", modifier.kind_name())
}

// ----------------------------------------------------------

pub struct ContainerOpts {
    pub rename: Option<String>,
    pub rename_all: Option<Case>,
    pub tag_style: TagStyle,
    pub default: DefaultValue,
    pub remote: Option<String>,
    pub transparent: bool,
    pub from: Option<String>,
    pub try_from: Option<String>,
    pub into: Option<String>,
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
        for modifier in modifiers {
            match modifier {
                Modifier::Rename {
                    serialize_name,
                    deserialize_name,
                } => {
                    assert_eq!(serialize_name, deserialize_name);
                    if let Some(name) = serialize_name {
                        self.rename = Some(name.clone());
                    }
                }
                Modifier::RenameAll {
                    serialize_case,
                    deserialize_case,
                } => {
                    assert_eq!(serialize_case, deserialize_case);
                    if let Some(case) = serialize_case {
                        self.rename_all = Some(Case::from_str(case)?);
                    }
                }
                Modifier::TagInternal { field } => {
                    self.tag_style = TagStyle::Internal {
                        field: field.clone(),
                    };
                }
                Modifier::TagAdjacent { tag, content } => {
                    self.tag_style = TagStyle::Adjacent {
                        tag: tag.clone(),
                        content: content.clone(),
                    };
                }
                _ if self.default.try_apply_modifier(modifier)? => {}
                Modifier::Remote { item } => {
                    self.remote = Some(item.clone());
                }
                Modifier::Transparent => self.transparent = true,
                Modifier::From { item } => {
                    self.from = Some(item.clone());
                }
                Modifier::TryFrom { item } => {
                    self.try_from = Some(item.clone());
                }
                Modifier::Into { item } => {
                    self.into = Some(item.clone());
                }
                _ => return Err(bad_modifier(modifier)),
            }
        }

        Ok(())
    }
}

// ----------------------------------------------------------

pub struct VariantOpts {
    pub rename: Option<String>,
    pub rename_all: Option<Case>,
    pub skip: Skip,
    pub with: With,
    pub other: bool,
    pub untagged: bool,
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
        for modifier in modifiers {
            match modifier {
                Modifier::Rename {
                    serialize_name,
                    deserialize_name,
                } => {
                    assert_eq!(serialize_name, deserialize_name);
                    if let Some(name) = serialize_name {
                        self.rename = Some(name.clone());
                    }
                }
                Modifier::RenameAll {
                    serialize_case,
                    deserialize_case,
                } => {
                    assert_eq!(serialize_case, deserialize_case);
                    if let Some(case) = serialize_case {
                        self.rename_all = Some(Case::from_str(case)?);
                    }
                }
                _ if self.skip.try_apply_modifier(modifier)? => {}
                _ if self.with.try_apply_modifier(modifier)? => {}
                Modifier::Other => self.other = true,
                Modifier::Untagged => self.untagged = true,
                _ => return Err(bad_modifier(modifier)),
            }
        }

        Ok(())
    }
}

// ----------------------------------------------------------

pub struct FieldOpts {
    pub rename: Option<String>,
    pub default: DefaultValue,
    pub flatten: bool,
    pub skip: Skip,
    pub with: With,
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
        for modifier in modifiers {
            match modifier {
                Modifier::Rename {
                    serialize_name,
                    deserialize_name,
                } => {
                    assert_eq!(serialize_name, deserialize_name);
                    if let Some(name) = serialize_name {
                        self.rename = Some(name.clone());
                    }
                }
                _ if self.default.try_apply_modifier(modifier)? => {}
                Modifier::Flatten => self.flatten = true,
                _ if self.skip.try_apply_modifier(modifier)? => {}
                _ if self.with.try_apply_modifier(modifier)? => {}
                _ => return Err(bad_modifier(modifier)),
            }
        }

        Ok(())
    }
}
