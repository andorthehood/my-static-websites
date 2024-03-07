use std::io::Result;
use crate::file_readers::load_and_parse_markdown_files_with_front_matter_in_directory;

mod front_matter;
mod handlebars;
mod markdown;
mod liquid;
mod load_includes;
mod markdown_with_front_matter;
mod file_readers;

fn main() -> Result<()> {
    let markdown_files = load_and_parse_markdown_files_with_front_matter_in_directory("../_posts")?;

    let mut html_list = String::new();
    html_list.push_str("<ul>\n");

    for markdown in markdown_files {
        if let Some(title) = markdown.get("title") {
            html_list.push_str(&format!("  <li>{}</li>\n", title));
        }
    }

    html_list.push_str("</ul>");

    println!("{}", html_list);

    Ok(())
}