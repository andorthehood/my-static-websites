use crate::{
    error::Result,
    render_page::render_page,
    template_processors::process_template_tags,
    types::{ContentCollection, TemplateIncludes, Variables},
};

pub fn generate_pagination_pages(
    posts_per_page: usize,
    posts: &ContentCollection,
    includes: &TemplateIncludes,
    main_layout: &str,
    global_variables: &Variables,
) -> Result<()> {
    let total_pages = (posts.len() as f64 / posts_per_page as f64).ceil() as usize;

    for page_num in 1..=total_pages {
        let start = (page_num - 1) * posts_per_page;
        let end = std::cmp::min(start + posts_per_page, posts.len());
        let page_posts = &posts[start..end];

        let mut html_list = String::new();
        for post in page_posts {
            html_list.push_str(&process_template_tags(
                &includes
                    .get("list_item.liquid")
                    .cloned()
                    .unwrap_or_default(),
                &post,
            )?);
        }

        let mut variables = global_variables.clone();
        variables.insert("content".to_string(), html_list);

        render_page(
            "",
            "out/",
            &format!("page{}", page_num),
            main_layout,
            includes,
            &variables,
        )?;
    }

    Ok(())
}
