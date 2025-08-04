{% for post in posts %}
- **{{ post.title }}** ({{ post.date }}) - [{{ post.slug }}](/posts/{{ post.slug }}.html)
{% endfor %}