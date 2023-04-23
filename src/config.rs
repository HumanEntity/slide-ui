
use toml::Table;
use eyre::Result;

pub fn get_config() -> Result<Table> {
    let home = std::env!("HOME");
    let content = std::fs::read_to_string(home.to_owned() + "/.config/sui/config.toml")?;
    Ok(content.parse::<Table>()?)
}
