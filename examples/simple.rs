use slide_ui::{parser::Parser, render::Renderer};

use eyre::Result;

fn main() -> Result<()> {
    let mut parser = Parser::new("# test\n".to_string());
    let presentation = parser.parse();
    println!("{:?}", presentation);

    let mut renderer = Renderer::new(presentation)?;

    while renderer.is_running() {
        renderer.process()?;
    }

    renderer.disable_raw_mode()?;
    Ok(())
}
