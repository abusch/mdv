use markdown::mdast::{
    BlockQuote, Code, Emphasis, Heading, InlineCode, Link, Node, Paragraph, Strong, Text,
};
use nu_ansi_term::{Color, Style};
use textwrap::{fill, termwidth, Options};

pub struct RenderOptions {
    width: usize,
    styles: RenderStyles,
}

impl RenderOptions {
    pub fn wrap(&self, s: impl AsRef<str>) -> String {
        fill(s.as_ref(), self.opt())
    }

    fn opt(&self) -> Options {
        Options::new(self.width)
    }
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            width: termwidth().min(120),
            styles: RenderStyles::default(),
        }
    }
}

pub struct RenderStyles {
    heading_style: Style,
    emphasis_style: Style,
    bold_style: Style,
    link_style: Style,
    inline_code_style: Style,
    code_block_border_style: Style,
}

impl Default for RenderStyles {
    fn default() -> Self {
        Self {
            heading_style: Style::default().fg(Color::Purple).bold(),
            emphasis_style: Style::default().italic(),
            bold_style: Style::default().bold(),
            link_style: Style::default().fg(Color::Blue).underline(),
            inline_code_style: Style::default().fg(Color::LightRed).on(Color::DarkGray),
            code_block_border_style: Style::default().fg(Color::LightRed),
        }
    }
}

pub fn render(node: &Node, opt: &RenderOptions) -> String {
    match node {
        Node::Root(root) => render_nodes(&root.children, "\n", opt),
        Node::Link(link) => render_link(link, opt),
        Node::Heading(heading) => render_heading(heading, opt),
        Node::Paragraph(para) => render_paragraph(para, opt),
        Node::Text(text) => render_text(text, opt),
        Node::Emphasis(emph) => render_emphasis(emph, opt),
        Node::Strong(strong) => render_strong(strong, opt),
        Node::Break(_) => "\n".to_string(),
        Node::BlockQuote(quote) => render_blockquote(quote, opt),
        Node::InlineCode(code) => render_inline_code(code, opt),
        Node::Code(code) => render_code(code, opt),
        _ => {
            /* not supported yet */
            String::default()
        }
    }
}

pub fn render_heading(heading: &Heading, opt: &RenderOptions) -> String {
    let content = render_nodes(&heading.children, "", opt);
    let content = format!("{} {content}", "#".repeat(heading.depth as usize));
    let content = opt.styles.heading_style.paint(&content).to_string();
    format!("{}\n", opt.wrap(content))
}

pub fn render_link(link: &Link, opt: &RenderOptions) -> String {
    opt.styles.link_style.paint(&link.url).to_string()
}

pub fn render_text(text: &Text, _opt: &RenderOptions) -> String {
    text.value.clone()
}

pub fn render_emphasis(emph: &Emphasis, opt: &RenderOptions) -> String {
    let content = render_nodes(&emph.children, "", opt);
    opt.styles.emphasis_style.paint(&content).to_string()
    // format!("_{content}_")
}

pub fn render_strong(strong: &Strong, opt: &RenderOptions) -> String {
    let content = render_nodes(&strong.children, "", opt);
    opt.styles.bold_style.paint(&content).to_string()
}

pub fn render_paragraph(para: &Paragraph, opt: &RenderOptions) -> String {
    let content = render_nodes(&para.children, "", opt);
    format!("{}\n", opt.wrap(content))
}

pub fn render_inline_code(code: &InlineCode, opt: &RenderOptions) -> String {
    opt.styles.inline_code_style.paint(&code.value).to_string()
}

pub fn render_blockquote(quote: &BlockQuote, opt: &RenderOptions) -> String {
    let content = render_nodes(&quote.children, "", opt);
    format!(
        "{}\n",
        fill(
            &content,
            Options::with_termwidth()
                .initial_indent("▎ ")
                .subsequent_indent("▎ ")
        )
    )
}

pub fn render_code(code: &Code, opt: &RenderOptions) -> String {
    let before = format!(
        "{} {}",
        opt.styles.code_block_border_style.paint("╭"),
        opt.styles
            .bold_style
            .paint(code.lang.as_deref().unwrap_or_default())
    );
    let after = opt.styles.code_block_border_style.paint("╰");
    let border = opt.styles.code_block_border_style.paint("│ ").to_string();
    let content = fill(
        &code.value,
        Options::with_termwidth()
            .initial_indent(&border)
            .subsequent_indent(&border),
    );
    format!("{before}\n{}\n{after}\n", &content)
}

pub fn render_nodes(nodes: &[Node], sep: &str, opt: &RenderOptions) -> String {
    let content = nodes.iter().map(|n| render(n, opt)).collect::<Vec<_>>();
    content.join(sep)
}
