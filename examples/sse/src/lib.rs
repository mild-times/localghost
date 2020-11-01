use localghost::dom::{self, Element, ElementKind};
use localghost::prelude::*;
use localghost::net::EventSource;

use futures::stream::StreamExt;
use std::io;

#[localghost::main]
async fn main() -> io::Result<()> {
    // Connect the `EventSource`.
    let interests = ["fruit"];
    let mut sse = EventSource::connect("http://localhost:8081/sse", &interests).await?;

    // Create a table
    let table = Element::new(ElementKind::Table);
    dom::body().append(&table);

    // Create the headings
    let tr = Element::new(ElementKind::Tr);
    tr.append(Element::with_text(ElementKind::Th, "name"));
    tr.append(Element::with_text(ElementKind::Th, "data"));
    table.append(tr);

    // For every event in the `EventSource` add an entry to the table.
    while let Some(ev) = sse.next().await.transpose()? {
        let tr = Element::new(ElementKind::Tr);
        tr.append(Element::with_text(ElementKind::Td, ev.name()));
        tr.append(Element::with_text(ElementKind::Td, ev.data()));
        table.append(tr);
    };

    Ok(())
}
