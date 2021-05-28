use std::collections::HashMap;
use std::io::Write;
use thoth_api::errors::ThothResult;
use thoth_client::Work;
use xml::writer::events::StartElementBuilder;
use xml::writer::{EventWriter, Result as XmlResult, XmlEvent};

pub(crate) fn write_element_block<W: Write, F: Fn(&mut EventWriter<W>)>(
    element: &str,
    w: &mut EventWriter<W>,
    f: F,
) -> XmlResult<()> {
    write_full_element_block(element, None, None, w, f)
}

pub(crate) fn write_full_element_block<W: Write, F: Fn(&mut EventWriter<W>)>(
    element: &str,
    ns: Option<HashMap<String, String>>,
    attr: Option<HashMap<&str, &str>>,
    w: &mut EventWriter<W>,
    f: F,
) -> XmlResult<()> {
    let mut event_builder: StartElementBuilder = XmlEvent::start_element(element);

    if let Some(ns) = ns {
        for (k, v) in ns.iter() {
            event_builder = event_builder.ns(k, v);
        }
    }

    if let Some(attr) = attr {
        for (k, v) in attr.iter() {
            event_builder = event_builder.attr(*k, *v);
        }
    }

    let mut event: XmlEvent = event_builder.into();
    w.write(event)?;
    f(w);
    event = XmlEvent::end_element().into();
    w.write(event)
}

pub(crate) trait XmlSpecification {
    fn generate(self, work: Work) -> ThothResult<String>;

    fn handle_event<W: Write>(w: &mut EventWriter<W>, work: &Work) -> XmlResult<()>;
}

pub(crate) trait XmlElement<T: XmlSpecification> {
    const ELEMENT: &'static str = "";

    fn value(&self) -> &'static str;

    fn xml_element<W: Write>(&self, w: &mut EventWriter<W>) -> XmlResult<()> {
        write_element_block(Self::ELEMENT, w, |w| {
            w.write(XmlEvent::Characters(self.value())).ok();
        })
    }
}

mod onix3_oapen;
mod onix3_project_muse;
pub(crate) use onix3_oapen::Onix3Oapen;
pub(crate) use onix3_project_muse::Onix3ProjectMuse;
