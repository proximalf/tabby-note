use dirs::home_dir;
use std::env::{current_dir, temp_dir};
use std::error::Error;
use std::path::PathBuf;
use std::{fs, io::Read, process};

use crate::settings::Settings;

// mod settings;
// use settings::Settings;

/// Convert a path to absolute, this is not a catch all and will need adding to..
pub fn convert_path_to_absolute(path: PathBuf) -> Result<PathBuf, Box<dyn Error>> {
    if path.starts_with("~") {
        // Relative to home.
        let home = home_dir().expect("No home directory found");
        let path = home.join(
            path.strip_prefix("~")
                .expect("Expected path to be prepended with: `~` to reference home dir."),
        );
        return Ok(path);
    } else if path.starts_with(".") {
        // Relative to CWD.
        let cwd = current_dir()?;
        let path = cwd.join(path.strip_prefix(".").unwrap());
        return Ok(path);
    } else {
        return Ok(path);
    }
}

/// Opens editor pointing to a file in a temp directory, then returns the string as read from the file.
pub fn open_editor(
    settings: &Settings,
    edit_existing_file: bool,
) -> Result<String, Box<dyn Error>> {
    let mut temp_path = temp_dir();
    temp_path.push("takenote-editor");

    // Open existing temp file to edit
    if !edit_existing_file {
        fs::File::create(&temp_path)?;
    }

    process::Command::new(&settings.editor)
        .arg(&temp_path)
        .status()
        .expect("Something went wrong");

    let mut editable = String::new();

    fs::File::open(&temp_path)
        .expect("Could not open file")
        .read_to_string(&mut editable)?;

    // Clean up temp file
    if settings.clean_up_editor_files {
        fs::remove_file(&temp_path)?;
    }

    Ok(editable)
}
