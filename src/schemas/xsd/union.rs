/***
 * <union
  id = ID 
  memberTypes = List of QNames 
  {any attributes with non-schema Namespace}...>
Content: (annotation?, (simpleType*))
</union>
 */

use super::*;
xsd_proc::const_qname!("union", XSD);
#[xsd_proc::xsdnode]
#[derive(Debug)]
///Defines a collection of multiple simpleType definitions.
 pub struct  Union {
    ///The list of names of built-in data types or simpleType elements defined in this schema (or another schema indicated by the specified namespace). The simpleType element containing the union element is derived from the simple types specified by the memberTypes value. The values in memberTypes must be qualified names (QNames).
    /// 
    /// For simple type union definitions, the list of simple types is the union of the contents of memberTypes (which is itself a list of simple types) and each of the child simpleType element definitions under the union element. See the second example later in this topic.
    /// 
    /// The memberTypes attribute is opposite of the itemType attribute for the list element which is mutually exclusive to the simpleType element child of the list element.
    pub members: xsdrs::List<xsdrs::SimpleMemberType>
 }