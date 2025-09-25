# Pagination Layout Templates

This feature adds support for configurable pagination layouts in the lepkefing static site generator. Instead of being limited to hardcoded HTML for pagination pages, sites can now define custom layouts that integrate with the existing template system.

## Configuration

Add the following settings to your site's `config.md` to enable custom pagination layouts:

```yaml
---
title: My Site
posts_per_page: 10
pagination_layout: pagination        # Optional: layout for regular pagination pages
category_pagination_layout: category-pagination  # Optional: layout for category pagination pages
---
```

## Layout Files

Create layout files in your site's `layouts/` directory:

- `pagination.html` - Used for regular pagination pages (e.g., `/page1.html`, `/page2.html`)
- `category-pagination.html` - Used for category pagination pages (e.g., `/category/tech/page1.html`)

## Available Template Variables

### Regular Pagination Layout

Your pagination layout has access to these variables:

- `page_number` - Current page number (e.g., "1", "2")
- `total_pages` - Total number of pagination pages
- `has_previous` - Boolean ("true"/"false") if previous page exists
- `has_next` - Boolean ("true"/"false") if next page exists  
- `previous_page_number` - Previous page number (if exists)
- `previous_page_url` - URL to previous page (e.g., "/page1")
- `next_page_number` - Next page number (if exists)
- `next_page_url` - URL to next page (e.g., "/page3")
- `page_posts.*` - Collection of posts on this page (accessible via for loops)

### Category Pagination Layout

Category layouts have all regular pagination variables plus:

- `category_name` - Display name of the category (e.g., "Technology")
- `category_slug` - URL-safe category slug (e.g., "technology")
- `category_index_url` - URL to category's first page (e.g., "/category/tech/page1")
- `site_index_url` - URL to site home ("/")

## Example Templates

### Basic Pagination Layout (`pagination.html`)

```html
<div class="pagination-page">
    <header>
        <h1>Archive - Page {{page_number}}</h1>
        <p>Showing page {{page_number}} of {{total_pages}} pages</p>
    </header>

    <section class="posts-list">
        {% for post in page_posts %}
        <article class="post-preview">
            <h2><a href="/posts/{{post.slug}}">{{post.title}}</a></h2>
            <time>{{post.date}}</time>
            <div class="excerpt">{{post.content}}</div>
        </article>
        {% endfor %}
    </section>

    <nav class="pagination-navigation">
        <ul class="pagination-links">
            {% if has_previous %}
            <li><a href="{{previous_page_url}}">← Previous</a></li>
            {% endif %}
            
            <li><a href="/">Home</a></li>
            
            {% if has_next %}
            <li><a href="{{next_page_url}}">Next →</a></li>
            {% endif %}
        </ul>
    </nav>
</div>
```

### Category Pagination Layout (`category-pagination.html`)

```html
<div class="category-pagination-page">
    <header>
        <h1>{{category_name}} - Page {{page_number}}</h1>
        <p>Posts in <strong>{{category_name}}</strong> category</p>
    </header>

    <section class="posts-list">
        {% for post in page_posts %}
        <article class="post-preview category-post">
            <h2><a href="/posts/{{post.slug}}">{{post.title}}</a></h2>
            <time>{{post.date}}</time>
            <span class="category">📁 {{post.category}}</span>
            <div class="excerpt">{{post.content}}</div>
        </article>
        {% endfor %}
    </section>

    <nav class="category-pagination-navigation">
        <ul class="pagination-links">
            {% if has_previous %}
            <li><a href="{{previous_page_url}}">← Previous</a></li>
            {% endif %}
            
            <li><a href="{{category_index_url}}">Category Index</a></li>
            <li><a href="{{site_index_url}}">Site Home</a></li>
            
            {% if has_next %}
            <li><a href="{{next_page_url}}">Next →</a></li>
            {% endif %}
        </ul>
    </nav>
</div>
```

## Fallback Behavior

The system provides robust fallback behavior:

1. **No layout configured**: Uses original hardcoded HTML pagination
2. **Layout file not found**: Shows warning message and falls back to hardcoded HTML
3. **Layout render error**: Shows warning message and falls back to hardcoded HTML
4. **Category layout preference**: 
   - First tries `category_pagination_layout` 
   - Then tries `pagination_layout`
   - Finally falls back to hardcoded HTML

This ensures existing sites continue to work without any changes, and new sites can gradually adopt custom layouts.

## Migration

Existing sites require no changes - they will continue to use the original hardcoded pagination HTML. To adopt custom layouts:

1. Add `pagination_layout` and/or `category_pagination_layout` to your `config.md`
2. Create corresponding layout files in your `layouts/` directory
3. Use the available template variables to create your desired pagination UI

## Technical Implementation

- Layouts are processed through the full template pipeline (Liquid tags, variables, includes)
- Template variables use the existing collection system (e.g., `page_posts.0.title`, `page_posts.1.title`)
- Layout resolution uses the same path-building logic as regular page layouts
- All existing tests continue to pass, ensuring backward compatibility

## Related Files

- `src/layout.rs` - Layout loading and rendering utilities
- `src/generate_pagination_pages.rs` - Regular pagination generator
- `src/generate_category_pages.rs` - Category pagination generator  
- `sites/test/layouts/pagination.html` - Example regular pagination layout
- `sites/test/layouts/category-pagination.html` - Example category pagination layout