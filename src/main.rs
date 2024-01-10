use anyhow::Context;

use crate::document::MdDocument;

mod document;
mod renderer;
mod viewer;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let file = std::env::args().nth(1).context("Usage: mdv <file.md>")?;
    let md = MdDocument::open(file)?;

    md.render();

    Ok(())
}
