use std::borrow::Borrow;

use super::*;
use crate::{_internal::Str, utils::parse_str};
#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct QName {
    pub namespace: xsdrs::Str,
    pub name: xsdrs::Str,
}
impl QName {
    #[inline]
    pub const fn as_str_tuple(&self) -> (&str, &str) {
        (&self.namespace, &self.name)
    }
    pub fn from_bytes(name: &[u8], ns: Option<&[u8]>) -> Self {
        QName {
            name: parse_str(name).into(),
            namespace: ns.map(parse_str).unwrap_or("").into(),
        }
    }
    pub fn new<A: Into<Str>, B: Into<Str>>(name: A, ns: B) -> Self {
        QName {
            name: name.into(),
            namespace: ns.into(),
        }
    }
    pub fn local<T: Into<Str>>(name: T) -> Self {
        QName {
            name: name.into(),
            namespace: Default::default(),
        }
    }
    #[inline]
    pub const fn borrowed(&self) -> xsdrs::BorrowedQname {
        let Self { namespace, name } = self;
        xsdrs::BorrowedQname { name, namespace }
    }
}
impl<'a> Borrow<(&'a str, &'a str)> for QName {
    fn borrow(&self) -> &'a (&'a str, &'a str) {
        unsafe { &*(self as *const _ as *const (&str, &str)) }
    }
}

impl Borrow<(Str, Str)> for QName {
    fn borrow(&self) -> &(Str, Str) {
        unsafe { &*(self as *const _ as *const (Str, Str)) }
    }
}

pub type Id = xsdrs::Str;

pub type AnyUri = xsdrs::Str;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Boolean(bool);

xsd_proc::const_qname!("boolean", XSD);
impl FromStr for Boolean {
    type Err = xsdrs::ParsingError;

    fn from_str(bool: &str) -> Result<Self, Self::Err> {
        Ok(Boolean(match bool {
            "true" => true,
            "false" => false,
            "0" => false,
            "1" => true,
            other => {
                return Err(ParsingError::Type {
                    ty: BOOLEAN,
                    value: other.into(),
                })
            }
        }))
    }
}
