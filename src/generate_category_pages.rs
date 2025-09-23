use crate::{
    config::SiteConfig,
    error::Result,
    render_page::render_page,
    template_processors::process_template_tags,
    types::{ContentCollection, TemplateIncludes, Variables},
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
            let prev_url = if prev_page == 1 {
                format!("/category/{category_slug}/")
            } else {
                format!("/category/{category_slug}/page{prev_page}")
            };
            write!(
                html_list,
                "<li><a href=\"{prev_url}\">🔙 Previous page</a>,&nbsp;</li>"
            )
            .unwrap();
        }

        // Index page link for this category
        write!(
            html_list,
            "<li><a href=\"/category/{category_slug}/\">Category index</a>,&nbsp;</li>"
        )
        .unwrap();

        // Global index page link
        html_list.push_str("<li><a href=\"/\">Site index</a>,&nbsp;</li>");

        // Page numbers
        for i in 1..=total_pages {
            let page_url = if i == 1 {
                format!("/category/{category_slug}/")
            } else {
                format!("/category/{category_slug}/page{i}")
            };
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

        // Determine the output file name and path
        let output_directory = format!(
            "{}/{}/category/{}/",
            config.output_dir, site_name, category_slug
        );
        let page_slug = if page_num == 1 {
            "index".to_string()
        } else {
            format!("page{}", page_num)
        };

        render_page(
            &html_list,
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
        assert!(Path::new("out/category-test/category/travel/index.html").exists());
        assert!(Path::new("out/category-test/category/travel/page2.html").exists());
        assert!(!Path::new("out/category-test/category/travel/page3.html").exists());

        // Check that music category pages were created (2 posts, 2 per page = 1 page)
        assert!(Path::new("out/category-test/category/music/index.html").exists());
        assert!(!Path::new("out/category-test/category/music/page2.html").exists());

        // Check the content of travel category index page
        let travel_index_content =
            fs::read_to_string("out/category-test/category/travel/index.html").unwrap();
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
}
