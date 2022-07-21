use super::*;
/***
 * 
<anyAttribute
  id = ID 
  namespace = ((##any | ##other) | List of (anyURI | (##targetNamespace | ##local))) : ##any 
  processContents = (lax | skip | strict): strict 
  {any attributes with non-schema Namespace...}>
Content: (annotation?)
</anyAttribute>
 */

#[xsd_proc::xsdnode(namespace=xsdrs::Namespaces::Any)]
#[derive(Debug)]
///Enables any attribute from the specified namespace(s) to appear in the containing complexType element or in the containing attributeGroup element.
pub struct AnyAttribute {
    ///The namespace containing the attributes that can be used. If no namespace is specified, ##any is the default. If the namespace is specified, it must one of the following.
    pub namespace: xsdrs::Namespaces,
    ///An indicator of how an application or XML processor should handle validation of XML documents against the attributes specified by this anyAttribute element.
    pub proccess_content: xsdrs::Validation,
}

xsd_proc::const_qname!("anyAttribute", XSD);