use std::{fmt::Debug, hash::Hash};

use super::*;

pub trait Def: Debug {
    type Args: Debug;
}

#[derive(Debug)]
pub struct Decl<D: Def> {
    pub decl: Declared<D>,
    pub args: D::Args,
}

#[derive(Debug)]
pub enum XsdDcl<D: Def> {
    Def(D),
    Decl(Decl<D>),
}

impl<D:Def> XsdNode for XsdDcl<D> {
    type Def = D;
    type Declared = Decl<D>;
}
#[derive(Debug)]
pub enum AttributeBasedDef {
    Attribute(AttributeDef),
    Group(AttributeGroupDef),
}

impl Hash for AttributeBasedDef {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match &self {
            AttributeBasedDef::Attribute(attr) => attr.name.hash(state),
            AttributeBasedDef::Group(group) => group.name.hash(state),
        }
    }
}

#[derive(Debug)]
pub enum TypeDef {
    Simple(xsd::SimpleType),
    Complex(xsd::ComplexType),
}

#[derive(Debug)]
pub enum Definition {
    Element(ElementDef),
    ElementGroup(ElementGroupDef),
    Attribute(AttributeDef),
    AttributeGroup(AttributeGroupDef),
    SimpleType(xsd::SimpleType),
    ComplexType(xsd::ComplexType),
}

impl Definition {
    #[inline]
    pub const fn name(&self) -> &xsd::QName {
        match &self {
            Definition::Element(el) => &el.name,
            Definition::ElementGroup(group) => &group.name,
            Definition::Attribute(attr) => &attr.name,
            Definition::AttributeGroup(group) => &group.name,
            Definition::SimpleType(ty) => &ty.name,
            Definition::ComplexType(ty) => &ty.name,

        }
    }
    
}

impl Hash for Definition {
    fn hash<H>(&self, h: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.name().hash(h)
    }
}

impl PartialEq for Definition {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

impl Eq for Definition {}
