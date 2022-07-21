use super::*;
/***
 * <appinfo
  source = anyURI>
Content: ({any})*
</appinfo> 
 */

///Specifies information to be used by applications within an annotation element.
#[derive(Debug)]
pub struct AppInfo {
    ///The source of the application information.
    pub source: Option<AnyUri>,
    pub content: crate::utils::GenericElement,
}
xsd_proc::const_qname!("appInfo", XSD);