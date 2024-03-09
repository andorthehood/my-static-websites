use std::io::Result;
use crate::file_readers::load_and_parse_markdown_files_with_front_matter_in_directory;
use crate::handlebars::replace_template_variables;
use crate::load_includes::load_liquid_includes;

mod front_matter;
mod handlebars;
mod markdown;
mod liquid;
mod load_includes;
mod markdown_with_front_matter;
mod file_readers;

fn main() -> Result<()> {
    let posts = load_and_parse_markdown_files_with_front_matter_in_directory("../_posts")?;
    let includes = load_liquid_includes("../_includes");
    let list_item_template = includes.get("list_item").cloned().unwrap_or_else(String::new);

    let mut html_list = String::new();
    html_list.push_str("<ul>\n");

    for post in posts {
        html_list.push_str(&replace_template_variables(&list_item_template, &post ));
    }

    html_list.push_str("</ul>");

    println!("{}", html_list);

    Ok(())
}