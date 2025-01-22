use std::collections::HashMap;
use std::io::Result;

use crate::{
    handlebars::{remove_handlebars_variables, replace_template_variables},
    layout::insert_body_into_layout,
    liquid::{_if::process_liquid_conditional_tags, include::process_liquid_includes},
    markdown::markdown_to_html,
    write::write_html_to_file,
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
) -> Result<()> {
    let mut html = markdown_to_html(&body);
    let file_name = directory.to_string() + &slug + ".html";
    html = process_liquid_includes(&html, &includes);
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
) {
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

        if let Err(e) = render_page(
            &html,
            "out/",
            &slug,
            &main_layout,
            &includes,
            &global_variables,
        ) {
            eprintln!("Error rendering page {}: {}", slug, e);
        }
    }
}
