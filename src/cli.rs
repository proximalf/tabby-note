use clap::builder::styling;
use clap::{crate_authors, crate_name, crate_version, Arg, ArgAction, Command};

const ABOUT_STRING: &str = "
A quick terminal note taking app.
";

const STYLES: styling::Styles = styling::Styles::styled()
    .header(styling::AnsiColor::Green.on_default().bold())
    .usage(styling::AnsiColor::Green.on_default().bold())
    .literal(styling::AnsiColor::Blue.on_default().bold())
    .placeholder(styling::AnsiColor::Cyan.on_default());

pub fn build_cli() -> clap::ArgMatches {
    let matches = Command::new(crate_name!())
        .styles(STYLES)
        .version(crate_version!())
        .about(format!(
            "{name} - {author} \n {about}",
            name = crate_name!(),
            author = crate_authors!(),
            about = ABOUT_STRING
        ))
        .arg(
            Arg::new("title")
                .short('t')
                .long("title")
                .help("Set title of file"),
        )
        .arg(
            Arg::new("write")
                .short('w')
                .long("write")
                .help("Writes to file, doesn't open editor."),
        )
        .arg(
            Arg::new("open")
                .short('o')
                .long("open")
                .help("Saves a given file as if it was created."),
        )
        .arg(
            Arg::new("retry")
                .action(ArgAction::SetTrue)
                .short('r')
                .long("retry")
                .help("Opens the newly created file for editing."),
        )
        .arg(
            Arg::new("write_config")
                .action(ArgAction::SetTrue)
                .long("config")
                .help("Opens the newly created file for editing."),
        )
        .get_matches();
    matches
}
