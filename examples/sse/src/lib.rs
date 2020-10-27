use localghost::dom::{Element, ElementKind};
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
    let tr = Element::new(ElementKind::Tr);
    tr.append_child(Element::with_text(ElementKind::Th, "name"));
    tr.append_child(Element::with_text(ElementKind::Th, "data"));
    table.append_child(tr);
    localghost::document().body().append_child(&table);

    // For every event in the `EventSource` add an entry to the table.
    while let Some(ev) = sse.next().await.transpose()? {
        let tr = Element::new(ElementKind::Tr);
        tr.append_child(Element::with_text(ElementKind::Td, ev.name()));
        tr.append_child(Element::with_text(ElementKind::Td, ev.data()));
        table.append_child(tr);
    };

    Ok(())
}
