mod primitives;
mod types;
mod consts;

use std::{borrow::Borrow, str::FromStr};
pub(crate) use crate::_internal::*;
pub use consts::*;
use crate::schemas::*;
pub use primitives::*;
pub use types::*;

#[repr(C)]
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct BorrowedQname<'a, 'b> {
    pub namespace: &'a str,
    pub name: &'b str,
}

impl<'a> Borrow<BorrowedQname<'a, 'a>> for xsd::QName
where
    Self: 'a,
{
    fn borrow(&self) -> &BorrowedQname<'a, 'a> {
        unsafe { &*(self as *const _ as *const _) }
    }
}

impl<'a, 'b> AsRef<Self> for BorrowedQname<'a, 'b> {
    fn as_ref(&self) -> &Self {
        &self
    }
}

impl<'a> ToOwned for BorrowedQname<'a, 'a> {
    type Owned = xsd::QName;

    fn to_owned(&self) -> Self::Owned {
        self.into()
    }
}
impl<'a, 'b> BorrowedQname<'a, 'b> {
    #[inline]
    pub const fn new(ns: &'a str, name: &'b str) -> Self {
        Self {
            namespace: ns,
            name,
        }
    }
    #[inline]
    pub const fn as_str_tuple(&self) -> (&'a str, &'b str) {
        (&self.namespace, &self.name)
    }
}

impl<'a, 'b> From<&BorrowedQname<'a, 'b>> for xsd::QName {
    fn from(qname: &BorrowedQname<'a, 'b>) -> Self {
        Self::new(qname.name, qname.namespace)
    }
}
pub type StaticQname = BorrowedQname<'static, 'static>;
#[derive(Debug)]
pub enum ParsingError {
    Type { ty: StaticQname, value: Str },
}

#[derive(Debug)]
pub struct Ref {
    pub name: xsd::QName,
}

#[derive(Debug)]
pub enum Declared<T> {
    ///Defiinition
    Def(T),
    ///Lookup item byt its qualified name
    Ref(Ref),
}

pub trait XsdNode {
    type Def;
    type Declared;
}
/*
impl<T> Declaration<T> {
    pub const fn id(&self) -> xsd::Id {
        match &self {
            Declaration::Def(def) => def.id,
            Declaration::Ref(r) => todo!(),
        }
    }
}*/

#[derive(Debug, Default, Clone, Copy)]
///The default is [`Form::Unqualified`].
pub enum Form {
    ///If the value is qualified, this element must be qualified with the namespace prefix.
    Qualified,
    #[default]
    ///If the value is unqualified, this element is not required to be qualified with the namespace prefix.
    Unqualified,
}

impl FromStr for Form {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "qualified" => Ok(Self::Qualified),
            "unqualified" => Ok(Self::Unqualified),
            other => Err(ParseError(other.into())),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy)]
///The block attribute prevents a [`xsd::ComplexType`] or [`xsd::Element`] that has the specified type of derivation from being used in place of the inherited [`xsd::ComplexType`] or [`xsd::Element`].
pub enum Block {
    #[default]
    None = 0,
    ///Prevents elements derived by extension from being used in place of this element.
    Extension = 1 << 1,
    ///Prevents elements derived by restriction from being used in place of this element.
    Restriction = 1 << 2,
    ///Prevents elements derived by substitution from being used in place of this element.
    Substitution = 1 << 3,
    ///Prevents all derived elements from being used in place of this element.
    All = (1 << 1) + (1 << 2) + (1 << 3),
}

impl std::ops::BitOr for Block {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        let or = self as u8 | rhs as u8;
        return unsafe { *(or as *const u8 as *const Block) };
    }
}
impl std::ops::BitOrAssign for Block {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl FromStr for Block {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut block = Block::None;
        for dev in s.split(" ") {
            match dev {
                "extension" => block |= Block::Extension,
                "restriction" => block |= Block::Restriction,
                "Substitution" => block |= Block::Substitution,
                "#all" => block |= Block::All,
                _ => {}
            }
        }
        Ok(block)
    }
}

#[derive(Debug, Default, Clone, Copy)]
///The type of derivation.
#[repr(u8)]
pub enum Final {
    #[default]
    None = 0,
    ///Prevents derivation by extension. Applies to [`xsd::Element`] and [`xsd::ComplexType`] elements only.
    Extension = 1 << 1,
    ///Prevents derivation by restriction.
    Restriction = 1 << 2,
    ///Prevents derivation by list. Applies to [`xsd::SimpleType`] elements only.
    List = 1 << 3,
    ///Prevents derivation by union. Applies to [`xsd::SimpleType`] elements only.
    Union = 1 << 4,
    ///By default, elements in this schema cannot be derived by any method.
    All = (1 << 1) + (1 << 2) + (1 << 3) + (1 << 4),
}

impl std::ops::BitOr for Final {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        let or = self as u8 | rhs as u8;
        return unsafe { *(or as *const u8 as *const Final) };
    }
}
impl std::ops::BitOrAssign for Final {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl FromStr for Final {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut block = Final::None;
        for dev in s.split(" ") {
            match dev {
                "extension" => block |= Final::Extension,
                "restriction" => block |= Final::Restriction,
                "list" => block |= Final::List,
                "union" => block |= Final::Union,
                "#all" => block |= Final::All,
                _ => {}
            }
        }
        Ok(block)
    }
}

flatten!(defs);
flatten!(common);
flatten!(attribute);
flatten!(element);

//#ANNOTATION
#[derive(Debug)]
pub enum AnnotationContent {
    ///Specifies information to be read or used by users within an annotation element.
    Documentation {
        source: Option<Str>,
        lang: Option<Str>,
        content: crate::utils::GenericElement,
    },
    ///Specifies information to be used by applications within an annotation element.
    AppInfo(xsd::AppInfo),
}

//#SIMPLETYPE

#[derive(Debug, Default)]
///Default is [`SimpleValueOption::None`]
pub enum SimpleValueOption {
    ///The default value of the element if its content is a simple type or its content is textOnly
    Default(Str),
    ///The predetermined, unchangeable value of the element if its content is a simple type or its content is textOnly.
    Fixed(Str),
    #[default]
    None,
}
