use ::xsd::{
    schemas::*,
    utils::{ElementReader, QuickXmlElementReader, XmlElementEvent},
};

fn main() {
    let mut reader = QuickXmlElementReader::from_file("test/basic.xsd").unwrap();
    while let Some(XmlElementEvent::Start(root)) = reader.next() {
        let schema = xsd::Schema::from(root);
        println!("{schema:#?}");
    }
}