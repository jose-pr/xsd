/***
 * <schema
  attributeFormDefault = (qualified | unqualified): unqualified
  blockDefault = (#all | List of (extension | restriction | substitution) : ''
  elementFormDefault = (qualified | unqualified): unqualified
  finalDefault = (#all | List of (extension | restriction | list |
union): ''
  id = ID
  targetNamespace = anyURI
  version = token
  xml:lang = language
  {any attributes with non-schema Namespace}...>
Content: ((include | import | redefine | annotation)*, (((simpleType |
complexType | group | attributeGroup) | element | attribute | notation),
annotation*)*)
</schema>
 */

use super::*;
///Contains the definition of a schema.
#[xsd_proc::xsdnode(attribute_form_default=xsdrs::Form::Unqualified, block_default= xsdrs::Block::None, element_form_default = xsdrs::Form::Unqualified, final_default = xsdrs::Final::None, defs = Default::default())]
#[derive(Debug)]
pub struct Schema {
    ///The form for attributes declared in the target namespace of this schema. The value must be one of the following strings: qualified or unqualified. The default is unqualified.
    ///
    /// This value is the global default for all attributes declared in the target namespace. Individual attributes can override this setting for their local scope using the form attribute.
    pub attribute_form_default: xsdrs::Form,
    ///The type of derivation. The blockDefault attribute sets the default value of the block attribute on element and complexType elements in the target namespace of this schema. The block attribute prevents a complex type (or element) that has the specified type of derivation from being used in place of the inherited complex type (or element).
    pub block_default: xsdrs::Block,
    ///The form for elements declared in the target namespace of this schema. The value must be one of the following strings: qualified or unqualified. The default is unqualified.
    ///
    /// This value is the global default for all elements declared in the target namespace. Individual elements can override this setting for their local scope using the form attribute.
    pub element_form_default: xsdrs::Form,
    ///The type of derivation. The finalDefault attribute sets the default value of the final attribute on element, simpleType, and complexType elements in the target namespace of this schema. The final attribute prevents the specified type of derivation of an element, simpleType,or complexType element. For element and complexType elements, this value can contain #all or a list that is a subset of extension or restriction. For simpleType elements, this value can additionally contain list and union.
    pub final_default: xsdrs::Final,
    ///The URI reference of the namespace of this schema. A prefix for the namespace can also be assigned. If no prefix is assigned, the schema components of the namespace can be used with unqualified references.
    pub target_namespace: AnyUri,
    ///The version of the schema.
    pub version: Option<xsdrs::Str>,
    ///The indicator of the language used in the contents.
    pub lang: Option<xsdrs::Str>,
    pub defs: xsdrs::HashedList<xsdrs::Definition>,
}
use crate::utils::XmlElement;
xsd_proc::const_qname!("schema", XSD);
impl<'a, R: crate::utils::ElementReader> From<XmlElement<'a, R>> for Schema {
    fn from(
        XmlElement {
            mut attrs,
            name: _,
            mut content,
        }: XmlElement<'a, R>,
    ) -> Self {
        let mut schema = Schema::new(
            attrs.extract_parsed(xsdrs::ATTRIBUTE_FORM_DEFAULT),
            attrs.extract_parsed(xsdrs::BLOCK_DEFAULT),
            attrs.extract_parsed(xsdrs::ELEMENT_FORM_DEFAULT),
            attrs.extract_parsed(xsdrs::FINAL_DEFAULT),
            attrs.extract(xsdrs::TARGET_NAMESPACE).unwrap(),
            attrs.extract(xsdrs::VERSION),
            attrs.extract(xml::LANG),
            None,
            attrs.extract(xsdrs::ID),
            None,
        );

        while let Some(global) = content.next() {
            match global.name.borrowed() {
                ELEMENT => {
                    let el = Element::from((global, &schema, true));
                    if let xsdrs::XsdDcl::Def(def) = el {
                        schema.defs.insert(xsdrs::Definition::Element(def));
                    } else {
                        unreachable!("Already check for this");
                    }
                }
                COMPLEX_TYPE => {
                    let mut def = ComplexType::from((global, &schema));
                    if def.name.name.as_ref() == "" {
                        panic!("Missing name")
                    }
                    def.name.namespace = schema.target_namespace.clone();
                    schema.defs.insert(xsdrs::Definition::ComplexType(def));
                }
                _ => {}
            }
        }
        schema
    }
}
