/***
 * ComplextContent
 * <restriction
  base = QName
  id = ID
  {any attributes with non-schema Namespace}...>
Content: (annotation?, (group | all | choice | sequence)?, ((attribute |
attributeGroup)*, anyAttribute?))
</restriction>
 */

/***
 * SimpleContent
 * <restriction
  base = QName 
  id = ID 
  {any attributes with non-schema Namespace}...>
Content: (annotation?, (simpleType?, (minExclusive | minInclusive | 
maxExclusive | maxInclusive | totalDigits |fractionDigits | length | 
minLength | maxLength | enumeration | whiteSpace | pattern)*)?, 
((attribute | attributeGroup)*, anyAttribute?))
</restriction>
 */

 /****
  * SimpleType
  * <restriction
  base = QName 
  id = ID 
  {any attributes with non-schema Namespace}...>
Content: (annotation?, (simpleType?, (minExclusive | minInclusive | 
maxExclusive | maxInclusive | totalDigits |fractionDigits | length | 
minLength | maxLength | enumeration | whiteSpace | pattern)*))
</restriction>
  */
use super::*;
xsd_proc::const_qname!("restriction", XSD);
#[xsd_proc::xsdnode]
#[derive(Debug)]
/// Defines constraints on a complexContent definition.
pub struct Restriction<T:xsdrs::Restriction> {
    pub base: QName,
    pub restriction: <T as xsdrs::Restriction>::Restriction
}
