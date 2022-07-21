use super::*;

/****
 * <choice
  id = ID 
  maxOccurs= (nonNegativeInteger | unbounded) : 1
  minOccurs= nonNegativeInteger : 1 
  {any attributes with non-schema Namespace}...>
Content: (annotation?, (element | group | choice | sequence | any)*)
</choice>
 */


///Allows one and only one of the elements contained in the selected group to be present within the containing element.
#[xsd_proc::xsdnode(occurs=xsdrs::Occurs::single())]
#[derive(Debug)]
 pub struct  Choice {
    pub occurs: xsdrs::Occurs,
    pub options: xsdrs::List<xsdrs::ElementBased>
 }
 xsd_proc::const_qname!("choice", XSD);
 impl<'a, R: ElementReader> From<(XmlElement<'a, R>, &Schema)> for Choice {
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

       Choice {
           occurs: xsdrs::Occurs::try_from(&mut attrs).unwrap(),
           options: content_.into_boxed_slice(),
           id: attrs.extract(xsdrs::ID),
           annotation: None,
       }
   }
}
