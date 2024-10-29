use crate::file_copier::copy_file_with_versioning;
use crate::file_readers::load_and_parse_markdown_files_with_front_matter_in_directory;
use crate::handlebars::remove_handlebars_variables;
use crate::handlebars::replace_template_variable;
use crate::handlebars::replace_template_variables;
use crate::layout::insert_body_into_layout;
use crate::layout::load_layout;
use crate::liquid::_if::process_liquid_conditional_tags;
use crate::liquid::include::process_liquid_includes;
use crate::load_includes::load_liquid_includes;
use crate::markdown::markdown_to_html;
use crate::write::write_html_to_file;
use std::collections::HashMap;
use std::io::Result;
use std::time::{SystemTime, UNIX_EPOCH};

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

pub fn generate() -> Result<()> {
    // Get the current system time
    let now = SystemTime::now();
    let duration_since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let generated_date = duration_since_epoch.as_secs().to_string();
    
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
    main_layout_variables.insert("generated_date".to_string(), generated_date);
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

    // Group posts by year
    let mut posts_by_year: HashMap<String, Vec<HashMap<String, String>>> = HashMap::new();
    for post in &posts {
        if let Some(date_str) = post.get("date") {
            let year = &date_str[0..4]; // Extract the first 4 characters as the year
            posts_by_year
                .entry(year.to_string())
                .or_default()
                .push(post.clone());
        }
    }

    // Collect and sort the years in descending order
    let mut years: Vec<String> = posts_by_year.keys().cloned().collect();
    years.sort_by(|a, b| b.cmp(a));

    let list_item_template = includes
        .get("list_item.liquid")
        .cloned()
        .unwrap_or_else(String::new);
    let mut html_list = String::new();
    html_list.push_str("<p>Hi there! 👋 My name is Andor Polgar. This is my personal website. Here, you'll find my photography projects and random snapshots from my life.</p><ul class=\"postlist\">\n");

    for year in years {
        if let Some(posts) = posts_by_year.get(&year) {
            html_list.push_str(
                &includes
                    .get(&format!("{}.liquid", year))
                    .cloned()
                    .unwrap_or_else(String::new),
            );
            html_list.push_str("<ul class=\"postlist\">\n");
            for post in posts {
                html_list.push_str(&process_template_tags(&list_item_template, &post));
            }
            html_list.push_str("</ul>\n");
        }
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
        let main_layout = replace_template_variables(&main_layout_template, &main_layout_variables);

        let post_html = process_template_tags(
            &includes
                .get("post.liquid")
                .cloned()
                .unwrap_or_else(String::new),
            &post,
        );

        render_page(
            &post_html,
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
            "out/",
            &page.get("slug").map(|s| s.as_str()).unwrap_or(""),
            &main_layout,
            &includes,
            &global_variables,
        )?;
    }

    Ok(())
}
