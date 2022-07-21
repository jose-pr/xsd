/***
 <all
  id = ID
  maxOccurs= 1: 1
  minOccurs= (0 | 1): 1
  {any attributes with non-schema Namespace...}>
Content: (annotation?, element*)
</all>
 */
use super::*;

/// Allows the elements in the group to appear (or not appear) in any order in the containing element.
/// # Examples
/// XML:
/// ```xml
/// <?xml version="1.0"?>
/// <myElement myAttribute="1.1">
/// <thing2>Some</thing2>
/// <thing3>text</thing3>
/// <thing1>for you</thing1>
/// </myElement>
/// ```
/// XSD:
/// ```xml
/// <xs:element name="thing1" type="xs:string"/>
/// <xs:element name="thing2" type="xs:string"/>
/// <xs:element name="thing3" type="xs:string"/>
/// <xs:attribute name="myAttribute" type="xs:decimal"/>
/// <xs:complexType name="myComplexType">
///  <xs:all>
///   <xs:element ref="thing1"/>
///   <xs:element ref="thing2"/>
///   <xs:element ref="thing3"/>
///  </xs:all>
///  <xs:attribute ref="myAttribute"/>
/// </xs:complexType>
/// ```
/// ```
///     All {
///         id: None,
///         annotations: None,
///         optional: false,
///         elements: [
///             Element {
///                 annotation: None,
///                 abstract_: false,
///                 block: xsdrs::ElementDeriviation::None,
///                 default_or_fixed: xsdrs::DefaultOrFixed::None,
///                 substitution_group: None,
///                 final_: xsdrs::Derivitation::None,
///                 form: xsdrs::Form::Qualified,
///                 id: None,
///                 nillable: false,
///                 name: QName { namespace: "".into(), name: "thing1".into() },
///                 type_: xsdrs::NodeType::Ref(),
///                 occurs: xsdrs::Occurs::Once,
///             },
///         ].into(),
///     
/// }
/// ```
///
#[xsd_proc::xsdnode(optional = false)]
#[derive(Debug)]
pub struct All {
    /// Is this element optional, by default it is required ([`false`])
    pub optional: bool,
    /// Elements that need to be present
    pub elements: xsdrs::List<<Element as XsdNode>::Declared>,
}
xsd_proc::const_qname!("all", XSD);

impl<'a, R: ElementReader> From<(XmlElement<'a, R>, &Schema)> for All {
    fn from(
        (
            XmlElement {
                mut attrs,
                name: _,
                mut content,
            },
            schema,
        ): (XmlElement<'a, R>, &Schema),
    ) -> Self {
        let mut content_ = vec![];
        while let Some(child) = content.next() {
            if child.name.borrowed() != ELEMENT {
                panic!()
            }
            content_.push(xsdrs::Decl::<xsdrs::ElementDef>::from((child, schema)));
        }

        All {
            optional: xsdrs::Occurs::try_from(&mut attrs).unwrap().is_optional(),
            elements: content_.into_boxed_slice(),
            id: attrs.extract(xsdrs::ID),
            annotation: None,
        }
    }
}
