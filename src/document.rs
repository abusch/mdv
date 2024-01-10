use std::{path::Path, time::Instant};

use anyhow::{Context, Result};
use log::trace;
use markdown::{mdast::Node, ParseOptions};

use crate::renderer::{self, RenderOptions};

pub struct MdDocument {
    ast: Node,
}

impl MdDocument {
    pub fn open(file: impl AsRef<Path>) -> Result<Self> {
        let start = Instant::now();
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
        Ok(Self { ast: root })
    }

    pub fn render(&self) {
        let start = Instant::now();
        let out = renderer::render(&self.ast, &RenderOptions::default());

        let lines = out.lines().collect::<Vec<_>>();
        for line in lines {
            println!("{line}");
        }

        trace!(
            "File rendered in {}ms",
            start.elapsed().as_millis() as f64 / 1000.0
        );
    }
}
