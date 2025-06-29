use crate::{
    config::{DEFAULT_POSTS_PER_PAGE, OUTPUT_POSTS_DIR},
    error::Result,
    file_copier::copy_file_with_versioning,
    file_readers::{load_and_parse_files_with_front_matter_in_directory, load_site_config},
    generate_pagination_pages::generate_pagination_pages,
    index_page::generate_index_page,
    layout::load_layout,
    load_includes::load_liquid_includes,
    render_page::render_page,
    template_processors::handlebars::replace_template_variables,
    template_processors::liquid::process_liquid_tags,
    types::{ContentCollection, TemplateIncludes, Variables},
};
use std::{
    collections::HashMap,
    fs,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

pub fn generate_posts(
    site_name: &str,
    posts: &ContentCollection,
    includes: &TemplateIncludes,
    main_layout: &str,
    main_layout_variables: &Variables,
    global_variables: &Variables,
) -> Result<()> {
    for post in posts {
        let mut variables = global_variables.clone();
        variables.extend(main_layout_variables.clone());
        variables.extend(post.clone());
        variables.insert("site_name".to_string(), site_name.to_string());
        variables.insert("layout".to_string(), "post".to_string());

        // Merge title with site title if post title exists
        if let Some(title) = post.get("title") {
            if let Some(site_title) = global_variables.get("title") {
                variables.insert("title".to_string(), format!("{} - {}", title, site_title));
            }
        }

        let content = post.get("content").cloned().unwrap_or_default();
        let slug = post.get("slug").cloned().unwrap_or_default();

        render_page(
            &content,
            &format!("{OUTPUT_POSTS_DIR}/"),
            &slug,
            main_layout,
            includes,
            &variables,
        )?;
    }

    Ok(())
}

pub fn generate_pages(
    site_name: &str,
    pages: &ContentCollection,
    includes: &TemplateIncludes,
    main_layout: &str,
    main_layout_variables: &Variables,
    global_variables: &Variables,
) -> Result<()> {
    for page in pages {
        let mut variables = global_variables.clone();
        variables.extend(main_layout_variables.clone());
        variables.extend(page.clone());
        variables.insert("site_name".to_string(), site_name.to_string());

        // Merge title with site title if page title exists
        if let Some(title) = page.get("title") {
            if let Some(site_title) = global_variables.get("title") {
                variables.insert("title".to_string(), format!("{} - {}", title, site_title));
            }
        }

        let content = page.get("content").cloned().unwrap_or_default();
        let slug = page.get("slug").cloned().unwrap_or_default();

        render_page(&content, "out/", &slug, main_layout, includes, &variables)?;
    }

    Ok(())
}

fn copy_assets(site_name: &str) -> Result<HashMap<String, String>> {
    let assets_dir = format!("./sites/{site_name}/assets");
    let mut versioned_assets = HashMap::new();

    if let Ok(entries) = fs::read_dir(&assets_dir) {
        for entry in entries.flatten() {
            if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                let path = entry.path();
                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                    let versioned_name = copy_file_with_versioning(
                        &format!("{assets_dir}/{file_name}"),
                        "./out/assets/",
                    )?;
                    versioned_assets.insert(file_name.to_string(), versioned_name);
                }
            }
        }
    }

    Ok(versioned_assets)
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

    let posts_dir = format!("./sites/{site_name}/posts");
    let pages_dir = format!("./sites/{site_name}/pages");
    let includes_dir = format!("./sites/{site_name}/includes");

    let versioned_assets = copy_assets(site_name)?;
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
    main_layout_variables.extend(versioned_assets);
    main_layout_variables.insert("generated_date".to_string(), generated_date);

    // First process liquid includes in the main layout template
    let keys: Vec<String> = main_layout_variables.keys().cloned().collect();
    let main_layout_with_includes = process_liquid_tags(&main_layout_template, &keys, &includes)?;
    // Then process handlebars variables
    let main_layout =
        replace_template_variables(&main_layout_with_includes, &main_layout_variables)?;

    generate_pagination_pages(
        site_name,
        posts_per_page,
        &posts,
        &includes,
        &main_layout,
        &global_variables,
    )?;

    // Generate index page
    generate_index_page(
        site_name,
        &posts,
        &includes,
        &main_layout,
        &global_variables,
    )?;

    // Generate posts
    generate_posts(
        site_name,
        &posts,
        &includes,
        &main_layout,
        &main_layout_variables,
        &global_variables,
    )?;

    // Generate pages
    generate_pages(
        site_name,
        &pages,
        &includes,
        &main_layout,
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
