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
    let table = Element::new("table");
    dom::body().append(&table);

    // Create the headings
    let tr = Element::new("tr");
    tr.append(Element::with_text("th", "name"));
    tr.append(Element::with_text("th", "data"));
    table.append(tr);

    // For every event in the `EventSource` add an entry to the table.
    while let Some(ev) = sse.next().await.transpose()? {
        let tr = Element::new("tr");
        tr.append(Element::with_text("td", ev.name()));
        tr.append(Element::with_text("td", ev.data()));
        table.append(tr);
    };

    Ok(())
}
