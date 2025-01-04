use chrono::{serde::ts_seconds, DateTime, Utc};
use minijinja::{path_loader, Environment};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateContext {
    title: String,
    content: String,
    #[serde(with = "ts_seconds")]
    now: DateTime<Utc>,
}

impl TemplateContext {
    pub fn new(title: String, content: String) -> Self {
        Self {
            title,
            content,
            now: Utc::now(),
        }
    }
}

// Render template from a given path.
pub fn generate_template(
    template_path: &PathBuf,
    context: TemplateContext,
) -> Result<String, Box<dyn Error>> {
    let template_directory = template_path.parent().unwrap();

    let mut environment = Environment::new();
    environment.set_loader(path_loader(template_directory.to_str().unwrap()));
    minijinja_contrib::add_to_environment(&mut environment);

    let template =
        environment.get_template(template_path.file_name().unwrap().to_str().unwrap())?;

    let rendered = template.render(&context)?;

    Ok(rendered)
}
