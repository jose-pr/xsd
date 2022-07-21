use super::*;

/***
 * <element
  abstract = Boolean : false
  block = (#all | List of (extension | restriction | substitution))
  default = string
  final = (#all | List of (extension | restriction))
  fixed = string
  form = (qualified | unqualified)
  id = ID
  maxOccurs = (nonNegativeInteger | unbounded) : 1
  minOccurs = nonNegativeInteger : 1
  name = NCName
  nillable = Boolean : false
  ref = QName
  substitutionGroup = QName
  type = QName
  {any attributes with non-schema Namespace}...>
Content: (annotation?, ((simpleType | complexType)?, (unique | key |
keyref)*))
</element>
 */

pub type Element = xsdrs::XsdDcl<xsdrs::ElementDef>;
xsd_proc::const_qname!("element", XSD);

impl<'a, R: ElementReader> From<(XmlElement<'a, R>, &xsd::Schema)>
    for xsdrs::Decl<xsdrs::ElementDef>
{
    fn from(
        (
            XmlElement {
                mut attrs,
                name: _,
                content,
            },
            schema,
        ): (XmlElement<'a, R>, &xsd::Schema),
    ) -> Self {
        let decl = if let Some(name) = attrs.extract(xsdrs::NAME) {
            let name = xsd::QName::local(name);
            let type_ = if let Some(name) = attrs.extract_qname(xsdrs::TYPE, content.reader, true) {
                xsdrs::Declared::Ref(xsdrs::Ref { name })
            } else {
                xsdrs::Declared::Ref(xsdrs::Ref {
                    name: xsd::QName::local(""),
                })
            };
            let def = xsdrs::ElementDef::new(
                attrs.extract_parsed(xsdrs::ABSTRACT),
                attrs
                    .extract_parsed(xsdrs::BLOCK)
                    .unwrap_or(schema.block_default),
                attrs.extract(xsdrs::FIXED).map_or_else(
                    || {
                        attrs
                            .extract(xsdrs::DEFAULT)
                            .map(xsdrs::SimpleValueOption::Default)
                            .unwrap_or_default()
                    },
                    xsdrs::SimpleValueOption::Fixed,
                ),
                attrs.extract_qname(xsdrs::SUBSTITUTION_GROUP, content.reader, true),
                attrs
                    .extract_parsed(xsdrs::FINAL)
                    .unwrap_or(schema.final_default),
                attrs
                    .extract_parsed(xsdrs::FORM)
                    .unwrap_or(schema.element_form_default),
                attrs.extract_parsed(xsi::NIL),
                name.clone(),
                type_,
                attrs.extract(xsdrs::ID),
                None,
            );
            xsdrs::Declared::Def(def)
        } else {
            xsdrs::Declared::Ref(xsdrs::Ref {
                name: attrs
                    .extract_qname(xsdrs::REF, content.reader, true)
                    .unwrap(),
            })
        };
        let args = xsdrs::ElementArgs {
            occurs: xsdrs::Occurs::try_from(&mut attrs).unwrap(),
        };
        if !attrs.is_empty() {
            panic!("didnt process keys :{attrs:?}")
        }
        xsdrs::Decl { decl, args }
    }
}

impl<'a, R: ElementReader> From<(XmlElement<'a, R>, &Schema, bool)> for Element {
    fn from((el, schema, is_global): (XmlElement<'a, R>, &Schema, bool)) -> Self {
        let decl = xsdrs::Decl::<xsdrs::ElementDef>::from((el, schema));

        let element = if is_global {
            if let xsdrs::Decl {
                args: _,
                decl: xsdrs::Declared::Def(mut def),
            } = decl
            {
                def.name.namespace = schema.target_namespace.clone();
                xsd::Element::Def(def)
            } else {
                panic!()
            }
        } else {
            xsdrs::XsdDcl::Decl(decl)
        };
        return element;
    }
}
