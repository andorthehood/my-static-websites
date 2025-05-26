use std::collections::HashMap;
use crate::error::MyError;

use crate::{
    handlebars::{remove_handlebars_variables, replace_template_variables},
    layout::insert_body_into_layout,
    liquid::{_if::process_liquid_conditional_tags, include::process_liquid_includes},
    markdown::markdown_to_html,
    write::write_html_to_file, // This now returns Result<(), MyError>
};

fn process_template_tags(input: &str, variables: &HashMap<String, String>) -> String {
    let mut result = input.to_string();
    let keys: Vec<String> = variables.keys().cloned().collect();
    result = process_liquid_conditional_tags(&result, &keys);
    result = replace_template_variables(&result, &variables);
    result = remove_handlebars_variables(&result);
    result
}

fn render_page(
    body: &str,
    directory: &str,
    slug: &str,
    layout: &str,
    includes: &HashMap<String, String>,
    variables: &HashMap<String, String>,
) -> Result<(), MyError> {
    let mut html = markdown_to_html(&body);
    let file_name = directory.to_string() + &slug + ".html";
    html = process_liquid_includes(&html, &includes); // This function takes HashMap, review if it needs error handling
    html = insert_body_into_layout(&layout, &html);
    html = process_template_tags(&html, &variables);
    write_html_to_file(&file_name, &html)?;
    Ok(())
}

pub fn generate_pagination_pages(
    posts_per_page: usize,
    posts: &Vec<HashMap<String, String>>,
    includes: &HashMap<String, String>,
    main_layout: &String,
    global_variables: &HashMap<String, String>,
    config: &crate::config::Config,
) -> Result<(), MyError> {
    let post_chunks: Vec<Vec<HashMap<String, String>>> = posts
        .chunks(posts_per_page)
        .map(|chunk| chunk.to_vec())
        .collect();

    for (index, chunk) in post_chunks.iter().enumerate() {
        let mut html = String::new();
        html.push_str("<div class=\"postlist\">\n");
        for post in chunk {
            html.push_str(&process_template_tags(
                &includes
                    .get("post.liquid")
                    .cloned()
                    // Consider how to handle missing "post.liquid". For now, an empty string is used.
                    // This could be a place to return an error if "post.liquid" is essential.
                    .unwrap_or_else(String::new),
                &post,
            ));
        }

        // Generate pagination links
        html.push_str("<ul class=\"pagination\">");
        if index > 0 {
            html.push_str(&format!(
                "<li><a href=\"/page{}\">🔙 Previous page</a>,&nbsp;</li>",
                index
            ));
        }

        html.push_str("<li><a href=\"/\">Index page</a>,&nbsp;</li>");

        for index in 0..post_chunks.len() {
            let url = format!("/page{}", index + 1);
            html.push_str(&format!(
                "<li><a href=\"{}\">{}</a>,&nbsp;</li>",
                url,
                index + 1
            ));
        }
        if index < post_chunks.len() - 1 {
            html.push_str(&format!(
                "<li><a href=\"/page{}\">Next page ⏭️</a></li>",
                index + 2
            ));
        }
        html.push_str("</ul>");

        html.push_str("</div>");

        let slug = format!("page{}", index + 1);

        // Ensure output directory ends with a slash for render_page path concatenation
        let output_dir_str = if config.output_dir.ends_with('/') {
            config.output_dir.clone()
        } else {
            format!("{}/", config.output_dir)
        };

        render_page(
            &html,
            &output_dir_str,
            &slug,
            &main_layout,
            &includes,
            &global_variables,
        )?;
    }
    Ok(())
}
