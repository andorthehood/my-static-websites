use std::fs;
use std::path::Path;
use crate::error::MyError;

use crate::handlebars::replace_template_variable;

pub fn load_layout(file: &str) -> Result<String, MyError> {
    let file_path = Path::new(file);
    let content = fs::read_to_string(file_path)?;
    Ok(content)
}

pub fn insert_body_into_layout(layout: &str, body: &str) -> String {
    let result = replace_template_variable(layout, &"body", body);
    result
}
