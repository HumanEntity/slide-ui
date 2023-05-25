use eyre::Result;
use std::path::PathBuf;

fn main() -> Result<()> {
    #[cfg(feature = "config")]
    {
        let home = std::env!("HOME");
        let config_dir = ".config/sui";
        let res = {
            let mut path = PathBuf::new();
            path.push(home);
            path.push(config_dir);
            path.exists()
        };
        if !res {
            let user_relative_path = ".config/sui/config.toml";

            let mut config_path = PathBuf::new();
            config_path.push(home);
            config_path.push(user_relative_path);

            let os_conf = config_path.clone().into_os_string();

            save_config(&os_conf, "\n")?;
        }
    }

    Ok(())
}

fn save_config(config_path: &std::ffi::OsStr, config_content: &str) -> Result<()> {
    let parent = std::path::Path::new(&config_path).parent().unwrap();
    std::fs::create_dir_all(parent)?;
    std::fs::write(config_path, config_content)?;
    Ok(())
}
