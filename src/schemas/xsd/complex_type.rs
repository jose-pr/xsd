/***<complexType
  abstract = Boolean : false
  block = (#all | List of (extension | restriction))
  final = (#all | List of (extension | restriction))
  id = ID
  mixed = Boolean : false
  name = NCName
  {any attributes with non-schema Namespace...}>
Content: (annotation?, (simpleContent | complexContent | ((group | all |
choice | sequence)?, ((attribute | attributeGroup)*, anyAttribute?))))
</complexType>***/
use super::*;
xsd_proc::const_qname!("complexType", XSD);
///Defines a complex type, which determines the set of attributes and the content of an element.
#[xsd_proc::xsdnode(abstract_ = false, mixed = false)]
#[derive(Debug)]
pub struct ComplexType {
    pub name: QName,
    ///An indicator of whether the complex type can be used in an instance document. If this value is true, an element cannot use this complex type directly but must use a complex type derived from this complex type. Default is false
    pub abstract_: bool,
    ///The block attribute prevents a complex type that has the specified type of derivation from being used in place of this complex type. Default is [`xsdrs::Derivitation::None`]
    pub block: xsdrs::Block,
    ///The final attribute prevents the specified type of derivation of this complexType element.  Default is [`xsdrs::Derivitation::None`]
    pub final_: xsdrs::Final,
    ///An indicator of whether character data is allowed to appear between the child elements of this complex type. Default is false
    pub mixed: bool,
    //The name of the type. Only if parent is schema
    // pub name:NCName
    pub content: xsdrs::ComplexContent,
}

impl<'a, R: ElementReader> From<(XmlElement<'a, R>, &Schema)> for ComplexType {
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
        let name = attrs.extract(xsdrs::NAME).unwrap_or_default();
        let name = xsd::QName::local(name);
        let mut complex_content = xsdrs::ComplexContent::Based(None, Default::default());
        if let Some(child) = content.next() {
            match child.name.borrowed() {
                COMPLEX_CONTENT => {}
                SIMPLE_CONTENT => {}
                GROUP => {}
                ALL => {}
                CHOICE => {}
                SEQUENCE => {
                    complex_content = xsdrs::ComplexContent::Based(
                        Some(xsdrs::ElementBase::Sequence(Sequence::from((
                            child, schema,
                        )))),
                        [].into(),
                    );
                }
                ATTRIBUTE => {}
                ATTRIBUTE_GROUP => {}
                ANY_ATTRIBUTE => {}
                _ => panic!(),
            }
        }

        let def = ComplexType::new(
            name,
            attrs.extract_parsed(xsdrs::ABSTRACT),
            attrs
                .extract_parsed(xsdrs::BLOCK)
                .unwrap_or(schema.block_default),
            attrs
                .extract_parsed(xsdrs::FINAL)
                .unwrap_or(schema.final_default),
            attrs.extract_parsed(xsdrs::MIXED),
            complex_content,
            attrs.extract(xsdrs::ID),
            None,
        );
        if !attrs.is_empty() {
            panic!("didnt process keys :{attrs:?}")
        }
        return def;
    }
}
