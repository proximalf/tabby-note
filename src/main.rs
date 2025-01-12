use chrono::Utc;
use std::{error::Error, fs, path::PathBuf};

mod cli;
mod settings;
mod templating;
mod utils;

use cli::build_cli;
use settings::get_config_file;
use templating::{generate_template, TemplateContext};
use utils::{convert_path_to_absolute, open_editor};

const SAVEFILE_FORMAT: &str = "md";

fn main() -> Result<(), Box<dyn Error>> {
    let matches = build_cli();

    println!("Tabby-note!");

    let write_config = matches.get_one::<bool>("write_config").unwrap_or(&false);
    // Save directory
    let config = get_config_file(write_config)?;

    if *write_config {
        // If writing config end here.
        return Ok(());
    }

    // Title
    let default_title = String::from("");
    let title = matches
        .get_one::<String>("title")
        .unwrap_or(&default_title)
        .clone();

    // Get file name
    let now = Utc::now();
    let formatted_filename = if title.is_empty() {
        format!(
            "{}.{}",
            now.format(&config.filename_format),
            SAVEFILE_FORMAT
        )
    } else {
        format!(
            "{} - {}.{}",
            now.format(&config.filename_format),
            title,
            SAVEFILE_FORMAT
        )
    };
    let savepath = config.save_path.join(&formatted_filename);

    let savepath = if savepath.is_relative() {
        convert_path_to_absolute(savepath)?
    } else {
        savepath
    };

    // Copy opened file
    if let Some(open_file) = matches.get_one::<String>("open") {
        let filepath = convert_path_to_absolute(PathBuf::from(open_file))?;

        fs::copy(filepath.as_path(), &savepath)
            .expect(format!("Unable to open and copy file: {:#?}", &filepath).as_str());

        println!("File copied to: {:#?}", &savepath);
        return Ok(());
    }

    let mut editable = String::new();
    // Check write arg else open editor
    if let Some(write) = matches.get_one::<String>("write") {
        editable.insert_str(0, write.as_str());
    }

    if editable.is_empty() {
        let edit_existing_file = matches.get_one::<bool>("retry").unwrap_or(&false);
        let editor_content = open_editor(&config, *edit_existing_file)?;

        editable.insert_str(0, editor_content.as_str());

        if editable.is_empty() {
            println!("Nothing saved to document.");
            return Ok(());
        }
    }

    let default_template = convert_path_to_absolute(config.default_template)?;

    if default_template.exists() {
        let context = TemplateContext::new(title, editable);
        let content = generate_template(&default_template, context)?;

        fs::write(&savepath, content)
            .expect(format!("Error when writing file! : {:#?}", savepath).as_str());
    } else {
        fs::write(&savepath, editable)
            .expect(format!("Error when writing file! : {:#?}", savepath).as_str());
    }

    println!("File written to path: {}", savepath.display());
    Ok(())
}
