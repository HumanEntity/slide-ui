use crate::{render::Renderer, parser::MdParser};
use crossterm::{Result, terminal::ClearType};

pub mod render;
pub mod event;
pub mod parser;
pub mod cli;

fn main() -> Result<()> {
    let args = cli::get_args();

    cli::manage_atributes(cli::separate(args.clone()).0);

    if cli::separate(args.clone()).1.len() < 2 {
        eprintln!("Expected /path/to/file which You want to open");
        return Ok(());
    }

    let content = cli::read_file(cli::separate(args).1[1].as_str())?;

    let mut parser = MdParser::new(content);
    let presentation = parser.parse();

    let mut renderer = Renderer::new(presentation)?;

    while renderer.is_running() {
        renderer.process()?;
    }

    renderer.disable_raw_mode()?;
    Ok(())
}
