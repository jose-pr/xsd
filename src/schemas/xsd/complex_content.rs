/***
 * <complexContent
  id = ID 
  mixed = Boolean 
  {any attributes with non-schema Namespace}...>
Content: (annotation?,  (restriction | extension))
</complexContent>
 */

 use super::*;

 #[xsd_proc::xsdnode(mixed=false)]
 #[derive(Debug)]
 ///Contains extensions or restrictions on a complex type that contains mixed content or elements only.
 pub struct ComplexContent {
    pub content: xsdrs::ComplexTypeContent<Self>,
    ///An indicator of whether character data is allowed to appear between the child elements of this complexType element.
    /// Default is false
    pub mixed: bool
 }
 xsd_proc::const_qname!("complexContent", XSD);
 