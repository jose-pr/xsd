use crate::utils::GenericAttributes;

use super::*;

#[derive(Debug)]
pub struct Occurs {
    ///The minimum number of times the element can occur within the containing element.
    pub min: u32,
    ///The maximum number of times the element can occur within the containing element.
    pub max: Option<u32>,
}
impl Occurs {
    pub const fn single() -> Self {
        Self {
            min: 1,
            max: Some(1),
        }
    }
    pub const fn optional() -> Self {
        Self {
            min: 0,
            max: Some(1),
        }
    }
    pub const fn list(max: Option<u32>) -> Self {
        Self { min: 0, max }
    }
    pub const fn is_list(&self) -> bool {
        match self.max {
            Some(val) => val > 1,
            None => true,
        }
    }
    pub const fn is_single(&self) -> bool {
        (match self.max {
            Some(val) => val == 1,
            None => true,
        }) && self.min == 1
    }
    pub const fn is_optional(&self) -> bool {
        (match self.max {
            Some(val) => val == 1,
            None => true,
        }) && self.min == 0
    }
}

impl TryFrom<&mut GenericAttributes> for Occurs {
    type Error = ParsingError;

    fn try_from(attrs: &mut GenericAttributes) -> Result<Self, Self::Error> {
        Ok(xsdrs::Occurs {
            max: attrs
                .extract(MAX_OCCURS)
                .map(|max| {
                    if max.as_ref() == "unbounded" {
                        None
                    } else {
                        max.parse().ok()
                    }
                })
                .unwrap_or(Some(1)),
            min: attrs.extract_parsed(MIN_OCCURS).unwrap_or(1),
        })
    }
}
impl Default for Occurs {
    fn default() -> Self {
        Occurs {
            min: 1,
            max: Some(1),
        }
    }
}

#[xsd_proc::xsdnode(abstract_=false, mixed=false, occurs=Occurs::single(), nillable=false)]
#[derive(Debug)]
///Declares an element.
pub struct ElementDef {
    ///An indicator of whether the element can be used in an instance document. If this value is true, the element cannot appear in the instance document. Instead, another element whose substitutionGroup attribute contains the qualified name (QName) of this element must appear in this element's place. More than one element can reference this element in its substitutionGroup attribute.
    /// Default is false
    pub abstract_: bool,
    ///The type of derivation. The block attribute prevents an element that has the specified type of derivation from being used in place of this element.
    pub block: Block,
    ///If the element contains a simple type, this value must be a valid value of that type.
    pub default_or_fixed: SimpleValueOption,
    ///The name of an element for which this element can be substituted. This element must have the same type or a type derived from the type of the specified element.
    ///
    /// This attribute can be used on any element if the referring element is declared at the global level (parent is schema element)
    pub substitution_group: Option<xsd::QName>,
    ///The type of derivation. The final attribute sets the default value of the final attribute on the element element
    ///
    ///Prohibited if the containing element is not the schema element.
    pub final_: Final,
    ///The form for the element.
    pub form: Form,
    ///The indicator of whether an explicit nil value can be assigned to the element. This applies to element content and not the attributes of the element.
    pub nillable: bool,
    /// The name of the element
    pub name: xsd::QName,
    /// The type of this element
    pub type_: Declared<TypeDef>,
}

#[derive(Debug, Default)]
pub struct ElementArgs {
    pub occurs: Occurs,
}

impl Def for ElementDef {
    type Args = ElementArgs;
}

#[xsd_proc::xsdnode]
#[derive(Debug)]
pub struct ElementGroupDef {
    pub name: xsd::QName,
    pub grouped: Grouped,
}

impl Def for ElementGroupDef {
    type Args = ElementArgs;
}

#[derive(Debug)]
pub enum Grouped {
    Choice(xsd::Choice),
    Sequence(xsd::Sequence),
    All(xsd::All),
}

impl<'a, R: ElementReader> From<(XmlElement<'a, R>, &xsd::Schema)> for Grouped {
    fn from((xml, schema): (XmlElement<'a, R>, &xsd::Schema)) -> Self {
        match xml.name.borrowed() {
            xsd::CHOICE => Grouped::Choice(xsd::Choice::from((xml, schema))),
            xsd::SEQUENCE => Grouped::Sequence(xsd::Sequence::from((xml, schema))),
            xsd::ALL => Grouped::All(xsd::All::from((xml, schema))),
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
pub enum ElementBased {
    Element(<xsd::Element as XsdNode>::Declared),
    Group(<xsd::Group as XsdNode>::Declared),
    Choice(xsd::Choice),
    Sequence(xsd::Sequence),
    Any(xsd::Any),
}

impl<'a, R: ElementReader> From<(XmlElement<'a, R>, &xsd::Schema)> for ElementBased {
    fn from((xml, schema): (XmlElement<'a, R>, &xsd::Schema)) -> Self {
        match xml.name.borrowed() {
            xsd::ELEMENT => ElementBased::Element(Decl::<ElementDef>::from((xml, schema))),
            xsd::GROUP => ElementBased::Group(Decl::<ElementGroupDef>::from((xml, schema))),
            xsd::CHOICE => ElementBased::Choice(xsd::Choice::from((xml, schema))),
            xsd::SEQUENCE => ElementBased::Sequence(xsd::Sequence::from((xml, schema))),
            xsd::ANY => ElementBased::Any(xsd::Any::from((xml, schema))),
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
pub enum ElementBase {
    Empty,
    Group(<xsd::Group as XsdNode>::Declared),
    Choice(xsd::Choice),
    Sequence(xsd::Sequence),
    All(xsd::All),
}
