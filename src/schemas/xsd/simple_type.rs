 /***
  * <simpleType
  final = (#all | (list | union | restriction)) 
  id = ID 
  name = NCName 
  {any attributes with non-schema Namespace}...>
Content: (annotation?, (restriction | list | union))
</simpleType>
  */
  use super::*;
  xsd_proc::const_qname!("simpleType", XSD);
  #[xsd_proc::xsdnode]
  #[derive(Debug)]
  pub struct SimpleType {
      pub name: QName,
      pub final_: xsdrs::Form,
      pub content: xsdrs::SimpleTypeContent
  }