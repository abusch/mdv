use anyhow::Context;
use markdown::ParseOptions;

use crate::renderer::RenderOptions;

mod renderer;

fn main() -> anyhow::Result<()> {
    let file = std::env::args().nth(1).context("Usage: mdv <file.md>")?;
    // let md = std::fs::read_to_string("/Users/abusch/code/ocrlabs/the-book/src/README.md")?;
    let md = std::fs::read_to_string(file).context("Failed to read markdown file")?;

    let root = markdown::to_mdast(&md, &ParseOptions::default()).unwrap();

    let out = renderer::render(&root, &RenderOptions::default());
    println!("{out}");

    Ok(())
}
