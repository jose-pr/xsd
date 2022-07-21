/***
 * <list
  id = ID 
  itemType = QName 
  {any attributes with non-schema Namespace}...>
Content: (annotation?, (simpleType?))
</list>
 */

 use super::*;
 xsd_proc::const_qname!("list", XSD);
 #[xsd_proc::xsdnode]
 #[derive(Debug)]
 pub struct  List {
    ///he name of a built-in data type or simpleType element defined in this schema (or another schema indicated by the specified namespace). The simpleType element containing the list element is derived from the simple type specified by the list value. The list value must be a qualified name (QName).
    pub item_type: Box<xsdrs::SimpleMemberType>,    
 }