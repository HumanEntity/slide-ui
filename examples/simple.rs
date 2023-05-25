use slide_ui::{parser::Parser, render::Renderer};

use eyre::Result;

fn main() -> Result<()> {
    let content = include_str!("test.md").to_string();
    let mut parser = Parser::new(content, toml::Table::new());
    let presentation = parser.parse();
    println!("{:?}", presentation);

    let mut renderer = Renderer::new(presentation)?;

    while renderer.is_running() {
        renderer.process()?;
    }

    renderer.disable_raw_mode()?;
    Ok(())
}
