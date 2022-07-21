/***
 * <simpleContent
  id = ID 
  {any attributes with non-schema Namespace}...>
Content: (annotation?, (restriction | extension))
</simpleContent>
 */

use super::*;
xsd_proc::const_qname!("simpleContent", XSD);
#[xsd_proc::xsdnode]
#[derive(Debug)]
pub struct SimpleContent {
   pub content: xsdrs::ComplexTypeContent<Self>
}
