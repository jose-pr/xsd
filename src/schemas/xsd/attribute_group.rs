/***
 * <attributeGroup
  id = ID
  name = NCName
  ref = QName
  {any attributes with non-schema Namespace...}>
Content: (annotation?), ((attribute | attributeGroup)*, anyAttribute?))
</attributeGroup>
 */
use super::*;

///Groups a set of attribute declarations so that they can be incorporated as a group for complex type definitions.
pub type AttributeGroup = xsdrs::XsdDcl<xsdrs::AttributeGroupDef>;
xsd_proc::const_qname!("attributeGroup", XSD);