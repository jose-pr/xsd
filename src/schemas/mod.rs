pub(crate) use self::xsdrs::{BorrowedQname, StaticQname};
pub(crate) use crate::schemas::xsdrs::XsdNode;
pub(crate) use crate::utils::AttributesExts;
pub(crate) use crate::utils::ElementReader;
pub(crate) use crate::utils::XmlElement;
pub(crate) use std::str::FromStr;
pub(crate) use xsd_proc;
pub(crate) use xsdrs::ParsingError;

pub const XSD: &'static str = "http://www.w3.org/2001/XMLSchema";
pub const XML: &'static str = "http://www.w3.org/XML/1998/namespace";
pub const XSI: &'static str = "http://www.w3.org/2001/XMLSchema-instance";

#[inline]
pub(crate) const fn _local<'a>(name: &'a str) -> BorrowedQname<'static, 'a> {
    BorrowedQname::new("", name)
}

#[inline]
pub const fn xsd<'a>(name: &'a str) -> BorrowedQname<'static, 'a> {
    BorrowedQname::new(XSD, name)
}
#[inline]
pub const fn xsi<'a>(name: &'a str) -> BorrowedQname<'static, 'a> {
    BorrowedQname::new(XSI, name)
}
#[inline]
pub const fn xml<'a>(name: &'a str) -> BorrowedQname<'static, 'a> {
    BorrowedQname::new(XML, name)
}
pub mod xsd;
pub mod xsdrs;
pub mod xml {
    use super::*;
    xsd_proc::const_qname!("lang", XML);
}
pub mod xsi {
    use super::*;
    xsd_proc::const_qname!("nil", XSI);
}
