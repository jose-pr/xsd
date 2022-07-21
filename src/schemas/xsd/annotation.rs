/***
 <annotation
  id = ID 
  {any attributes with non-schema Namespace}...>
Content: (appinfo | documentation)*
</annotation>  
 
 */

use super::*;
///Defines an annotation.
/// 
/// # Example
/// XSD
/// ```xml
/// <xs:simpleType name="northwestStates">
/// <xs:annotation>
/// <xs:documentation>States in the Pacific Northwest of US
/// </xs:documentation>
/// </xs:annotation>
/// <xs:restriction base="xs:string">
/// <xs:enumeration value='WA'>
/// <xs:annotation>
/// <xs:documentation>Washington</xs:documentation>
/// </xs:annotation>
/// </xs:enumeration>
/// <xs:enumeration value='OR'/>
/// <xs:annotation>
/// <xs:documentation>Oregon</xs:documentation>
/// </xs:annotation>
/// </xs:enumeration>
/// <xs:enumeration value='ID'/>
/// <xs:annotation>
/// <xs:documentation>Idaho</xs:documentation>
/// </xs:annotation>
/// </xs:enumeration>
/// </xs:restriction>
/// </xs:simpleType> */
/// ```
#[xsd_proc::with_id]
#[derive(Debug)]
pub struct Annotation {
    pub content: xsdrs::List<xsdrs::AnnotationContent>
}
xsd_proc::const_qname!("annotation", XSD);
