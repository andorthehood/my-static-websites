use crate::error::Result;
use std::fs;
use std::path::Path;

use crate::template_processors::handlebars::replace_template_variables;
use std::collections::HashMap;

pub fn load_layout(file: &str) -> Result<String> {
    let file_path = Path::new(file);
    let content = fs::read_to_string(file_path)?;
    Ok(content)
}

pub fn insert_body_into_layout(layout: &str, body: &str) -> Result<String> {
    let mut variables = HashMap::new();
    variables.insert("body".to_string(), body.to_string());
    replace_template_variables(layout, &variables)
}
