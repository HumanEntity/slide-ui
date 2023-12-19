use crate::{parser::Parser, render::Renderer};
use config::ConfigReader;
use eyre::Result;

use slide_ui::{*, lexer::MdLexer};

fn main() -> Result<()> {
    let args = cli::get_args();

    let config = ConfigReader::get_config()?;
    println!("{config:?}");

    cli::manage_atributes(cli::separate(args.clone()).0);

    if cli::separate(args.clone()).1.len() < 2 {
        eprintln!("Expected /path/to/file which You want to open");
        return Ok(());
    }

    let content = cli::read_file(cli::separate(args).1[1].as_str())?;
    
    let mut lexer = MdLexer::new();

    let mut parser = Parser::new(content, config.clone());
    let presentation = parser.parse(&mut lexer);

    println!("{presentation:?}");

    let mut renderer = Renderer::new(presentation, config)?;

    while renderer.is_running() {
        renderer.process()?;
    }
    Ok(())
}
