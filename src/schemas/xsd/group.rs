/***
 * Groups a set of element declarations so that they can be incorporated as a group into complex type definitions.


Copy
<group
  name= NCName
  id = ID
  maxOccurs = (nonNegativeInteger | unbounded) : 1
  minOccurs = nonNegativeInteger : 1
  name = NCName
  ref = QName
  {any attributes with non-schema Namespace}...>
Content: (annotation?, (all | choice | sequence))
</group>
 */

use super::*;
xsd_proc::const_qname!("group", XSD);
///Groups a set of element declarations so that they can be incorporated as a group into complex type definitions;
pub type Group = xsdrs::XsdDcl<xsdrs::ElementGroupDef>;
impl<'a, R: ElementReader> From<(XmlElement<'a, R>, &xsd::Schema)>
    for xsdrs::Decl<xsdrs::ElementGroupDef>
{
    fn from(
        (
            XmlElement {
                mut attrs,
                name: _,
                mut content,
            },
            schema,
        ): (XmlElement<'a, R>, &xsd::Schema),
    ) -> Self {
        let decl = if let Some(name) = attrs.extract(xsdrs::NAME) {
            let name = xsd::QName::local(name);
            let grouped = content.next().unwrap();
            let def = xsdrs::ElementGroupDef::new(
                name,
                xsdrs::Grouped::from((grouped, schema)),
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

impl<'a, R: ElementReader> From<(XmlElement<'a, R>, &Schema, bool)> for Group {
    fn from((el, schema, is_global): (XmlElement<'a, R>, &Schema, bool)) -> Self {
        let decl = xsdrs::Decl::<xsdrs::ElementGroupDef>::from((el, schema));

        let element = if is_global {
            if let xsdrs::Decl {
                args: _,
                decl: xsdrs::Declared::Def(mut def),
            } = decl
            {
                def.name.namespace = schema.target_namespace.clone();
                Self::Def(def)
            } else {
                panic!()
            }
        } else {
            Self::Decl(decl)
        };
        return element;
    }
}
