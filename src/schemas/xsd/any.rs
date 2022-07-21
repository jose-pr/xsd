/***
 * <any
  id = ID
  maxOccurs = (nonNegativeInteger | unbounded) : 1
  minOccurs = nonNegativeInteger : 1
  namespace = "(##any | ##other) | List of (anyURI | (##targetNamespace |  ##local))) : ##any
  processContents = (lax | skip | strict) : strict
  {any attributes with non-schema Namespace...}>
Content: (annotation?)
</any>
 */
use super::*;

//Enables any element from the specified namespace(s) to appear in the containing sequence or choice element.
#[xsd_proc::xsdnode(occurs=xsdrs::Occurs::single(), namespace=xsdrs::Namespaces::Any, proccess_content= xsdrs::Validation::Strict)]
#[derive(Debug, Default)]
pub struct Any {
    pub occurs: xsdrs::Occurs,
    ///The namespaces containing the elements that can be used. If no namespace is specified, ##any is the default.
    pub namespace: xsdrs::Namespaces,
    ///An indicator of how an application or XML processor should handle validation of XML documents against the elements specified by this any element. If no processContents attribute is specified, the default is strict.
    pub proccess_content: xsdrs::Validation,
}
xsd_proc::const_qname!("any", XSD);
impl<'a, R: ElementReader> From<(XmlElement<'a, R>, &Schema)> for Any {
  fn from(
      (
          XmlElement {
              mut attrs,
              name: _,
              content:_,
          },
          _schema,
      ): (XmlElement<'a, R>, &Schema),
  ) -> Self {
      Any {
          occurs: xsdrs::Occurs::try_from(&mut attrs).unwrap(),
          id: attrs.extract(xsdrs::ID),
          annotation: None,
          namespace: xsdrs::Namespaces::Any,
          proccess_content: xsdrs::Validation::Strict
      }

  }
}
