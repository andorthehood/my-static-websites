# Category Front Matter Guidelines

This document provides guidelines for using the `category` field in post front matter to ensure consistency and enable category-based features.

## Rules

### Use Singular `category` Field
- ✅ Use `category: music` 
- ❌ Do not use `categories: [music]`

### Single Meaningful Value
- ✅ Use one meaningful category: `category: music`
- ❌ Do not use placeholder values: `category: post` or `category: posts`
- ✅ If no meaningful category exists, omit the field entirely

### Supported Categories

Based on the current content, these categories are used:

- `music` - Posts about music, instruments, concerts, etc.
- `cinemascope` - Posts featuring anamorphic/cinemascope photography
- `travel` - Travel-related posts
- `artnude` - Artistic nude photography

## Examples

### Good Examples

```yaml
---
layout: post
title: 'Concert Photography'
category: music
---
```

```yaml
---
layout: post
title: 'Portrait Session'
# No category field if no meaningful category applies
---
```

### Bad Examples

```yaml
---
layout: post
title: 'Concert Photography'
categories: [music]  # ❌ Use singular 'category'
---
```

```yaml
---
layout: post
title: 'Random Post'
category: post  # ❌ Remove placeholder values
---
```

## Validation

Run the validation script to check for compliance:

```bash
python3 scripts/validate_categories.py
```

This script will:
- Check for use of `categories` (plural) field
- Detect placeholder values like `post`/`posts`
- Flag array syntax usage
- Provide helpful error messages

## Background

This normalization was completed as part of TODO 011 to prepare for category-based pagination and navigation features. Consistent category metadata enables reliable grouping and filtering of posts.