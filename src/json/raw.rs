use std::borrow::Cow;
use std::fmt::{self, Debug};
use std::mem;

use crate::Place;
use crate::de::{self, Deserialize, Visitor};
use crate::error::{Error, Result};
use crate::ser::{self, Fragment, Serialize};

#[repr(transparent)]
pub struct RawValue {
    json: str,
}

impl RawValue {
    const fn from_borrowed(json: &str) -> &Self {
        unsafe { mem::transmute::<&str, &RawValue>(json) }
    }

    fn from_owned(json: Box<str>) -> Box<Self> {
        unsafe { mem::transmute::<Box<str>, Box<RawValue>>(json) }
    }

    fn into_owned(raw_value: Box<Self>) -> Box<str> {
        unsafe { mem::transmute::<Box<RawValue>, Box<str>>(raw_value) }
    }
}

impl Clone for Box<RawValue> {
    fn clone(&self) -> Self {
        (**self).to_owned()
    }
}

impl ToOwned for RawValue {
    type Owned = Box<RawValue>;

    fn to_owned(&self) -> Self::Owned {
        RawValue::from_owned(self.json.to_owned().into_boxed_str())
    }
}

impl Default for Box<RawValue> {
    fn default() -> Self {
        RawValue::NULL.to_owned()
    }
}

impl Debug for RawValue {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter
            .debug_tuple("RawValue")
            .field(&format_args!("{}", &self.json))
            .finish()
    }
}

impl RawValue {
    pub const NULL: &'static RawValue = RawValue::from_borrowed("null");
    pub const TRUE: &'static RawValue = RawValue::from_borrowed("true");
    pub const FALSE: &'static RawValue = RawValue::from_borrowed("false");

    pub fn from_string(json: String) -> Box<Self> {
        Self::from_owned(json.into_boxed_str())
    }

    pub fn get(&self) -> &str {
        &self.json
    }
}

impl From<Box<RawValue>> for Box<str> {
    fn from(raw_value: Box<RawValue>) -> Self {
        RawValue::into_owned(raw_value)
    }
}

pub fn to_raw_value<T>(value: &T) -> Result<Box<RawValue>>
where
    T: ?Sized + Serialize,
{
    let json_string = super::to_string(value);
    Ok(RawValue::from_owned(json_string.into_boxed_str()))
}

pub const TOKEN: &str = "$qser::json::private::RawValue";

impl Serialize for RawValue {
    fn begin(&self) -> Fragment {
        struct RawValueStream<'a> {
            data: &'a RawValue,
            state: bool,
        }

        impl<'a> ser::Map for RawValueStream<'a> {
            fn next(&mut self) -> Option<(Cow<str>, &dyn Serialize)> {
                if !self.state {
                    self.state = true;
                    Some((Cow::Borrowed(TOKEN), &self.data))
                } else {
                    None
                }
            }
        }

        Fragment::Map(Box::new(RawValueStream {
            data: self,
            state: false,
        }))
    }
}

impl Deserialize for Box<RawValue> {
    fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
        struct RawValueBuilder<'a> {
            json: Option<String>, // TODO: impl deserialize for box<str>
            out: &'a mut Option<Box<RawValue>>,
        }

        impl<'a> de::Map for RawValueBuilder<'a> {
            fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
                match k {
                    TOKEN => Ok(Deserialize::begin(&mut self.json)),
                    _ => Ok(<dyn Visitor>::ignore()),
                }
            }

            fn finish(&mut self) -> Result<()> {
                let json = self.json.take().ok_or(Error)?;
                *self.out = Some(RawValue::from_string(json));
                Ok(())
            }
        }

        #[allow(non_local_definitions)]
        impl Visitor for Place<Box<RawValue>> {
            fn map(&mut self) -> Result<Box<dyn de::Map + '_>> {
                Ok(Box::new(RawValueBuilder {
                    json: None,
                    out: &mut self.out,
                }))
            }
        }

        Place::new(out)
    }
}
