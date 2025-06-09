mod validation;
mod replace;
mod remove;

pub use replace::{replace_template_variable, replace_template_variables};
pub use remove::remove_handlebars_variables; 