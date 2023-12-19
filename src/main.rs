use crate::{parser::Parser, render::Renderer};
use config::ConfigReader;
use eyre::Result;

use slide_ui::{*, lexer::{MdLexer, OrgLexer, Lexer}};

fn main() -> Result<()> {
    let args = cli::get_args();

    let config = ConfigReader::get_config()?;
    println!("{config:?}");

    cli::manage_atributes(cli::separate(args.clone()).0);

    if cli::separate(args.clone()).1.len() < 2 {
        eprintln!("Expected /path/to/file which You want to open");
        return Ok(());
    }

    let filename = cli::separate(args.clone()).1[1].split('/').collect::<String>().split('.').last().unwrap().to_owned();

    let content = cli::read_file(cli::separate(args).1[1].as_str())?;

    let mut lexer: Box<dyn Lexer> = match filename.as_str() {
	"md" | "markdown" => Box::new(MdLexer::new()),
	"org" => Box::new(OrgLexer::new()),
	_ => panic!("Unsupported format {filename}"),
    };


    let mut parser = Parser::new(content, config.clone());
    let presentation = parser.parse(&mut lexer);

    println!("{presentation:?}");

    let mut renderer = Renderer::new(presentation, config)?;

    while renderer.is_running() {
        renderer.process()?;
    }
    Ok(())
}
