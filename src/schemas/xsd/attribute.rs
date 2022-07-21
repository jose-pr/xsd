use super::{*, xsdrs::XsdDcl};

/****
 * <attribute
  default = string
  fixed = string
  form = (qualified | unqualified)
  id = ID
  name = NCName
  ref = QName
  type = QName
  use = (optional | prohibited | required): optional
  {any attributes with non-schema Namespace...}>
Content: (annotation?, (simpleType?))
</attribute>
 */

//Attribute Declaration;
pub type Attribute = XsdDcl<xsdrs::AttributeDef>;
xsd_proc::const_qname!("attribute", XSD);