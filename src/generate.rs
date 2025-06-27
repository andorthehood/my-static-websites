use crate::{
    config::{DEFAULT_POSTS_PER_PAGE, OUTPUT_DIR, OUTPUT_POSTS_DIR},
    error::Result,
    file_copier::copy_file_with_versioning,
    file_readers::{load_and_parse_files_with_front_matter_in_directory, load_site_config},
    generate_pagination_pages::generate_pagination_pages,
    index_page::generate_index_page,
    layout::load_layout,
    load_includes::load_liquid_includes,
    render_page::render_page,
    template_processors::handlebars::replace_template_variables,
    template_processors::process_template_tags,
    types::{ContentCollection, ContentItem, TemplateIncludes, Variables},
};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

fn prepare_page_context(
    item: &ContentItem,
    site_title: &str,
    main_layout_template: &str,
    main_layout_variables: &mut Variables,
    is_post: bool,
) -> Result<(String, String)> {
    let title = format!(
        "{} - {}",
        item.get("title").cloned().unwrap_or_default(),
        site_title
    );

    let slug = item.get("slug").cloned().unwrap_or_default();
    let pathname = if is_post {
        format!("posts/{slug}")
    } else {
        slug
    };

    main_layout_variables.insert("pathname".to_string(), pathname);
    let main_layout = replace_template_variables(main_layout_template, main_layout_variables)?;

    Ok((title, main_layout))
}

fn generate_posts(
    posts: &ContentCollection,
    includes: &TemplateIncludes,
    main_layout_template: &str,
    main_layout_variables: &Variables,
    global_variables: &Variables,
) -> Result<()> {
    let site_title = global_variables.get("title").cloned().unwrap_or_default();

    for post in posts {
        let mut post_layout_vars = main_layout_variables.clone();
        let mut post_global_vars = global_variables.clone();

        let (title, main_layout) = prepare_page_context(
            post,
            &site_title,
            main_layout_template,
            &mut post_layout_vars,
            true,
        )?;

        post_global_vars.insert("title".to_string(), title);

        let post_html = process_template_tags(
            &includes.get("post.liquid").cloned().unwrap_or_default(),
            post,
        )?;

        render_page(
            &post_html,
            &format!("{OUTPUT_POSTS_DIR}/"),
            post.get("slug").map_or("", std::string::String::as_str),
            &main_layout,
            includes,
            &post_global_vars,
        )?;
    }
    Ok(())
}

fn generate_pages(
    pages: &ContentCollection,
    includes: &TemplateIncludes,
    main_layout_template: &str,
    main_layout_variables: &Variables,
    global_variables: &Variables,
) -> Result<()> {
    let site_title = global_variables.get("title").cloned().unwrap_or_default();

    for page in pages {
        let mut page_layout_vars = main_layout_variables.clone();
        let mut page_global_vars = global_variables.clone();

        let (title, main_layout) = prepare_page_context(
            page,
            &site_title,
            main_layout_template,
            &mut page_layout_vars,
            false,
        )?;

        page_global_vars.insert("title".to_string(), title);

        render_page(
            page.get("content").map_or("", std::string::String::as_str),
            &format!("{OUTPUT_DIR}/"),
            page.get("slug").map_or("", std::string::String::as_str),
            &main_layout,
            includes,
            &page_global_vars,
        )?;
    }
    Ok(())
}

pub fn generate(site_name: &str) -> Result<()> {
    // Validate that the site directory exists
    let site_dir = format!("./sites/{site_name}");
    if !std::path::Path::new(&site_dir).exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!(
                "Site directory '{}' does not exist. Available sites: {}",
                site_dir,
                std::fs::read_dir("./sites")
                    .map(|entries| entries
                        .filter_map(|entry| entry.ok()?.file_name().into_string().ok())
                        .collect::<Vec<_>>()
                        .join(", "))
                    .unwrap_or_else(|_| "none".to_string())
            ),
        )
        .into());
    }

    // Start timing the generation process
    let start_time = Instant::now();

    // Get the current system time
    let now = SystemTime::now();
    let duration_since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let generated_date = duration_since_epoch.as_secs().to_string();

    let css_file_path = format!("./sites/{site_name}/assets/style.css");
    let js_file_path = format!("./sites/{site_name}/assets/script.js");
    let posts_dir = format!("./sites/{site_name}/posts");
    let pages_dir = format!("./sites/{site_name}/pages");
    let includes_dir = format!("./sites/{site_name}/includes");

    let css_file_name = copy_file_with_versioning(&css_file_path, "./out/assets/")?;
    let js_file_name = copy_file_with_versioning(&js_file_path, "./out/assets/")?;
    let posts = load_and_parse_files_with_front_matter_in_directory(&posts_dir)?;
    let pages = load_and_parse_files_with_front_matter_in_directory(&pages_dir)?;
    let includes = load_liquid_includes(&includes_dir);
    let site_config = load_site_config(site_name)?;

    let mut global_variables = Variables::new();
    global_variables.insert(
        "title".to_string(),
        site_config
            .get("title")
            .cloned()
            .unwrap_or_else(|| "My Site".to_string()),
    );
    global_variables.insert(
        "index_filename".to_string(),
        site_config
            .get("index_filename")
            .cloned()
            .unwrap_or_else(|| "index.html".to_string()),
    );

    // Get posts per page from site config, fallback to default
    let posts_per_page = site_config
        .get("posts_per_page")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(DEFAULT_POSTS_PER_PAGE);

    let layout_path = format!("./sites/{site_name}/layouts/main.html");
    let main_layout_template = load_layout(&layout_path)?;
    let mut main_layout_variables = Variables::new();
    main_layout_variables.insert("css_file_name".to_string(), css_file_name);
    main_layout_variables.insert("js_file_name".to_string(), js_file_name);
    main_layout_variables.insert("generated_date".to_string(), generated_date);
    let main_layout = replace_template_variables(&main_layout_template, &main_layout_variables)?;

    generate_pagination_pages(
        posts_per_page,
        &posts,
        &includes,
        &main_layout,
        &global_variables,
    )?;

    // Generate index page
    generate_index_page(&posts, &includes, &main_layout, &global_variables)?;

    // Generate posts
    generate_posts(
        &posts,
        &includes,
        &main_layout_template,
        &main_layout_variables,
        &global_variables,
    )?;

    // Generate pages
    generate_pages(
        &pages,
        &includes,
        &main_layout_template,
        &main_layout_variables,
        &global_variables,
    )?;

    // Log the total generation time
    let elapsed = start_time.elapsed();
    println!(
        "✓ Generated site '{}' in {}ms",
        site_name,
        elapsed.as_millis()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;
    use std::fs;
    use std::path::Path;

    fn clean_output_directory() {
        let _ = fs::remove_dir_all("out");
    }

    fn read_file_content(path: &str) -> String {
        fs::read_to_string(path).unwrap_or_else(|_| String::new())
    }

    #[test]
    fn test_site_generation() {
        clean_output_directory();

        // Create out directory
        fs::create_dir_all("out").expect("Failed to create out directory");

        // Generate the test site
        generate("test").expect("Failed to generate test site");

        // Check if files exist
        let html_files = vec![
            "out/index.html",
            "out/about.html",
            "out/posts/test-post.html",
        ];

        for file in &html_files {
            assert!(Path::new(file).exists(), "File {} does not exist", file);
        }

        // Take snapshots of the generated files
        assert_snapshot!("index_html", read_file_content("out/index.html"));
        assert_snapshot!("post_html", read_file_content("out/posts/test-post.html"));
        assert_snapshot!("about_html", read_file_content("out/about.html"));
        assert_snapshot!(
            "style_css",
            read_file_content("out/style-d41d8cd98f00b204e9800998ecf8427e.css")
        );

        clean_output_directory();
    }
}
