use crate::{
    config::{
        ASSETS_SUBDIR, DEFAULT_POSTS_PER_PAGE, INCLUDES_SUBDIR, LAYOUTS_SUBDIR, MAIN_LAYOUT,
        OUTPUT_POSTS_DIR, PAGES_SUBDIR, POSTS_SUBDIR, SITES_BASE_DIR,
    },
    error::Result,
    file_copier::copy_file_with_versioning,
    file_readers::{load_and_parse_files_with_front_matter_in_directory, load_site_config},
    generate_pagination_pages::generate_pagination_pages,
    index_page::generate_index_page,
    layout::load_layout,
    load_data::load_site_data,
    load_includes::load_liquid_includes,
    render_page::render_page,
    rss_feed::generate_rss_feed,
    types::{ContentCollection, TemplateIncludes, Variables},
};
use std::{
    collections::HashMap,
    fs,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

/// Convert a content collection into indexed global variables for use with for loops
///
/// Converts a collection like [{"title": "Post 1", "slug": "post-1"}, {"title": "Post 2", "slug": "post-2"}]
/// into variables like:
/// - "posts.0.title" => "Post 1"
/// - "posts.0.slug" => "post-1"
/// - "posts.1.title" => "Post 2"
/// - "posts.1.slug" => "post-2"
fn add_collection_to_global_variables(
    global_variables: &mut Variables,
    collection_name: &str,
    collection: &ContentCollection,
) {
    for (index, item) in collection.iter().enumerate() {
        for (key, value) in item {
            let global_key = format!("{}.{}.{}", collection_name, index, key);
            global_variables.insert(global_key, value.clone());
        }
    }
}

/// Configuration for generating content items
struct ContentGenerationConfig<'a> {
    site_name: &'a str,
    content_items: &'a ContentCollection,
    includes: &'a TemplateIncludes,
    main_layout: &'a str,
    global_variables: &'a Variables,
    output_directory: &'a str,
    default_layout: Option<&'a str>,
}

/// Generic function to generate content items (posts or pages)
fn generate_content_items(config: ContentGenerationConfig) -> Result<()> {
    for content_item in config.content_items {
        let mut variables = config.global_variables.clone();
        variables.extend(content_item.clone());
        variables.insert("site_name".to_string(), config.site_name.to_string());

        // Set default layout if provided
        if let Some(layout) = config.default_layout {
            variables.insert("layout".to_string(), layout.to_string());
        }

        // Merge title with site title if content item title exists
        if let Some(title) = content_item.get("title") {
            if let Some(site_title) = config.global_variables.get("title") {
                variables.insert("title".to_string(), format!("{} - {}", title, site_title));
            }
        }

        let content = content_item.get("content").map_or("", |s| s.as_str());
        let slug = content_item.get("slug").map_or("", |s| s.as_str());

        render_page(
            &content,
            config.output_directory,
            &slug,
            config.main_layout,
            config.includes,
            &variables,
        )?;
    }

    Ok(())
}

fn copy_assets(site_name: &str) -> Result<HashMap<String, String>> {
    let assets_dir = format!("{SITES_BASE_DIR}/{site_name}/{ASSETS_SUBDIR}");
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
    let site_dir = format!("{SITES_BASE_DIR}/{site_name}");
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

    let posts_dir = format!("{SITES_BASE_DIR}/{site_name}/{POSTS_SUBDIR}");
    let pages_dir = format!("{SITES_BASE_DIR}/{site_name}/{PAGES_SUBDIR}");
    let includes_dir = format!("{SITES_BASE_DIR}/{site_name}/{INCLUDES_SUBDIR}");

    let versioned_assets = copy_assets(site_name)?;
    let posts = load_and_parse_files_with_front_matter_in_directory(&posts_dir)?;
    let pages = load_and_parse_files_with_front_matter_in_directory(&pages_dir)?;
    let includes = load_liquid_includes(&includes_dir);
    let site_config = load_site_config(site_name)?;
    let data_variables = load_site_data(site_name)?;

    let mut global_variables = Variables::new();

    // Set defaults first
    global_variables.insert("title".to_string(), "My Site".to_string());
    global_variables.insert("index_filename".to_string(), "index.html".to_string());
    global_variables.insert("site_url".to_string(), "https://example.com".to_string());
    global_variables.insert(
        "description".to_string(),
        "Latest posts from my site".to_string(),
    );

    // Get posts per page from site config before we move it, fallback to default
    let posts_per_page = site_config
        .get("posts_per_page")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(DEFAULT_POSTS_PER_PAGE);

    // Then merge/override with site config
    global_variables.extend(site_config);

    // Add data variables from JSON files
    global_variables.extend(data_variables);

    // Add versioned assets and generated date to global variables
    global_variables.extend(versioned_assets);
    global_variables.insert("generated_date".to_string(), generated_date);

    // Add RSS feed URL to global variables
    global_variables.insert("rss_feed_url".to_string(), "/feed.xml".to_string());

    // Add posts and pages collections as indexed global variables for for loops
    add_collection_to_global_variables(&mut global_variables, "posts", &posts);
    add_collection_to_global_variables(&mut global_variables, "pages", &pages);

    let layout_path = format!("{SITES_BASE_DIR}/{site_name}/{LAYOUTS_SUBDIR}/{MAIN_LAYOUT}");
    let main_layout = load_layout(&layout_path)?;

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

    // Generate RSS feed
    generate_rss_feed(site_name, &posts, &includes, &global_variables)?;

    // Generate posts
    generate_content_items(ContentGenerationConfig {
        site_name,
        content_items: &posts,
        includes: &includes,
        main_layout: &main_layout,
        global_variables: &global_variables,
        output_directory: &format!("{OUTPUT_POSTS_DIR}/"),
        default_layout: Some("post"),
    })?;

    // Generate pages
    generate_content_items(ContentGenerationConfig {
        site_name,
        content_items: &pages,
        includes: &includes,
        main_layout: &main_layout,
        global_variables: &global_variables,
        output_directory: "out/",
        default_layout: None,
    })?;

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
    use crate::config::OUTPUT_DIR;
    use insta::assert_snapshot;
    use std::fs;
    use std::path::Path;

    fn clean_output_directory() {
        let _ = fs::remove_dir_all(OUTPUT_DIR);
    }

    fn read_file_content(path: &str) -> String {
        fs::read_to_string(path).unwrap_or_else(|_| String::new())
    }

    #[test]
    fn test_site_generation() {
        clean_output_directory();

        // Create out directory
        fs::create_dir_all(OUTPUT_DIR).expect("Failed to create out directory");

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
