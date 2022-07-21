use std::path::Path;
use std::str::FromStr;

use crate::_internal::*;
use crate::schemas::xsd;

pub type GenericAttributes = OderedMap<xsd::QName, Str>;

pub trait AttributesExts {
    fn extract<'a, Q>(&mut self, name: Q) -> Option<Str>
    where
        Q: AsRef<xsd::BorrowedQname<'a, 'a>>;

    fn extract_parsed<'a, Q, T>(&mut self, name: Q) -> Option<T>
    where
        Q: AsRef<xsd::BorrowedQname<'a, 'a>>,
        T: FromStr,
    {
        self.extract(name)
            .map(|v| T::from_str(&v).ok())
            .unwrap_or(None)
    }
    fn extract_into<'a, Q, T>(&mut self, name: Q) -> Option<T>
    where
        Q: AsRef<xsd::BorrowedQname<'a, 'a>>,
        T: From<Str>,
    {
        match self.extract(name) {
            Some(val) => Some(val.into()),
            None => None,
        }
    }
    fn extract_try_into<'a, Q, T>(&mut self, name: Q) -> Option<T>
    where
        Q: AsRef<xsd::BorrowedQname<'a, 'a>>,
        T: TryFrom<Str>,
    {
        match self.extract(name) {
            Some(val) => val.try_into().ok(),
            None => None,
        }
    }
    fn extract_qname<'a, Q>(
        &mut self,
        name: Q,
        reader: &impl ElementReader,
        use_default: bool,
    ) -> Option<xsd::QName>
    where
        Q: AsRef<xsd::BorrowedQname<'a, 'a>>,
    {
        self.extract(name)
            .map(|name| reader.resolve_namespace(&name, use_default))
    }
}

impl AttributesExts for GenericAttributes {
    #[inline]
    fn extract<'a, Q>(&mut self, name: Q) -> Option<Str>
    where
        Q: AsRef<xsd::BorrowedQname<'a, 'a>>,
    {
        self.remove(name.as_ref())
    }
}

#[derive(Debug)]
pub enum GenericElement {
    Text(Str),
    Element {
        attrs: OderedMap<xsd::QName, Str>,
        content: OderedMap<xsd::QName, Self>,
    },
}

pub(crate) fn parse_str(bytes: &[u8]) -> &str {
    std::str::from_utf8(bytes).unwrap()
}

pub struct XmlElement<'a, R: ElementReader>
where
    Self: 'a,
{
    pub name: xsd::QName,
    pub attrs: GenericAttributes,
    pub content: ElementContent<'a, R>,
}

pub struct ElementContent<'a, R: ElementReader> {
    pub reader: &'a mut R,
    pub done: bool,
}
impl<'a, R: ElementReader> ElementContent<'a, R> {
    pub fn next<'b>(&'b mut self) -> Option<XmlElement<'b, R>> {
        if self.done {
            return None;
        }
        loop {
            match self.reader.next()? {
                XmlElementEvent::Start(el) => {
                    return Some(el);
                }
                XmlElementEvent::End(_name) => {
                    self.done = true;
                    return None;
                }
            }
        }
    }
}

impl<'a, R: ElementReader> Drop for ElementContent<'a, R> {
    fn drop(&mut self) {
        while let Some(_) = self.next() {}
    }
}
pub enum XmlElementEvent<'a, R: ElementReader> {
    Start(XmlElement<'a, R>),
    End(xsd::QName),
}

pub trait ElementReader: Sized {
    fn next<'a>(&'a mut self) -> Option<XmlElementEvent<'a, Self>>;
    fn depth(&self) -> usize;
    fn resolve_namespace(&self, qname: &str, use_default: bool) -> xsd::QName;
}
fn _get_attrs_with_ns<'a, 'b, 'c>(
    reader: &'c quick_xml::Reader<impl std::io::BufRead>,
    ns_buf: &'a [u8],
    tag: &'b quick_xml::events::BytesStart<'b>,
) -> GenericAttributes {
    tag.attributes()
        .filter_map(|r| {
            r.map(|attr| {
                let (ns, name) = reader.attribute_namespace(attr.key, ns_buf);
                (
                    xsd::QName::from_bytes(name, ns),
                    parse_str(attr.unescape_and_decode_value(reader).unwrap().as_ref()).into(),
                )
            })
            .ok()
        })
        .collect()
}

pub struct QuickXmlElementReader<R: std::io::BufRead> {
    pub reader: quick_xml::Reader<R>,
    pub depth: usize,
    pub ns_buf: Vec<u8>,
    pub buf: Vec<u8>,
}

impl<R: std::io::BufRead> QuickXmlElementReader<R> {
    pub fn new(reader: quick_xml::Reader<R>) -> Self {
        let mut reader = reader;
        reader.expand_empty_elements(true);
        reader.check_end_names(true);
        Self {
            reader,
            depth: 0,
            ns_buf: vec![],
            buf: vec![],
        }
    }
}

impl QuickXmlElementReader<std::io::BufReader<std::fs::File>> {
    pub fn from_file(path: impl AsRef<Path>) -> quick_xml::Result<Self> {
        quick_xml::Reader::from_file(path).map(Self::new)
    }
}

impl<R: std::io::BufRead> ElementReader for QuickXmlElementReader<R> {
    fn next<'a>(&'a mut self) -> Option<XmlElementEvent<'a, Self>> {
        let Self {
            reader,
            depth,
            ns_buf,
            buf,
        } = self;

        loop {
            match reader.read_namespaced_event(buf, ns_buf) {
                Ok((ns, evt)) => match evt {
                    quick_xml::events::Event::Start(tag) => {
                        let name = xsd::QName::from_bytes(tag.local_name(), ns);
                        *depth += 1;
                        return Some(XmlElementEvent::Start(XmlElement {
                            name,
                            attrs: _get_attrs_with_ns(reader, ns_buf, &tag),
                            content: ElementContent {
                                reader: self,
                                done: false,
                            },
                        }));
                    }
                    quick_xml::events::Event::End(tag) => {
                        let name = xsd::QName::from_bytes(tag.local_name(), ns);
                        *depth -= 1;
                        return Some(XmlElementEvent::End(name));
                    }
                    quick_xml::events::Event::Empty(_) => {
                        panic!()
                    }
                    quick_xml::events::Event::Text(_) => {}
                    quick_xml::events::Event::Comment(_) => {}
                    quick_xml::events::Event::CData(_) => {}
                    quick_xml::events::Event::Decl(_) => {}
                    quick_xml::events::Event::PI(_) => todo!(),
                    quick_xml::events::Event::DocType(_) => todo!(),
                    quick_xml::events::Event::Eof => break None,
                },
                Err(_) => break None,
            }
        }
    }
    fn depth(&self) -> usize {
        self.depth
    }

    fn resolve_namespace(&self, qname: &str, use_default: bool) -> xsd::QName {
        let (ns, local) = if use_default {
            self.reader.event_namespace(qname.as_bytes(), &self.ns_buf)
        } else {
            self.reader
                .attribute_namespace(qname.as_bytes(), &self.ns_buf)
        };
        xsd::QName::from_bytes(local, ns)
    }
}
