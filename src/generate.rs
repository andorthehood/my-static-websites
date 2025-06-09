use crate::file_copier::copy_file_with_versioning;
use crate::file_readers::{load_and_parse_markdown_files_with_front_matter_in_directory, load_site_config};
use crate::generate_pagination_pages::generate_pagination_pages;
use crate::template_processors::handlebars::replace_template_variables;
use crate::layout::load_layout;
use crate::load_includes::load_liquid_includes;
use crate::error::Result;
use crate::index_page::generate_index_page;
use crate::template_processor::render_page;
use crate::template_processors::process_template_tags;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

fn prepare_page_context<'a>(
    item: &'a HashMap<String, String>,
    site_title: &str,
    main_layout_template: &str,
    main_layout_variables: &mut HashMap<String, String>,
    is_post: bool,
) -> Result<(String, String)> {
    let title = format!(
        "{} - {}",
        item.get("title").cloned().unwrap_or_default(),
        site_title
    );
    
    let slug = item.get("slug").cloned().unwrap_or_default();
    let pathname = if is_post {
        format!("posts/{}", slug)
    } else {
        slug
    };
    
    main_layout_variables.insert("pathname".to_string(), pathname);
    let main_layout = replace_template_variables(main_layout_template, main_layout_variables)?;
    
    Ok((title, main_layout))
}

fn generate_posts(
    posts: &Vec<HashMap<String, String>>,
    includes: &HashMap<String, String>,
    main_layout_template: &str,
    main_layout_variables: &mut HashMap<String, String>,
    global_variables: &mut HashMap<String, String>,
) -> Result<()> {
    let site_title = global_variables.get("title").cloned().unwrap_or_default();
    
    for post in posts {
        let (title, main_layout) = prepare_page_context(
            post,
            &site_title,
            main_layout_template,
            main_layout_variables,
            true
        )?;
        
        global_variables.insert("title".to_string(), title);

        let post_html = process_template_tags(
            &includes.get("post.liquid").cloned().unwrap_or_default(),
            post,
        )?;

        render_page(
            &post_html,
            "out/posts/",
            &post.get("slug").map(|s| s.as_str()).unwrap_or(""),
            &main_layout,
            includes,
            global_variables,
        )?;
    }
    Ok(())
}

fn generate_pages(
    pages: &Vec<HashMap<String, String>>,
    includes: &HashMap<String, String>,
    main_layout_template: &str,
    main_layout_variables: &mut HashMap<String, String>,
    global_variables: &mut HashMap<String, String>,
) -> Result<()> {
    let site_title = global_variables.get("title").cloned().unwrap_or_default();
    
    for page in pages {
        let (title, main_layout) = prepare_page_context(
            page,
            &site_title,
            main_layout_template,
            main_layout_variables,
            false
        )?;
        
        global_variables.insert("title".to_string(), title);

        render_page(
            &page.get("content").map(|s| s.as_str()).unwrap_or(""),
            "out/",
            &page.get("slug").map(|s| s.as_str()).unwrap_or(""),
            &main_layout,
            includes,
            global_variables,
        )?;
    }
    Ok(())
}

pub fn generate(site_name: &str) -> Result<()> {
    // Get the current system time
    let now = SystemTime::now();
    let duration_since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let generated_date = duration_since_epoch.as_secs().to_string();

    let css_file_name = copy_file_with_versioning(&format!("./sites/{}/style.css", site_name), "./out/")?;
    let posts = load_and_parse_markdown_files_with_front_matter_in_directory(&format!("./sites/{}/posts", site_name))?;
    let pages = load_and_parse_markdown_files_with_front_matter_in_directory(&format!("./sites/{}/pages", site_name))?;
    let includes = load_liquid_includes(&format!("./sites/{}/includes", site_name));
    let site_config = load_site_config(site_name)?;

    let mut global_variables = HashMap::new();
    global_variables.insert(
        "title".to_string(),
        site_config.get("title").cloned().unwrap_or_else(|| "My Site".to_string()),
    );
    global_variables.insert(
        "index_filename".to_string(),
        site_config.get("index_filename").cloned().unwrap_or_else(|| "index.html".to_string()),
    );

    let main_layout_template = load_layout(&format!("./sites/{}/layouts/main.html", site_name))?;
    let mut main_layout_variables = HashMap::new();
    main_layout_variables.insert("css_file_name".to_string(), css_file_name);
    main_layout_variables.insert("generated_date".to_string(), generated_date);
    let main_layout = replace_template_variables(&main_layout_template, &main_layout_variables)?;

    generate_pagination_pages(5, &posts, &includes, &main_layout, &global_variables)?;

    // Generate index page
    generate_index_page(&posts, &includes, &main_layout, &global_variables)?;

    // Generate posts
    generate_posts(
        &posts,
        &includes,
        &main_layout_template,
        &mut main_layout_variables,
        &mut global_variables,
    )?;

    // Generate pages
    generate_pages(
        &pages,
        &includes,
        &main_layout_template,
        &mut main_layout_variables,
        &mut global_variables,
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    use insta::assert_snapshot;

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
        assert_snapshot!("style_css", read_file_content("out/style-d41d8cd98f00b204e9800998ecf8427e.css"));

        clean_output_directory();
    }
}
