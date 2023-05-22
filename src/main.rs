use crate::{parser::Parser, render::Renderer};
use config::get_config;
use eyre::Result;

use slide_ui::*;

fn main() -> Result<()> {
    let args = cli::get_args();

    println!("{:?}", get_config()?);

    cli::manage_atributes(cli::separate(args.clone()).0);

    if cli::separate(args.clone()).1.len() < 2 {
        eprintln!("Expected /path/to/file which You want to open");
        return Ok(());
    }

    let content = cli::read_file(cli::separate(args).1[1].as_str())?;

    let mut parser = Parser::new(content);
    let presentation = parser.parse();

    println!("{presentation:?}");

    let mut renderer = Renderer::new(presentation)?;

    while renderer.is_running() {
        renderer.process()?;
    }

    renderer.disable_raw_mode()?;
    Ok(())
}
