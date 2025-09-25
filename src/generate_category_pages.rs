use crate::{
    config::SiteConfig,
    error::Result,
    layout::load_and_render_pagination_layout,
    render_page::render_page,
    template_processors::process_template_tags,
    types::{ContentCollection, ContentItem, TemplateIncludes, Variables},
};
use std::{collections::HashMap, fmt::Write};

/// Convert a category name to a URL-safe slug
fn slugify_category(category: &str) -> String {
    category
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// Group posts by their category field
pub fn group_posts_by_category(
    posts: &ContentCollection,
) -> HashMap<String, (String, ContentCollection)> {
    let mut categories: HashMap<String, (String, ContentCollection)> = HashMap::new();

    for post in posts {
        if let Some(category) = post.get("category") {
            let category = category.trim();
            if !category.is_empty() {
                let slug = slugify_category(category);
                categories
                    .entry(slug)
                    .or_insert_with(|| (category.to_string(), Vec::new()))
                    .1
                    .push(post.clone());
            }
        }
    }

    categories
}

/// Generate pagination pages for a single category
fn generate_category_pagination_pages(
    site_name: &str,
    category_slug: &str,
    category_name: &str,
    posts_per_page: usize,
    posts: &ContentCollection,
    includes: &TemplateIncludes,
    main_layout: &str,
    global_variables: &Variables,
    config: &SiteConfig,
) -> Result<()> {
    let total_pages = posts.len().div_ceil(posts_per_page);

    for page_num in 1..=total_pages {
        let start = (page_num - 1) * posts_per_page;
        let end = std::cmp::min(start + posts_per_page, posts.len());
        let page_posts = &posts[start..end];

        // Create context variables for category pagination template
        let mut variables = global_variables.clone();
        variables.insert(
            "title".to_string(),
            if page_num == 1 {
                format!(
                    "{} - Category: {}",
                    global_variables
                        .get("title")
                        .unwrap_or(&"My Site".to_string()),
                    category_name
                )
            } else {
                format!(
                    "{} - Category: {} - Page {}",
                    global_variables
                        .get("title")
                        .unwrap_or(&"My Site".to_string()),
                    category_name,
                    page_num
                )
            },
        );
        variables.insert("site_name".to_string(), site_name.to_string());
        variables.insert("category_name".to_string(), category_name.to_string());
        variables.insert("category_slug".to_string(), category_slug.to_string());
        variables.insert("page_number".to_string(), page_num.to_string());
        variables.insert("total_pages".to_string(), total_pages.to_string());

        // Add pagination navigation context for categories
        let has_previous = page_num > 1;
        let has_next = page_num < total_pages;
        variables.insert("has_previous".to_string(), has_previous.to_string());
        variables.insert("has_next".to_string(), has_next.to_string());
        
        if has_previous {
            let prev_page = page_num - 1;
            let prev_url = format!("/category/{category_slug}/page{prev_page}");
            variables.insert("previous_page_number".to_string(), prev_page.to_string());
            variables.insert("previous_page_url".to_string(), prev_url);
        }
        
        if has_next {
            let next_page = page_num + 1;
            let next_url = format!("/category/{category_slug}/page{next_page}");
            variables.insert("next_page_number".to_string(), next_page.to_string());
            variables.insert("next_page_url".to_string(), next_url);
        }

        // Add category-specific navigation URLs
        variables.insert("category_index_url".to_string(), format!("/category/{category_slug}/page1"));
        variables.insert("site_index_url".to_string(), "/".to_string());

        // Add posts collection to context
        add_category_posts_collection_to_variables(&mut variables, "page_posts", page_posts);

        // Try to render using category pagination layout template first, then fallback to regular pagination layout
        let body = if let Some(rendered_content) = load_and_render_pagination_layout(
            site_name,
            global_variables.get("category_pagination_layout"),
            &variables,
            includes,
            config,
        ) {
            rendered_content
        } else if let Some(rendered_content) = load_and_render_pagination_layout(
            site_name,
            global_variables.get("pagination_layout"),
            &variables,
            includes,
            config,
        ) {
            rendered_content
        } else {
            // Fall back to original hardcoded HTML generation
            generate_fallback_category_pagination_html(
                page_posts,
                page_num,
                total_pages,
                category_name,
                category_slug,
                includes,
            )?
        };

        // Determine the output file name and path
        let output_directory = format!(
            "{}/{}/category/{}/",
            config.output_dir, site_name, category_slug
        );
        let page_slug = format!("page{}", page_num);

        render_page(
            &body,
            &output_directory,
            &page_slug,
            main_layout,
            includes,
            &variables,
            config,
        )?;
    }

    Ok(())
}

/// Generate pagination pages for all categories
pub fn generate_category_pages(
    site_name: &str,
    posts_per_page: usize,
    posts: &ContentCollection,
    includes: &TemplateIncludes,
    main_layout: &str,
    global_variables: &Variables,
    config: &SiteConfig,
) -> Result<()> {
    // Filter out unlisted posts for category pagination (same as main pagination)
    let filtered_posts: ContentCollection = posts
        .iter()
        .filter(|post| {
            post.get("unlisted")
                .is_none_or(|value| value.to_lowercase() != "true")
        })
        .cloned()
        .collect();

    let categories = group_posts_by_category(&filtered_posts);

    for (category_slug, (category_name, category_posts)) in categories {
        generate_category_pagination_pages(
            site_name,
            &category_slug,
            &category_name,
            posts_per_page,
            &category_posts,
            includes,
            main_layout,
            global_variables,
            config,
        )?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ContentItem;
    use crate::{config::SiteConfig, load_includes::load_liquid_includes};
    use std::collections::HashMap;
    use std::fs;
    use std::path::Path;

    fn create_test_post_with_category(
        title: &str,
        date: &str,
        category: Option<&str>,
    ) -> ContentItem {
        let mut post = HashMap::new();
        post.insert("title".to_string(), title.to_string());
        post.insert("date".to_string(), date.to_string());
        post.insert("slug".to_string(), title.to_lowercase().replace(' ', "-"));
        post.insert("content".to_string(), format!("Content of {}", title));
        if let Some(cat) = category {
            post.insert("category".to_string(), cat.to_string());
        }
        post
    }

    #[test]
    fn test_slugify_category() {
        assert_eq!(slugify_category("Travel"), "travel");
        assert_eq!(slugify_category("Music & Art"), "music-art");
        assert_eq!(slugify_category("Tech/Programming"), "tech-programming");
        assert_eq!(slugify_category("  Spaced  Out  "), "spaced-out");
        assert_eq!(slugify_category("Under_Score"), "under-score");
    }

    #[test]
    fn test_group_posts_by_category() {
        let posts = vec![
            create_test_post_with_category("Post 1", "2024-01-01", Some("Travel")),
            create_test_post_with_category("Post 2", "2024-01-02", Some("Music")),
            create_test_post_with_category("Post 3", "2024-01-03", Some("Travel")),
            create_test_post_with_category("Post 4", "2024-01-04", None), // No category
            create_test_post_with_category("Post 5", "2024-01-05", Some("")), // Empty category
        ];

        let groups = group_posts_by_category(&posts);

        assert_eq!(groups.len(), 2);
        assert!(groups.contains_key("travel"));
        assert!(groups.contains_key("music"));

        let (travel_name, travel_posts) = &groups["travel"];
        assert_eq!(travel_name, "Travel");
        assert_eq!(travel_posts.len(), 2);

        let (music_name, music_posts) = &groups["music"];
        assert_eq!(music_name, "Music");
        assert_eq!(music_posts.len(), 1);
    }

    #[test]
    fn test_group_posts_by_category_empty() {
        let posts = vec![];
        let groups = group_posts_by_category(&posts);
        assert!(groups.is_empty());
    }

    #[test]
    fn test_group_posts_by_category_no_categories() {
        let posts = vec![
            create_test_post_with_category("Post 1", "2024-01-01", None),
            create_test_post_with_category("Post 2", "2024-01-02", Some("")),
        ];

        let groups = group_posts_by_category(&posts);
        assert!(groups.is_empty());
    }

    #[test]
    fn test_generate_category_pages_integration() {
        let posts = vec![
            create_test_post_with_category("Travel Post 1", "2024-01-01", Some("Travel")),
            create_test_post_with_category("Travel Post 2", "2024-01-02", Some("Travel")),
            create_test_post_with_category("Travel Post 3", "2024-01-03", Some("Travel")),
            create_test_post_with_category("Music Post 1", "2024-01-04", Some("Music")),
            create_test_post_with_category("Music Post 2", "2024-01-05", Some("Music")),
            create_test_post_with_category("Uncategorized", "2024-01-06", None),
        ];

        let includes = load_liquid_includes("./sites/test/includes");
        let main_layout = "<!DOCTYPE html><html><body>{{body}}</body></html>";
        let mut global_variables = HashMap::new();
        global_variables.insert("title".to_string(), "Test Site".to_string());
        let config = SiteConfig::default();

        // Clean up any existing output directory
        let _ = fs::remove_dir_all(&config.output_dir);
        fs::create_dir_all(&config.output_dir).expect("Failed to create output directory");

        // Generate category pages with 2 posts per page
        generate_category_pages(
            "category-test",
            2,
            &posts,
            &includes,
            main_layout,
            &global_variables,
            &config,
        )
        .expect("Failed to generate category pages");

        // Check that travel category pages were created (3 posts, 2 per page = 2 pages)
        assert!(Path::new("out/category-test/category/travel/page1.html").exists());
        assert!(Path::new("out/category-test/category/travel/page2.html").exists());
        assert!(!Path::new("out/category-test/category/travel/page3.html").exists());

        // Check that music category pages were created (2 posts, 2 per page = 1 page)
        assert!(Path::new("out/category-test/category/music/page1.html").exists());
        assert!(!Path::new("out/category-test/category/music/page2.html").exists());

        // Check the content of travel category index page
        let travel_index_content =
            fs::read_to_string("out/category-test/category/travel/page1.html").unwrap();
        assert!(travel_index_content.contains("Posts in category:"));
        assert!(travel_index_content.contains("<strong>Travel</strong>"));
        assert!(travel_index_content.contains("Travel Post 1"));
        assert!(travel_index_content.contains("Travel Post 2"));
        assert!(!travel_index_content.contains("Travel Post 3")); // Should be on page 2

        // Check the content of travel category page 2
        let travel_page2_content =
            fs::read_to_string("out/category-test/category/travel/page2.html").unwrap();
        assert!(travel_page2_content.contains("Posts in category:"));
        assert!(travel_page2_content.contains("<strong>Travel</strong>"));
        assert!(travel_page2_content.contains("Travel Post 3"));
        assert!(!travel_page2_content.contains("Travel Post 1")); // Should be on page 1

        // Check that uncategorized posts don't get category pages
        assert!(!Path::new("out/category-test/category/uncategorized").exists());

        // Clean up
        let _ = fs::remove_dir_all(&config.output_dir);
    }

    #[test]
    fn test_category_pagination_with_custom_layout() {
        let posts = vec![
            create_test_post_with_category("Tech Post 1", "2024-01-01", Some("Technology")),
            create_test_post_with_category("Tech Post 2", "2024-01-02", Some("Technology")),
        ];

        let includes = HashMap::new();
        let main_layout = "<!DOCTYPE html><html><body>{{body}}</body></html>";
        let mut global_variables = HashMap::new();
        global_variables.insert("title".to_string(), "Test Site".to_string());
        global_variables.insert("category_pagination_layout".to_string(), "category-pagination".to_string());
        let config = SiteConfig::default();

        // Clean up any existing output directory
        let _ = fs::remove_dir_all(&config.output_dir);
        fs::create_dir_all(&config.output_dir).expect("Failed to create output directory");

        // Generate category pages with 1 post per page to create multiple pages
        generate_category_pages(
            "test",
            1,
            &posts,
            &includes,
            main_layout,
            &global_variables,
            &config,
        )
        .expect("Failed to generate category pages");

        // Check that category pages were created
        assert!(Path::new("out/test/category/technology/page1.html").exists());
        assert!(Path::new("out/test/category/technology/page2.html").exists());

        // Verify that the custom layout would be used (if available)
        let page1_content = fs::read_to_string("out/test/category/technology/page1.html").unwrap();
        // Should contain category-specific content regardless of template
        assert!(page1_content.contains("Tech Post 1"));
        assert!(!page1_content.contains("Tech Post 2"));

        // Clean up
        let _ = fs::remove_dir_all(&config.output_dir);
    }

    #[test]
    fn test_category_pagination_layout_fallback_behavior() {
        let posts = vec![
            create_test_post_with_category("Fallback Post", "2024-01-01", Some("Test Category")),
        ];

        let includes = HashMap::new();
        let main_layout = "<!DOCTYPE html><html><body>{{body}}</body></html>";
        let mut global_variables = HashMap::new();
        global_variables.insert("title".to_string(), "Test Site".to_string());
        // Set a non-existent layout to test fallback to regular pagination layout
        global_variables.insert("category_pagination_layout".to_string(), "non-existent".to_string());
        global_variables.insert("pagination_layout".to_string(), "also-non-existent".to_string());
        let config = SiteConfig::default();

        // Clean up any existing output directory
        let _ = fs::remove_dir_all(&config.output_dir);
        fs::create_dir_all(&config.output_dir).expect("Failed to create output directory");

        // Generate category pages - should fallback to hardcoded HTML
        generate_category_pages(
            "test",
            1,
            &posts,
            &includes,
            main_layout,
            &global_variables,
            &config,
        )
        .expect("Failed to generate category pages with fallback");

        // Verify that the page was created with fallback HTML
        let page1_path = Path::new("out/test/category/test-category/page1.html");
        assert!(page1_path.exists());
        
        let page1_content = fs::read_to_string(&page1_path).unwrap();
        // Should contain the hardcoded category pagination text
        assert!(page1_content.contains("Posts in category:"));
        assert!(page1_content.contains("Test Category"));
        assert!(page1_content.contains("This site uses classic pagination"));

        // Clean up
        let _ = fs::remove_dir_all(&config.output_dir);
    }
}

/// Generates the original hardcoded category pagination HTML as a fallback
fn generate_fallback_category_pagination_html(
    page_posts: &[ContentItem],
    page_num: usize,
    total_pages: usize,
    category_name: &str,
    category_slug: &str,
    includes: &TemplateIncludes,
) -> Result<String> {
    let mut html_list = String::new();

    // Add posts using post.liquid template
    for post in page_posts {
        let post_template = includes
            .get("post")
            .or_else(|| includes.get("post.liquid"))
            .map_or("", |s| s.as_str());

        html_list.push_str(&process_template_tags(post_template, post, None, None)?);
    }

    // Add pagination links
    html_list.push_str(&format!(
        "<p>Posts in category: <strong>{}</strong></p>",
        category_name
    ));
    html_list.push_str("<p>This site uses classic pagination on purpose to help you stop when you want to. Doomscrolling not included.</p><ul class=\"pagination\">");

    // Previous page link
    if page_num > 1 {
        let prev_page = page_num - 1;
        let prev_url = format!("/category/{category_slug}/page{prev_page}");
        write!(
            html_list,
            "<li><a href=\"{prev_url}\">🔙 Previous page</a>,&nbsp;</li>"
        )
        .unwrap();
    }

    // Index page link for this category
    write!(
        html_list,
        "<li><a href=\"/category/{category_slug}/page1\">Category index</a>,&nbsp;</li>"
    )
    .unwrap();

    // Global index page link
    html_list.push_str("<li><a href=\"/\">Site index</a>,&nbsp;</li>");

    // Page numbers
    for i in 1..=total_pages {
        let page_url = format!("/category/{category_slug}/page{i}");
        write!(html_list, "<li><a href=\"{page_url}\">{i}</a>,&nbsp;</li>").unwrap();
    }

    // Next page link
    if page_num < total_pages {
        let next_page = page_num + 1;
        write!(
            html_list,
            "<li><a href=\"/category/{category_slug}/page{next_page}\">Next page ⏭️</a></li>"
        )
        .unwrap();
    }

    html_list.push_str("</ul>");

    Ok(html_list)
}

/// Adds a posts collection to variables for category template access
fn add_category_posts_collection_to_variables(
    variables: &mut Variables,
    collection_name: &str,
    posts: &[ContentItem],
) {
    for (index, post) in posts.iter().enumerate() {
        for (key, value) in post {
            let variable_name = format!("{}.{}.{}", collection_name, index, key);
            variables.insert(variable_name, value.clone());
        }
    }
}
