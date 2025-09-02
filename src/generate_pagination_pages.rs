use crate::{
    config::SiteConfig,
    error::Result,
    render_page::render_page,
    template_processors::process_template_tags,
    types::{ContentCollection, TemplateIncludes, Variables},
};
use std::fmt::Write;

pub fn generate_pagination_pages(
    site_name: &str,
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
            html_list.push_str(&process_template_tags(
                includes.get("post.liquid").map_or("", |s| s.as_str()),
                post,
                None,
                None,
            )?);
        }

        // Add pagination links
        html_list.push_str("<p>This site uses classic pagination on purpose to help you stop when you want to. Doomscrolling not included.</p><ul class=\"pagination\">");

        // Previous page link
        if page_num > 1 {
            let prev_page = page_num - 1;
            write!(
                html_list,
                "<li><a href=\"/page{prev_page}\">🔙 Previous page</a>,&nbsp;</li>"
            ).unwrap();
        }

        // Index page link
        html_list.push_str("<li><a href=\"/\">Index page</a>,&nbsp;</li>");

        // Page numbers
        for i in 1..=total_pages {
            write!(html_list, "<li><a href=\"/page{i}\">{i}</a>,&nbsp;</li>").unwrap();
        }

        // Next page link
        if page_num < total_pages {
            let next_page = page_num + 1;
            write!(
                html_list,
                "<li><a href=\"/page{next_page}\">Next page ⏭️</a></li>"
            ).unwrap();
        }

        html_list.push_str("</ul>");

        let mut variables = global_variables.clone();
        variables.insert("title".to_string(), format!("Page {page_num}"));
        variables.insert("site_name".to_string(), site_name.to_string());

        render_page(
            &html_list,
            &format!("{}/{site_name}/", config.output_dir),
            &format!("page{page_num}"),
            main_layout,
            includes,
            &variables,
        )?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ContentItem;
    use std::collections::HashMap;
    use std::fs;
    use std::path::Path;

    fn create_test_post(title: &str, date: &str) -> ContentItem {
        let mut post = HashMap::new();
        post.insert("title".to_string(), title.to_string());
        post.insert("date".to_string(), date.to_string());
        post.insert("slug".to_string(), title.to_lowercase().replace(' ', "-"));
        post.insert("content".to_string(), format!("Content of {title}"));
        post
    }

    #[test]
    fn test_pagination_generation() {
        // Create test data
        let mut posts = Vec::new();
        for i in 1..=7 {
            posts.push(create_test_post(&format!("Test Post {i}"), "2024-03-20"));
        }

        let mut includes = HashMap::new();
        includes.insert(
            "post.liquid".to_string(),
            "<div class=\"post\">{{title}}</div>".to_string(),
        );

        let main_layout = "<!DOCTYPE html><html><body>{{body}}</body></html>";
        let mut global_variables = HashMap::new();
        global_variables.insert("site_title".to_string(), "Test Site".to_string());
        let config = SiteConfig::default();

        // Clean up any existing output directory
        let _ = fs::remove_dir_all(&config.output_dir);

        // Create output directory
        fs::create_dir_all(&config.output_dir).expect("Failed to create output directory");

        // Generate pagination pages (3 posts per page should create 3 pages)
        generate_pagination_pages("test", 3, &posts, &includes, main_layout, &global_variables, &config)
            .expect("Failed to generate pagination pages");

        // Verify the pages were created
        assert!(Path::new("out/test/page1.html").exists());
        assert!(Path::new("out/test/page2.html").exists());
        assert!(Path::new("out/test/page3.html").exists());

        // Verify page contents
        let page1_content = fs::read_to_string("out/test/page1.html").unwrap();
        assert!(page1_content.contains("Test Post 1"));
        assert!(page1_content.contains("Test Post 2"));
        assert!(page1_content.contains("Test Post 3"));
        assert!(!page1_content.contains("Test Post 4"));

        let page2_content = fs::read_to_string("out/test/page2.html").unwrap();
        assert!(page2_content.contains("Test Post 4"));
        assert!(page2_content.contains("Test Post 5"));
        assert!(page2_content.contains("Test Post 6"));
        assert!(!page2_content.contains("Test Post 7"));

        let page3_content = fs::read_to_string("out/test/page3.html").unwrap();
        assert!(page3_content.contains("Test Post 7"));
        assert!(!page3_content.contains("Test Post 1"));

        // Clean up
        let _ = fs::remove_dir_all("out");
    }
}
