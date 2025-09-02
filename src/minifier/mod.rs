pub mod css;
pub mod html;
pub mod js;

// Re-export the trait implementations and functions
pub use css::{CssMinifier, minify_css};
pub use html::{HtmlMinifier, minify_html};
pub use js::{JsMinifier, minify_js};
