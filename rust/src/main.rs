use std::io::Result;
use crate::file_readers::load_and_parse_markdown_files_with_front_matter_in_directory;
use crate::handlebars::replace_template_variables;
use crate::load_includes::load_liquid_includes;
use crate::layout::load_layout;
use crate::layout::insert_body_into_layout;
use crate::write::write_html_to_file;
use crate::liquid::process_liquid_includes;

mod front_matter;
mod handlebars;
mod markdown;
mod liquid;
mod load_includes;
mod markdown_with_front_matter;
mod file_readers;
mod layout;
mod write;

fn main() -> Result<()> {
    let posts = load_and_parse_markdown_files_with_front_matter_in_directory("../_posts")?;
    // let pages = load_and_parse_markdown_files_with_front_matter_in_directory("../_pages")?;
    let includes = load_liquid_includes("../_includes");
    let main_layout = load_layout("../_layouts/main.html")?;
    
    // Generate index.html
    let list_item_template = includes.get("list_item.liquid").cloned().unwrap_or_else(String::new);
    let mut html_list = String::new();
    html_list.push_str("<ul>\n");
    for post in &posts {
        html_list.push_str(&replace_template_variables(&list_item_template, &post ));
    }
    html_list.push_str("</ul>");
    let html = insert_body_into_layout(&main_layout, &html_list);
    write_html_to_file(&"out/index.html", &html)?;

    // Generate posts
    for post in &posts {
        let content = post.get("content").cloned().unwrap_or_else(String::new);
        let slug = post.get("slug").cloned().unwrap_or_else(String::new);
        let mut html = insert_body_into_layout(&main_layout, &content);
        let file_name = "out/posts/".to_string() + &slug + ".html";
        html = process_liquid_includes(&html, &includes);
        write_html_to_file(&file_name, &html)?;
    }

    Ok(())
}