
use super::*;
#[xsd_proc::xsdnode(default_or_fixed = SimpleValueOption::None)]
#[derive(Debug)]
///Declares an attribute.
pub struct AttributeDef {
    ///If the element contains a simple type, this value must be a valid value of that type.
    pub default_or_fixed: SimpleValueOption,
    ///The form for the element.
    pub form: Form,
    /// The name of the element
    pub name: xsd::QName,
    /// The type of this element
    pub type_: Declared<xsd::SimpleType>,
}

impl Def for AttributeDef {
    type Args = AttributeArgs;
}

#[derive(Debug, Default)]
pub struct AttributeArgs {
    ///An indicator of how the attribute is used, default is optional
    pub occurs: AttrOccurs
}

#[xsd_proc::xsdnode]
#[derive(Debug)]
pub struct AttributeGroupDef {
    pub name: xsd::QName,
    pub attributes: List<AttributeBased>
}

impl Def for AttributeGroupDef {
    type Args = AttributeArgs;
}

#[derive(Debug)]
pub enum AttributeBased {
    Attribute(<xsd::Attribute as XsdNode>::Declared),
    Group(<xsd::AttributeGroup as XsdNode>::Declared)
}

#[derive(Debug, Default)]
///Default [`AttrOccurs::Optional`]
pub enum AttrOccurs {
    ///Attribute is optional and may have any value.
    #[default]
    Optional,
    ///Attribute cannot be used.
    Prohibited,
    ///Attribute must appear once.
    ///
    ///The attribute is required and can contain any value allowed by this type definition of the attribute.
    Required,
}
