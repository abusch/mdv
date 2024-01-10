use std::time::Instant;

use anyhow::Context;
use log::trace;
use markdown::ParseOptions;

use crate::renderer::RenderOptions;

mod renderer;
mod viewer;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let start = Instant::now();
    let file = std::env::args().nth(1).context("Usage: mdv <file.md>")?;
    // let md = std::fs::read_to_string("/Users/abusch/code/ocrlabs/the-book/src/README.md")?;
    let md = std::fs::read_to_string(file).context("Failed to read markdown file")?;
    trace!(
        "File read in {}ms",
        start.elapsed().as_nanos() as f64 / 1000.0
    );

    let start = Instant::now();
    let root = markdown::to_mdast(&md, &ParseOptions::default()).unwrap();
    trace!(
        "File parsed in {}ms",
        start.elapsed().as_millis() as f64 / 1000.0
    );

    let start = Instant::now();
    let out = renderer::render(&root, &RenderOptions::default());

    let lines = out.lines().collect::<Vec<_>>();
    for line in lines {
        println!("{line}");
    }

    trace!(
        "File rendered in {}ms",
        start.elapsed().as_millis() as f64 / 1000.0
    );

    Ok(())
}
