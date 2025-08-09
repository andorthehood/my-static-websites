use crate::error::Result;
use crate::template_processors::process_template_tags;
use crate::types::{ContentCollection, ContentItem, TemplateIncludes, Variables};
use crate::write::write_html_to_file;

pub fn generate_rss_feed(
    _site_name: &str,
    posts: &ContentCollection,
    includes: &TemplateIncludes,
    global_variables: &Variables,
) -> Result<()> {
    // Get the 20 latest posts sorted by date (newest first)
    let mut sorted_post_refs: Vec<&ContentItem> = posts.iter().collect();
    sorted_post_refs.sort_by(|a, b| {
        let empty_string = String::new();
        let date_a = a.get("date").unwrap_or(&empty_string);
        let date_b = b.get("date").unwrap_or(&empty_string);
        date_b.cmp(date_a) // Reverse order for newest first
    });

    // Take only the 20 latest posts
    let latest_posts: Vec<&ContentItem> = sorted_post_refs.into_iter().take(20).collect();

    // Get site information
    let site_title = global_variables
        .get("title")
        .map_or("My Site", String::as_str);
    let site_description = global_variables
        .get("description")
        .map_or("Latest posts from my site", String::as_str);
    let site_url = global_variables
        .get("site_url")
        .map_or("https://example.com", String::as_str);

    // Start building RSS XML
    let mut rss_xml = String::new();

    // XML declaration and RSS opening
    rss_xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    rss_xml.push_str("<rss version=\"2.0\" xmlns:atom=\"http://www.w3.org/2005/Atom\">\n");
    rss_xml.push_str("  <channel>\n");

    // Channel metadata
    rss_xml.push_str(&format!("    <title>{}</title>\n", escape_xml(site_title)));
    rss_xml.push_str(&format!(
        "    <description>{}</description>\n",
        escape_xml(site_description)
    ));
    rss_xml.push_str(&format!("    <link>{}</link>\n", escape_xml(site_url)));
    rss_xml.push_str(&format!(
        "    <atom:link href=\"{}/feed.xml\" rel=\"self\" type=\"application/rss+xml\" />\n",
        escape_xml(site_url)
    ));
    rss_xml.push_str("    <language>en-us</language>\n");
    rss_xml.push_str("    <generator>lepkefing static site generator</generator>\n");

    // Add current date as lastBuildDate
    let current_date = get_current_rfc2822_date();
    rss_xml.push_str(&format!(
        "    <lastBuildDate>{}</lastBuildDate>\n",
        current_date
    ));

    // Add items
    for post in &latest_posts {
        let empty_string = String::new();
        let title = post.get("title").unwrap_or(&empty_string);
        let slug = post.get("slug").unwrap_or(&empty_string);
        let date = post.get("date").unwrap_or(&empty_string);
        let content = post.get("content").unwrap_or(&empty_string);

        // Process content through centralized processor (handles liquid includes, markdown, etc.)
        let html_content =
            process_template_tags(content, global_variables, Some(includes), Some(post))?;

        // Format date for RSS (RFC 2822 format)
        let pub_date = format_date_for_rss(date);

        rss_xml.push_str("    <item>\n");
        rss_xml.push_str(&format!("      <title>{}</title>\n", escape_xml(title)));
        rss_xml.push_str(&format!(
            "      <link>{}/posts/{}</link>\n",
            escape_xml(site_url),
            escape_xml(slug)
        ));
        rss_xml.push_str(&format!(
            "      <guid>{}/posts/{}</guid>\n",
            escape_xml(site_url),
            escape_xml(slug)
        ));
        rss_xml.push_str(&format!("      <pubDate>{}</pubDate>\n", pub_date));
        rss_xml.push_str(&format!(
            "      <description><![CDATA[{}]]></description>\n",
            html_content
        ));
        rss_xml.push_str("    </item>\n");
    }

    // Close RSS XML
    rss_xml.push_str("  </channel>\n");
    rss_xml.push_str("</rss>\n");

    // Write RSS feed to file
    let output_path = "out/feed.xml";
    write_html_to_file(output_path, &rss_xml)?;

    println!("✓ Generated RSS feed with {} posts", latest_posts.len());

    Ok(())
}

/// Escapes XML special characters
fn escape_xml(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// Formats a date string for RSS (RFC 2822 format)
fn format_date_for_rss(date_str: &str) -> String {
    // Try to parse the date string (assuming YYYY-MM-DD format)
    if let Some(rfc2822_date) = parse_date_to_rfc2822(date_str) {
        rfc2822_date
    } else {
        // If parsing fails, return current date as fallback
        get_current_rfc2822_date()
    }
}

/// Parses a YYYY-MM-DD date string to RFC 2822 format
fn parse_date_to_rfc2822(date_str: &str) -> Option<String> {
    let parts: Vec<&str> = date_str.split('-').collect();
    if parts.len() != 3 {
        return None;
    }

    let year: i32 = parts[0].parse().ok()?;
    let month: u32 = parts[1].parse().ok()?;
    let day: u32 = parts[2].parse().ok()?;

    // Validate month and day
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }

    let month_names = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];

    let month_name = month_names.get((month - 1) as usize)?;

    // Format as RFC 2822: "Mon, 01 Jan 2024 00:00:00 +0000"
    Some(format!(
        "Mon, {:02} {} {} 00:00:00 +0000",
        day, month_name, year
    ))
}

/// Gets the current date in RFC 2822 format
fn get_current_rfc2822_date() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let now = SystemTime::now();
    let duration = now.duration_since(UNIX_EPOCH).unwrap_or_default();
    let timestamp = duration.as_secs();

    // Convert Unix timestamp to a basic date format
    // This is a simplified implementation for RFC 2822 format
    let days_since_epoch = timestamp / 86400;
    let year = 1970 + (days_since_epoch / 365);
    let day_of_year = days_since_epoch % 365;
    let month = (day_of_year / 30) + 1;
    let day = (day_of_year % 30) + 1;

    let month_names = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];

    let month_name = month_names
        .get((month.min(12) - 1) as usize)
        .unwrap_or(&"Jan");

    format!("Mon, {:02} {} {} 00:00:00 +0000", day, month_name, year)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::OUTPUT_DIR;
    use crate::types::ContentItem;
    use std::collections::HashMap;
    use std::fs;
    use std::path::Path;

    fn create_test_post(title: &str, date: &str, content: &str) -> ContentItem {
        let mut post = HashMap::new();
        post.insert("title".to_string(), title.to_string());
        post.insert("date".to_string(), date.to_string());
        post.insert("slug".to_string(), title.to_lowercase().replace(' ', "-"));
        post.insert("content".to_string(), content.to_string());
        post.insert("file_type".to_string(), "md".to_string());
        post
    }

    #[test]
    fn test_generate_rss_feed() {
        // Create test posts
        let posts = vec![
            create_test_post(
                "First Post",
                "2024-01-01",
                "This is the first post content.",
            ),
            create_test_post(
                "Second Post",
                "2024-01-02",
                "This is the second post content.",
            ),
            create_test_post(
                "Third Post",
                "2024-01-03",
                "This is the third post content.",
            ),
        ];

        // Create global variables
        let mut global_variables = Variables::new();
        global_variables.insert("title".to_string(), "Test Blog".to_string());
        global_variables.insert("description".to_string(), "A test blog for RSS".to_string());
        global_variables.insert(
            "site_url".to_string(),
            "https://test.example.com".to_string(),
        );

        // Create out directory
        fs::create_dir_all(OUTPUT_DIR).expect("Failed to create out directory");

        // Create includes (empty for this test)
        let includes = std::collections::HashMap::new();

        // Generate RSS feed
        generate_rss_feed("test", &posts, &includes, &global_variables)
            .expect("Failed to generate RSS feed");

        // Check if RSS file was created
        assert!(Path::new("out/feed.xml").exists());

        // Read and verify RSS content
        let rss_content = fs::read_to_string("out/feed.xml").unwrap();
        assert!(rss_content.contains("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
        assert!(rss_content.contains("<rss version=\"2.0\""));
        assert!(rss_content.contains("<title>Test Blog</title>"));
        assert!(rss_content.contains("<description>A test blog for RSS</description>"));
        assert!(rss_content.contains("<link>https://test.example.com</link>"));
        assert!(rss_content.contains("First Post"));
        assert!(rss_content.contains("Second Post"));
        assert!(rss_content.contains("Third Post"));

        // Clean up
        let _ = fs::remove_file("out/feed.xml");
        let _ = fs::remove_dir_all(OUTPUT_DIR);
    }

    #[test]
    fn test_escape_xml() {
        assert_eq!(escape_xml("Hello & World"), "Hello &amp; World");
        assert_eq!(escape_xml("Less < than"), "Less &lt; than");
        assert_eq!(escape_xml("Greater > than"), "Greater &gt; than");
        assert_eq!(escape_xml("Quote \"test\""), "Quote &quot;test&quot;");
        assert_eq!(escape_xml("Apostrophe's test"), "Apostrophe&apos;s test");
    }

    #[test]
    fn test_format_date_for_rss() {
        let formatted = format_date_for_rss("2024-01-01");
        assert!(formatted.contains("2024"));
        assert!(formatted.contains("Jan"));
        assert!(formatted.contains("01"));
    }

    #[test]
    fn test_parse_date_to_rfc2822() {
        let result = parse_date_to_rfc2822("2024-01-01");
        assert!(result.is_some());
        let formatted = result.unwrap();
        assert!(formatted.contains("2024"));
        assert!(formatted.contains("Jan"));
        assert!(formatted.contains("01"));
        assert!(formatted.contains("00:00:00 +0000"));
    }
}
