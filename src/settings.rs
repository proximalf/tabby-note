use config::{Config, File, FileFormat};
use dirs;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::PathBuf;
use std::{env::var, fs};
use toml::to_string_pretty;

const CONFIG_FILENAME: &str = "tn/takenote.toml";

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub save_path: PathBuf,
    pub editor: String,
    pub filename_format: String,
    pub default_template: PathBuf,
    pub clean_up_editor_files: bool,
}

/// Get Config file, if `write_config` is `true` and config file doesn't exist, then the file
/// will be written, else uses defaults
pub fn get_config_file(write_config: &bool) -> Result<Settings, Box<dyn Error>> {
    let config_path = dirs::config_dir()
        .map(|mut path| {
            path.push(CONFIG_FILENAME);
            path
        })
        .unwrap();

    let mut config = Config::builder()
        .set_default("save_path", "~/Documents")?
        .set_default("editor", var("EDITOR").unwrap())?
        .set_default("filename_format", "%y%m_%d%H%M")?
        .set_default("default_template", "default")?
        .set_default("clean_up_editor_files", false)?;

    if config_path.exists() {
        let config_file = File::new(config_path.as_path().to_str().unwrap(), FileFormat::Toml);
        config = config.add_source(config_file);
    }

    let config = config.build()?;

    let settings = config.try_deserialize::<Settings>()?;

    if !config_path.exists() & write_config {
        let config_toml = to_string_pretty(&settings)?;
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&config_path, config_toml)?;
        println!("Config file created! {:#?}", config_path);
    }

    Ok(settings)
}
