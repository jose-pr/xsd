/****
 * <extension
  base = QName
  id = ID 
  {any attributes with non-schema Namespace}...>
Content: (annotation?, ((group | all | choice | sequence)?, ((attribute |
 attributeGroup)*, anyAttribute?)))
</extension>
 */

 use super::*;

 xsd_proc::const_qname!("extension", XSD);

 #[xsd_proc::xsdnode(content=xsdrs::ElementBase::Empty)]
 #[derive(Debug)]
 ///Contains extensions on complexContent.
 pub struct Extension {
    ///The name complexType element.
    pub base: QName,
    pub content: xsdrs::ElementBase,
    pub attributes: xsdrs::List<xsdrs::AttributeBased>
 }
 
 /***
  * Simple
  * <extension
  base = QName
  id = ID 
  {any attributes with non-schema Namespace}...>
Content: (annotation?, ((attribute | attributeGroup)*, anyAttribute?))
</extension> 
  */