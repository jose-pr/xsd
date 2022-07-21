/***
 * <sequence
  id = ID
  maxOccurs = (nonNegativeInteger | unbounded) : 1
  minOccurs = nonNegativeInteger : 1
  {any attributes with non-schema Namespace}...>
Content: (annotation?, (element | group | choice | sequence | any)*)
</sequence>
 */
use super::*;
xsd_proc::const_qname!("sequence", XSD);

#[xsd_proc::xsdnode(occurs=xsdrs::Occurs::single())]
#[derive(Debug)]
pub struct Sequence {
    pub occurs: xsdrs::Occurs,
    pub content: xsdrs::List<xsdrs::ElementBased>,
}
impl<'a, R: ElementReader> From<(XmlElement<'a, R>, &Schema)> for Sequence {
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
            content_.push(xsdrs::ElementBased::from((child, schema)));
        }

        Sequence {
            occurs: xsdrs::Occurs::try_from(&mut attrs).unwrap(),
            content: content_.into_boxed_slice(),
            id: attrs.extract(xsdrs::ID),
            annotation: None,
        }
    }
}
