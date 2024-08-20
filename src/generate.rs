use crate::file_copier::copy_file_with_versioning;
use crate::file_readers::load_and_parse_markdown_files_with_front_matter_in_directory;
use crate::handlebars::remove_handlebars_variables;
use crate::handlebars::replace_template_variable;
use crate::handlebars::replace_template_variables;
use crate::layout::insert_body_into_layout;
use crate::layout::load_layout;
use crate::liquid::process_liquid_includes;
use crate::load_includes::load_liquid_includes;
use crate::markdown::markdown_to_html;
use crate::write::write_html_to_file;
use std::collections::HashMap;
use std::io::Result;

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
    html = replace_template_variables(&html, &variables);
    html = remove_handlebars_variables(&html);
    write_html_to_file(&file_name, &html)?;

    Ok(())
}

pub fn generate() -> Result<()> {
    let css_file_name = copy_file_with_versioning("./style.css", "./out/")?;
    let posts = load_and_parse_markdown_files_with_front_matter_in_directory("./_posts")?;
    let pages = load_and_parse_markdown_files_with_front_matter_in_directory("./_pages")?;
    let includes = load_liquid_includes("./_includes");

    let mut global_variables = HashMap::new();
    global_variables.insert(
        "title".to_string(),
        "Andor Polgar's Visual Journal".to_string(),
    );

    let main_layout_template = load_layout("./_layouts/main.html")?;
    let mut main_layout_variables = HashMap::new();
    main_layout_variables.insert("css_file_name".to_string(), css_file_name);
    let mut main_layout = replace_template_variables(&main_layout_template, &main_layout_variables);

    // Generate pagination pages
    let posts_per_page = 5;
    let post_chunks: Vec<Vec<HashMap<String, String>>> = posts
        .chunks(posts_per_page)
        .map(|chunk| chunk.to_vec())
        .collect();

    for (index, chunk) in post_chunks.iter().enumerate() {
        let mut html = String::new();
        html.push_str("<div class=\"postlist\">\n");
        for post in chunk {
            html.push_str(&replace_template_variables(
                &includes
                    .get("post.liquid")
                    .cloned()
                    .unwrap_or_else(String::new),
                &post,
            ));
        }
        html.push_str("</div>");
 
        let slug = if index == 0 {
            "index".to_string()
        } else {
            format!("page{}", index + 1)
        };

        render_page(
            &html,
            "out/",
            &slug,
            &main_layout,
            &includes,
            &global_variables,
        )?;
    }

    // Generate index page
    let list_item_template = includes
        .get("list_item.liquid")
        .cloned()
        .unwrap_or_else(String::new);
    let mut html_list = String::new();
    html_list.push_str("<ul class=\"postlist\">\n");
    for post in &posts {
        html_list.push_str(&replace_template_variables(&list_item_template, &post));
    }
    html_list.push_str("</ul>");
    let mut html = insert_body_into_layout(&main_layout, &html_list);
    html = replace_template_variable(&html, "title", "Andor Polgar's Visual Journal");
    html = remove_handlebars_variables(&html);
    write_html_to_file(&"out/index.html", &html)?;

    // Generate posts
    for post in &posts {
        global_variables.insert(
            "title".to_string(),
            post.get("title")
                .cloned()
                .unwrap_or_else(String::new)
                .to_owned()
                + " - "
                + "Andor Polgar's Visual Journal",
        );

        let slug = post.get("slug").cloned().unwrap_or_else(String::new);
        let pathname: String = "posts/".to_owned() + &slug;
        main_layout_variables.insert("pathname".to_string(), pathname);
        main_layout = replace_template_variables(&main_layout_template, &main_layout_variables);

        render_page(
            &post.get("content").map(|s| s.as_str()).unwrap_or(""),
            "out/posts/",
            &post.get("slug").map(|s| s.as_str()).unwrap_or(""),
            &main_layout,
            &includes,
            &global_variables,
        )?;
    }

    // Generate pages
    for page in &pages {
        global_variables.insert(
            "title".to_string(),
            page.get("title")
                .cloned()
                .unwrap_or_else(String::new)
                .to_owned()
                + " - "
                + "Andor Polgar's Visual Journal",
        );

        let slug = page.get("slug").cloned().unwrap_or_else(String::new);
        main_layout_variables.insert("pathname".to_string(), slug);
        main_layout = replace_template_variables(&main_layout_template, &main_layout_variables);

        render_page(
            &page.get("content").map(|s| s.as_str()).unwrap_or(""),
            "out/posts/",
            &page.get("slug").map(|s| s.as_str()).unwrap_or(""),
            &main_layout,
            &includes,
            &global_variables,
        )?;
    }

    Ok(())
}
