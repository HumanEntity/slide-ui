use eyre::Result;
use toml::Table;

use crate::cli::read_file;

pub struct ConfigReader;

impl ConfigReader {
    pub fn get_config() -> Result<Table> {
        let home = std::env!("HOME");
        let path = home.to_owned() + "/.config/sui/config.toml";
        let content = read_file(&path)?;
        // let content = std::fs::read_to_string(home.to_owned() + "/.config/sui/config.toml")?;
        Ok(content.parse::<Table>()?)
    }
}
