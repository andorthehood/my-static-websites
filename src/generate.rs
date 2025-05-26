use crate::file_copier::copy_file_with_versioning;
use crate::file_readers::load_and_parse_markdown_files_with_front_matter_in_directory;
use crate::generate_pagination_pages::generate_pagination_pages;
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
use std::path::PathBuf; // For joining paths
use std::time::{SystemTime, UNIX_EPOCH};
use crate::error::MyError;
use crate::config::Config;

// Struct to hold all loaded site data
struct SiteData {
    posts: Vec<HashMap<String, String>>,
    pages: Vec<HashMap<String, String>>,
    includes: HashMap<String, String>,
    main_layout_template: String,
    css_file_name: String,
    generated_date: String,
}

// Function to load all initial site data, now using Config
fn load_site_data(config: &Config) -> Result<SiteData, MyError> {
    let now = SystemTime::now();
    let duration_since_epoch = now
        .duration_since(UNIX_EPOCH)
        .map_err(|e| MyError::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;
    let generated_date = duration_since_epoch.as_secs().to_string();

    // Construct full path for style.css before copying
    let style_source_path = PathBuf::from(&config.static_dir).join(&config.style_path);

    let css_file_name = copy_file_with_versioning(
        style_source_path.to_str().ok_or_else(|| MyError::Io(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid style_path")))?,
        &config.output_dir
    )?;
    let posts = load_and_parse_markdown_files_with_front_matter_in_directory(&config.posts_dir)?;
    let pages = load_and_parse_markdown_files_with_front_matter_in_directory(&config.pages_dir)?;
    let includes = load_liquid_includes(&config.includes_dir)?;
    
    let main_layout_path = PathBuf::from(&config.layouts_dir).join("main.html");
    let main_layout_template = load_layout(
        main_layout_path.to_str().ok_or_else(|| MyError::Io(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid layouts_dir")))?,
    )?;

    Ok(SiteData {
        posts,
        pages,
        includes,
        main_layout_template,
        css_file_name,
        generated_date,
    })
}

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
) -> Result<(), MyError> {
    let mut html = markdown_to_html(&body);
    let file_name = directory.to_string() + &slug + ".html";
    html = process_liquid_includes(&html, &includes);
    html = insert_body_into_layout(&layout, &html);
    html = process_template_tags(&html, &variables);
    write_html_to_file(&file_name, &html)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    // Minimal Config for testing prepare_content_item_data logic that depends on output_dir
    fn minimal_config_for_prepare() -> Config {
        Config {
            posts_dir: "_posts_test".to_string(),
            pages_dir: "_pages_test".to_string(),
            includes_dir: "_includes_test".to_string(),
            layouts_dir: "_layouts_test".to_string(),
            output_dir: "test_out".to_string(),
            style_path: "style_test.css".to_string(),
            static_dir: "static_test".to_string(),
            output_static_dir_name: "static_test_out".to_string(),
            content_image_dir_name: "images_content_test".to_string(),
            output_image_dir_name: "images_out_test".to_string(),
        }
    }


    #[test]
    fn test_process_template_tags_basic_replacement() {
        let mut variables = HashMap::new();
        variables.insert("name".to_string(), "World".to_string());
        variables.insert("thing".to_string(), "Rust".to_string());
        let input = "Hello {{name}}! This is {{thing}}.";
        let expected = "Hello World! This is Rust.";
        assert_eq!(process_template_tags(input, &variables), expected);
    }

    #[test]
    fn test_process_template_tags_liquid_if_true() {
        let mut variables = HashMap::new();
        variables.insert("show_section".to_string(), "true".to_string()); // Value doesn't matter, just presence
        variables.insert("item".to_string(), "Awesome".to_string());
        let input = "{% if show_section %}This section with {{item}} is shown.{% endif %}";
        let expected = "This section with Awesome is shown.";
        assert_eq!(process_template_tags(input, &variables), expected);
    }

    #[test]
    fn test_process_template_tags_liquid_if_false() {
        let variables = HashMap::new(); // show_section is not present
        let input = "{% if show_section %}This section is hidden.{% endif %}This part is always visible.";
        let expected = "This part is always visible.";
        assert_eq!(process_template_tags(input, &variables), expected);
    }

    #[test]
    fn test_process_template_tags_remove_undefined_handlebars() {
        let mut variables = HashMap::new();
        variables.insert("defined_var".to_string(), "Value".to_string());
        let input = "With {{defined_var}}. And {{undefined_var}} here. {{another_one}}";
        let expected = "With Value. And  here. "; // Undefined vars are removed
        assert_eq!(process_template_tags(input, &variables), expected);
    }

    #[test]
    fn test_process_template_tags_all_features_combined() {
        let mut variables = HashMap::new();
        variables.insert("user_name".to_string(), "Alice".to_string());
        variables.insert("display_greeting".to_string(), "true".to_string());
        variables.insert("item_name".to_string(), "widget".to_string());

        let input = "{% if display_greeting %}Hello {{user_name}}!{% endif %} You have a {{item_name}}. {{undefined_details}}";
        let expected = "Hello Alice! You have a widget. ";
        assert_eq!(process_template_tags(input, &variables), expected);
    }

    #[test]
    fn test_process_template_tags_empty_input() {
        let variables = HashMap::new();
        assert_eq!(process_template_tags("", &variables), "");
    }

    #[test]
    fn test_process_template_tags_no_tags() {
        let mut variables = HashMap::new();
        variables.insert("var".to_string(), "value".to_string());
        let input = "Just plain text.";
        assert_eq!(process_template_tags(input, &variables), "Just plain text.");
    }

    #[test]
    fn test_process_template_tags_liquid_if_with_no_content() {
        let mut variables = HashMap::new();
        variables.insert("condition".to_string(), "true".to_string());
        let input = "{% if condition %}{% endif %}After";
        let expected = "After";
        assert_eq!(process_template_tags(input, &variables), expected);

        let input_false = "{% if non_existent_condition %}{% endif %}AfterAlso";
        let expected_false = "AfterAlso";
        assert_eq!(process_template_tags(input_false, &variables), expected_false);
    }

    // Tests for prepare_content_item_data
    #[test]
    fn test_prepare_content_item_data_post() {
        let config = minimal_config_for_prepare();
        let mut item_data = HashMap::new();
        item_data.insert("title".to_string(), "My Post".to_string());
        item_data.insert("slug".to_string(), "my-post".to_string());

        let mut includes = HashMap::new();
        includes.insert("post.liquid".to_string(), "Post: {{title}}".to_string());
        
        let base_main_layout_variables = HashMap::new();
        let site_global_variables = HashMap::from([("title".to_string(), "Site Title".to_string())]);

        let result = prepare_content_item_data(
            &item_data, "post", "Layout: {{pathname}}", &base_main_layout_variables,
            &includes, &site_global_variables, &config
        ).unwrap();
        
        let expected_output_dir = PathBuf::from(&config.output_dir).join("posts").to_str().unwrap().to_string() + "/";

        assert_eq!(result.body_content, "Post: My Post");
        assert_eq!(result.slug, "my-post");
        assert_eq!(result.output_directory, expected_output_dir);
        assert!(result.final_layout_string.contains("Layout: posts/my-post"));
        assert_eq!(result.item_global_variables.get("title").unwrap(), "My Post - Site Title");
    }

    #[test]
    fn test_prepare_content_item_data_page() {
        let config = minimal_config_for_prepare();
        let mut item_data = HashMap::new();
        item_data.insert("title".to_string(), "About Page".to_string());
        item_data.insert("slug".to_string(), "about".to_string());
        item_data.insert("content".to_string(), "This is about us.".to_string());

        let includes = HashMap::new(); 
        let base_main_layout_variables = HashMap::new();
        let site_global_variables = HashMap::from([("title".to_string(), "Site Title".to_string())]);

        let result = prepare_content_item_data(
            &item_data, "page", "Layout: {{pathname}}", &base_main_layout_variables,
            &includes, &site_global_variables, &config
        ).unwrap();
        
        let expected_output_dir = config.output_dir + "/";

        assert_eq!(result.body_content, "This is about us.");
        assert_eq!(result.slug, "about");
        assert_eq!(result.output_directory, expected_output_dir);
        assert!(result.final_layout_string.contains("Layout: about"));
        assert_eq!(result.item_global_variables.get("title").unwrap(), "About Page - Site Title");
    }

    #[test]
    fn test_prepare_content_item_data_empty_slug() {
        let config = minimal_config_for_prepare();
        let item_data = HashMap::new();
        let includes = HashMap::new();
        let base_main_layout_variables = HashMap::new();
        let site_global_variables = HashMap::new();

        let result = prepare_content_item_data(
            &item_data, "post", "", &base_main_layout_variables,
            &includes, &site_global_variables, &config
        );
        assert!(matches!(result, Err(MyError::Io(_))));
        if let Err(MyError::Io(e)) = result {
            assert_eq!(e.kind(), std::io::ErrorKind::InvalidInput);
            assert!(e.to_string().contains("Slug is empty"));
        }
    }

    #[test]
    fn test_prepare_content_item_data_invalid_type() {
        let config = minimal_config_for_prepare();
        let mut item_data = HashMap::new();
        item_data.insert("slug".to_string(), "something".to_string());
        let includes = HashMap::new();
        let base_main_layout_variables = HashMap::new();
        let site_global_variables = HashMap::new();

        let result = prepare_content_item_data(
            &item_data, "invalid_type", "", &base_main_layout_variables,
            &includes, &site_global_variables, &config
        );
        assert!(matches!(result, Err(MyError::Io(_))));
        if let Err(MyError::Io(e)) = result {
            assert_eq!(e.kind(), std::io::ErrorKind::InvalidInput);
            assert!(e.to_string().contains("Invalid item_type"));
        }
    }
}

// Function to generate the index page
fn generate_index_page(
    posts: &Vec<HashMap<String, String>>,
    includes: &HashMap<String, String>,
    main_layout: &str, // Already processed with site-wide vars like CSS
    site_global_variables: &HashMap<String, String>,
    config: &Config,
) -> Result<(), MyError> {
    // Group posts by year
    let mut posts_by_year: HashMap<String, Vec<HashMap<String, String>>> = HashMap::new();
    for post in posts {
        if let Some(date_str) = post.get("date") {
            if date_str.len() >= 4 { // Ensure date_str is long enough
                let year = &date_str[0..4];
                posts_by_year
                    .entry(year.to_string())
                    .or_default()
                    .push(post.clone());
            }
        }
    }

    // Collect and sort the years in descending order
    let mut years: Vec<String> = posts_by_year.keys().cloned().collect();
    years.sort_by(|a, b| b.cmp(a));

    let list_item_template = includes
        .get("list_item.liquid")
        .cloned()
        .unwrap_or_default(); // Default to empty if not found, consistent with previous behavior

    let mut html_list = String::new();
    html_list.push_str("<p>Hi there! 👋 My name is Andor Polgar. This is my personal website. Here, you'll find my photography projects and random snapshots from my life.</p><ul class=\"postlist\">\n");

    for year in years {
        if let Some(year_posts) = posts_by_year.get(&year) {
            let year_template_content = includes
                .get(&format!("{}.liquid", year))
                .cloned()
                .unwrap_or_default(); // Default to empty if not found
            html_list.push_str(&year_template_content);
            html_list.push_str("<ul class=\"postlist\">\n");
            for post_item in year_posts {
                html_list.push_str(&process_template_tags(&list_item_template, post_item));
            }
            html_list.push_str("</ul>\n");
        }
    }
    html_list.push_str("</ul>");

    let mut index_html = insert_body_into_layout(main_layout, &html_list);
    // Use the site's main title for the index page
    index_html = replace_template_variable(&index_html, "title", site_global_variables.get("title").unwrap_or(&"".to_string()));
    index_html = remove_handlebars_variables(&index_html); // Clean up any remaining tags

    let index_output_path = PathBuf::from(&config.output_dir).join("index.html");
    write_html_to_file(
        index_output_path.to_str().ok_or_else(|| MyError::Io(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid output_dir for index.html")))?,
        &index_html
    )?;
    Ok(())
}

// Struct to hold the output of prepare_content_item_data for testing or actual rendering
#[derive(Debug, PartialEq)]
struct RenderItemOutput {
    body_content: String,
    output_directory: String,
    slug: String,
    final_layout_string: String,
    item_global_variables: HashMap<String, String>,
}

// Modified function to prepare data for rendering a single content item (post or page)
// It now returns RenderItemOutput or an error.
fn prepare_content_item_data(
    item_data: &HashMap<String, String>,
    item_type: &str, // "post" or "page"
    main_layout_template: &str,
    base_main_layout_variables: &HashMap<String, String>,
    includes: &HashMap<String, String>, // Still needed for post_template
    site_global_variables: &HashMap<String, String>,
    config: &Config,
) -> Result<RenderItemOutput, MyError> {
    let mut item_global_variables = site_global_variables.clone();
    let mut item_main_layout_variables = base_main_layout_variables.clone();

    let title = item_data.get("title").cloned().unwrap_or_default();
    item_global_variables.insert(
        "title".to_string(),
        format!("{} - {}", title, site_global_variables.get("title").unwrap_or(&"".to_string())),
    );

    let slug = item_data.get("slug").cloned().unwrap_or_default();
    if slug.is_empty() {
        return Err(MyError::Io(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Slug is empty")));
    }
    
    let body_content: String;
    let output_directory: String;

    if item_type == "post" {
        let posts_output_path = PathBuf::from(&config.output_dir).join("posts");
        output_directory = posts_output_path.to_str().ok_or_else(|| MyError::Io(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid output_dir for posts")))?.to_string() + "/";
        let post_template = includes.get("post.liquid").cloned().unwrap_or_default();
        body_content = process_template_tags(&post_template, item_data);
        item_main_layout_variables.insert("pathname".to_string(), format!("posts/{}", slug));
    } else if item_type == "page" {
        output_directory = config.output_dir.clone() + "/";
        body_content = item_data.get("content").cloned().unwrap_or_default();
        item_main_layout_variables.insert("pathname".to_string(), slug.clone());
    } else {
        return Err(MyError::Io(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid item_type")));
    }

    let final_layout_string = replace_template_variables(main_layout_template, &item_main_layout_variables);

    Ok(RenderItemOutput {
        body_content,
        output_directory,
        slug,
        final_layout_string,
        item_global_variables,
    })
}

pub fn generate() -> Result<(), MyError> {
    // Load configuration
    let config = Config::load("config.toml")?;

    // Load all site data using the config
    let site_data = load_site_data(&config)?;

    // Initialize global variables for templating
    let mut global_variables = HashMap::new();
    global_variables.insert(
        "title".to_string(),
        "Andor Polgar's Visual Journal".to_string(), // Site's main title
    );

    // Prepare base main layout variables
    let mut base_main_layout_variables = HashMap::new();
    base_main_layout_variables.insert("css_file_name".to_string(), site_data.css_file_name.clone());
    base_main_layout_variables.insert("generated_date".to_string(), site_data.generated_date.clone());
    
    // Process main layout with initial variables (CSS, generated date)
    let main_layout_processed_initial = replace_template_variables(
        &site_data.main_layout_template,
        &base_main_layout_variables,
    );

    // Generate pagination pages
    generate_pagination_pages(
        5,
        &site_data.posts,
        &site_data.includes,
        &main_layout_processed_initial,
        &global_variables,
        &config,
    )?;

    // Generate index page
    generate_index_page(
        &site_data.posts,
        &site_data.includes,
        &main_layout_processed_initial,
        &global_variables,
        &config,
    )?;

    // Generate posts
    for post_data in &site_data.posts {
        let render_data = prepare_content_item_data(
            post_data,
            "post",
            &site_data.main_layout_template,
            &base_main_layout_variables,
            &site_data.includes,
            &global_variables,
            &config,
        )?;
        render_page(
            &render_data.body_content,
            &render_data.output_directory,
            &render_data.slug,
            &render_data.final_layout_string,
            &site_data.includes, // render_page needs this for {% include %} within final content
            &render_data.item_global_variables,
        )?;
    }

    // Generate pages
    for page_data in &site_data.pages {
        let render_data = prepare_content_item_data(
            page_data,
            "page",
            &site_data.main_layout_template,
            &base_main_layout_variables,
            &site_data.includes, // `includes` isn't strictly used for page body_content but render_page expects it
            &global_variables,
            &config,
        )?;
        render_page(
            &render_data.body_content,
            &render_data.output_directory,
            &render_data.slug,
            &render_data.final_layout_string,
            &site_data.includes, // render_page needs this
            &render_data.item_global_variables,
        )?;
    }

    Ok(())
}
