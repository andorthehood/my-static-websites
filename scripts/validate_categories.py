#!/usr/bin/env python3
"""
Validation script to check for invalid category front matter usage.
This can be run as a pre-commit hook or part of CI to prevent regressions.

Usage: python3 validate_categories.py

Checks for:
- Use of 'categories' (plural) field - should be 'category' (singular)
- Placeholder values like 'post', 'posts'
- Array syntax in category values
"""

import os
import re
import sys
from pathlib import Path

def extract_front_matter(content):
    """Extract YAML front matter from a markdown file."""
    match = re.match(r'^---\n(.*?)\n---', content, re.DOTALL)
    if match:
        return match.group(1)
    return None

def validate_front_matter(file_path, front_matter):
    """Validate front matter for category issues."""
    issues = []
    
    for line in front_matter.split('\n'):
        line = line.strip()
        
        # Check for plural 'categories' field
        if line.startswith('categories:'):
            issues.append(f"Use 'category' instead of 'categories' (line: {line})")
        
        # Check for invalid category values
        elif line.startswith('category:'):
            value = line.split(':', 1)[1].strip()
            
            # Check for placeholder values
            if value in ['post', 'posts', '"post"', "'post'", '"posts"', "'posts'"]:
                issues.append(f"Remove placeholder category value (line: {line})")
            
            # Check for array syntax
            elif value.startswith('[') and value.endswith(']'):
                issues.append(f"Use single category value instead of array (line: {line})")
    
    return issues

def validate_file(file_path):
    """Validate a single markdown file."""
    try:
        content = file_path.read_text(encoding='utf-8')
        front_matter = extract_front_matter(content)
        
        if front_matter:
            issues = validate_front_matter(file_path, front_matter)
            return issues
        
        return []
    except Exception as e:
        return [f"Error reading file: {e}"]

def validate_site(site_path):
    """Validate all posts in a site."""
    all_issues = []
    
    # Check active posts
    posts_dir = site_path / 'posts'
    if posts_dir.exists():
        for post_file in posts_dir.glob('*.md'):
            issues = validate_file(post_file)
            if issues:
                relative_path = post_file.relative_to(Path.cwd())
                all_issues.append((str(relative_path), issues))
    
    # Check archived posts
    archived_dir = site_path / '_archived'
    if archived_dir.exists():
        for post_file in archived_dir.glob('*.md'):
            issues = validate_file(post_file)
            if issues:
                relative_path = post_file.relative_to(Path.cwd())
                all_issues.append((str(relative_path), issues))
    
    return all_issues

def main():
    """Main validation function."""
    base_path = Path.cwd()
    sites_path = base_path / 'sites'
    
    if not sites_path.exists():
        print("❌ sites/ directory not found. Run this script from the project root.")
        sys.exit(1)
    
    all_issues = []
    
    for site_dir in sites_path.iterdir():
        if site_dir.is_dir():
            site_issues = validate_site(site_dir)
            all_issues.extend(site_issues)
    
    if all_issues:
        print("❌ Category validation failed!")
        print()
        for file_path, issues in all_issues:
            print(f"File: {file_path}")
            for issue in issues:
                print(f"  - {issue}")
            print()
        
        print("Guidelines:")
        print("- Use 'category: value' instead of 'categories: [value]'")
        print("- Remove placeholder values like 'post' or 'posts'")
        print("- Use single meaningful category values")
        print("- If no meaningful category exists, omit the field entirely")
        
        sys.exit(1)
    else:
        print("✅ All category front matter is valid!")
        sys.exit(0)

if __name__ == '__main__':
    main()