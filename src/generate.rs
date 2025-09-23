use crate::{
    config::SiteConfig,
    error::Result,
    file_copier::copy_file_with_versioning,
    file_readers::{load_and_parse_files_with_front_matter_in_directory, load_site_config},
    generate_pagination_pages::generate_pagination_pages,
    layout::load_layout,
    load_data::load_site_data,
    load_includes::load_liquid_includes,
    render_page::render_page,
    types::{ContentCollection, TemplateIncludes, Variables},
};
use std::{
    collections::HashMap,
    fs,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

/// Validates that the site directory exists
fn validate_site_directory(site_name: &str, config: &SiteConfig) -> Result<()> {
    let site_dir = format!("{}/{site_name}", config.sites_base_dir);
    if !std::path::Path::new(&site_dir).exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!(
                "Site directory '{}' does not exist. Available sites: {}",
                site_dir,
                std::fs::read_dir(&config.sites_base_dir).map_or_else(
                    |_| "none".to_string(),
                    |entries| entries
                        .filter_map(|entry| entry.ok()?.file_name().into_string().ok())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            ),
        )
        .into());
    }
    Ok(())
}

/// Container for all loaded site content
struct SiteContent {
    posts: ContentCollection,
    pages: ContentCollection,
    includes: TemplateIncludes,
    site_config: Variables,
    data_variables: Variables,
    main_layout: String,
}

/// Loads all site content (posts, pages, includes, etc.)
fn load_site_content(site_name: &str, config: &SiteConfig) -> Result<SiteContent> {
    let posts_dir = format!(
        "{}/{site_name}/{}",
        config.sites_base_dir, config.posts_subdir
    );
    let pages_dir = format!(
        "{}/{site_name}/{}",
        config.sites_base_dir, config.pages_subdir
    );
    let includes_dir = format!(
        "{}/{site_name}/{}",
        config.sites_base_dir, config.includes_subdir
    );

    // Gracefully handle sites without a posts directory
    let posts = if std::path::Path::new(&posts_dir).exists() {
        load_and_parse_files_with_front_matter_in_directory(&posts_dir)?
    } else {
        Vec::new()
    };
    let pages = load_and_parse_files_with_front_matter_in_directory(&pages_dir)?;
    let includes = load_liquid_includes(&includes_dir);
    let site_config = load_site_config(site_name, config)?;
    let data_variables = load_site_data(site_name, config)?;

    let layout_path = format!(
        "{}/{site_name}/{}/{}",
        config.sites_base_dir, config.layouts_subdir, config.main_layout
    );
    let main_layout = load_layout(&layout_path)?;

    Ok(SiteContent {
        posts,
        pages,
        includes,
        site_config,
        data_variables,
        main_layout,
    })
}

/// Sets up global variables from various sources
fn setup_global_variables(
    content: &SiteContent,
    versioned_assets: HashMap<String, String>,
    generated_date: String,
    config: &SiteConfig,
) -> (Variables, usize) {
    let mut global_variables = Variables::new();

    // Set defaults first
    global_variables.insert("title".to_string(), "My Site".to_string());
    global_variables.insert("site_url".to_string(), "https://example.com".to_string());
    global_variables.insert(
        "description".to_string(),
        "Latest posts from my site".to_string(),
    );

    // Get posts per page from site config before we move it, fallback to default
    let posts_per_page = content
        .site_config
        .get("posts_per_page")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(config.default_posts_per_page);

    // Then merge/override with site config
    global_variables.extend(content.site_config.clone());

    // Add data variables from JSON files
    global_variables.extend(content.data_variables.clone());

    // Add versioned assets and generated date to global variables
    global_variables.extend(versioned_assets);
    global_variables.insert("generated_date".to_string(), generated_date);

    // Add RSS feed URL to global variables
    global_variables.insert("rss_feed_url".to_string(), "/feed.xml".to_string());

    // Add posts and pages collections as indexed global variables for for loops
    add_collection_to_global_variables(&mut global_variables, "posts", &content.posts);
    add_collection_to_global_variables(&mut global_variables, "pages", &content.pages);

    (global_variables, posts_per_page)
}

/// Generates all site content (pagination, posts, pages)
fn generate_site_content(
    site_name: &str,
    content: &SiteContent,
    global_variables: &Variables,
    posts_per_page: usize,
    config: &SiteConfig,
) -> Result<()> {
    // Filter out unlisted posts for pagination
    let filtered_posts: ContentCollection = content
        .posts
        .iter()
        .filter(|post| {
            post.get("unlisted")
                .is_none_or(|value| value.to_lowercase() != "true")
        })
        .cloned()
        .collect();

    generate_pagination_pages(
        site_name,
        posts_per_page,
        &filtered_posts,
        &content.includes,
        &content.main_layout,
        global_variables,
        config,
    )?;

    // Generate posts
    generate_content_items(&ContentGenerationConfig {
        site_name,
        content_items: &content.posts,
        includes: &content.includes,
        main_layout: &content.main_layout,
        global_variables,
        output_directory: &format!("{}/{site_name}/posts/", config.output_dir),
        default_layout: Some("post"),
        site_config: config,
    })?;

    // Generate pages
    generate_content_items(&ContentGenerationConfig {
        site_name,
        content_items: &content.pages,
        includes: &content.includes,
        main_layout: &content.main_layout,
        global_variables,
        output_directory: &format!("{}/{site_name}/", config.output_dir),
        default_layout: None,
        site_config: config,
    })?;

    Ok(())
}

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
            let global_key = format!("{collection_name}.{index}.{key}");
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
    site_config: &'a SiteConfig,
}

/// Generic function to generate content items (posts or pages)
fn generate_content_items(config: &ContentGenerationConfig) -> Result<()> {
    for content_item in config.content_items {
        let mut variables = config.global_variables.clone();
        variables.extend(content_item.clone());
        variables.insert("site_name".to_string(), config.site_name.to_string());

        // Set default layout if provided
        if let Some(layout) = config.default_layout {
            variables.insert("layout".to_string(), layout.to_string());
        }

        // Handle page-specific CSS from front matter
        if let Some(css_file) = content_item.get("css") {
            // Look up the versioned filename from global variables (which contains versioned_assets)
            if let Some(versioned_css) = config.global_variables.get(css_file) {
                variables.insert("page_specific_css".to_string(), versioned_css.clone());
            } else {
                eprintln!(
                    "⚠️  Warning: CSS file '{css_file}' specified in front matter was not found in assets"
                );
            }
        }

        // Store original page title before combining with site title
        if let Some(original_title) = content_item.get("title") {
            variables.insert("original_title".to_string(), original_title.clone());
        }

        // Merge title with site title if content item title exists
        if let Some(title) = content_item.get("title") {
            if let Some(site_title) = config.global_variables.get("title") {
                variables.insert("title".to_string(), format!("{title} - {site_title}"));
            }
        }

        let content = content_item.get("content").map_or("", String::as_str);
        let slug = content_item.get("slug").map_or("", String::as_str);

        render_page(
            content,
            config.output_directory,
            slug,
            config.main_layout,
            config.includes,
            &variables,
            config.site_config,
        )?;
    }

    Ok(())
}

fn copy_assets(site_name: &str, config: &SiteConfig) -> Result<HashMap<String, String>> {
    let assets_dir = format!(
        "{}/{site_name}/{}",
        config.sites_base_dir, config.assets_subdir
    );
    let mut versioned_assets = HashMap::new();

    if let Ok(entries) = fs::read_dir(&assets_dir) {
        for entry in entries.flatten() {
            if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                let path = entry.path();
                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                    // Skip files starting with underscore (e.g., partials)
                    if file_name.starts_with('_') {
                        continue;
                    }
                    let versioned_name = copy_file_with_versioning(
                        &format!("{assets_dir}/{file_name}"),
                        &format!("./{}/{site_name}/assets/", config.output_dir),
                    )?;
                    versioned_assets.insert(file_name.to_string(), versioned_name);
                }
            }
        }
    }

    Ok(versioned_assets)
}

fn copy_data(site_name: &str, config: &SiteConfig) -> Result<()> {
    let data_dir = format!(
        "{}/{site_name}/{}",
        config.sites_base_dir, config.data_subdir
    );
    let output_data_dir = format!("./{}/{site_name}/data", config.output_dir);

    if let Ok(entries) = fs::read_dir(&data_dir) {
        fs::create_dir_all(&output_data_dir)?;
        for entry in entries.flatten() {
            if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                let path = entry.path();
                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                    fs::copy(
                        format!("{data_dir}/{file_name}"),
                        format!("{output_data_dir}/{file_name}"),
                    )?;
                }
            }
        }
    }

    Ok(())
}

pub fn generate(site_name: &str, config: &SiteConfig) -> Result<()> {
    // Start timing the generation process
    let start_time = Instant::now();

    // Validate that the site directory exists
    validate_site_directory(site_name, config)?;

    // Get the current system time
    let now = SystemTime::now();
    let duration_since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let generated_date = duration_since_epoch.as_secs().to_string();

    // Copy assets and data
    let versioned_assets = copy_assets(site_name, config)?;
    copy_data(site_name, config)?;

    // Load all site content
    let content = load_site_content(site_name, config)?;

    // Setup global variables
    let (global_variables, posts_per_page) =
        setup_global_variables(&content, versioned_assets, generated_date, config);

    // Generate all content
    generate_site_content(
        site_name,
        &content,
        &global_variables,
        posts_per_page,
        config,
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
        let config = SiteConfig::default();
        clean_output_directory();

        // Create out directory
        fs::create_dir_all(&config.output_dir).expect("Failed to create out directory");

        // Generate the test site
        generate("test", &config).expect("Failed to generate test site");

        // Check if files exist
        let html_files = vec![
            "out/test/index.html",
            "out/test/about.html",
            "out/test/posts/test-post.html",
        ];

        for file in &html_files {
            assert!(Path::new(file).exists(), "File {file} does not exist");
        }

        // Take snapshots of the generated files
        assert_snapshot!("index_html", read_file_content("out/test/index.html"));
        assert_snapshot!(
            "post_html",
            read_file_content("out/test/posts/test-post.html")
        );
        assert_snapshot!(
            "style_css",
            read_file_content("out/style-d41d8cd98f00b204e9800998ecf8427e.css")
        );

        clean_output_directory();
    }
}
