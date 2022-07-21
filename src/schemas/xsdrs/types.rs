
use std::fmt::Debug;

use super::*;

pub trait Restriction {
    type Restriction:Debug;
}
#[derive(Debug)]
pub enum ComplexTypeContent<T:Restriction> {
    Extension(xsd::Extension),
    Restriction(xsd::Restriction<T>),
}

#[derive(Debug)]
pub enum SimpleTypeContent {
    List(xsd::List),
    Union(xsd::Union),
    Restriction(xsd::Restriction<xsd::SimpleType>),
}

#[derive(Debug)]
pub struct SimpleTypeRestriction {
    pub facets: List<Facet>,
}

impl Restriction for xsd::SimpleType {
    type Restriction = SimpleTypeRestriction;
}


#[derive(Debug)]
pub struct SimpleContentRestriction {
    pub facets: List<Facet>,
    pub attributes: List<AttributeBased>
}

impl Restriction for xsd::SimpleContent {
    type Restriction = SimpleContentRestriction;
}

#[derive(Debug)]
pub enum SimpleMemberType {
    AttrRef(xsd::QName),
    Ref(xsd::QName),
    Def(xsd::SimpleType),
}

#[derive(Debug)]
pub enum ComplexContent {
    Simple(xsd::SimpleContent),
    Complex(xsd::ComplexContent),
    Based(Option<ElementBase>, List<AttributeBased>)
}

#[derive(Debug)]
pub struct ComplexContentRestriction {
    pub content: ElementBase,
    pub attributes: List<AttributeBased>
}

impl Restriction for xsd::ComplexContent {
    type Restriction = ComplexContentRestriction;
}

//(simpleContent | complexContent | ((group | all | choice | sequence)?, ((attribute | attributeGroup)*, anyAttribute?))))
