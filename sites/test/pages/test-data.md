---
title: Test Data Functionality
slug: test-data
---

# Testing Data Files

This page demonstrates the new data file functionality.

## Navigation Data

### Main Navigation
{% for item in data.navigation.main %}
- {{ item.icon }} [{{ item.name }}]({{ item.url }})
{% endfor %}

### Footer Navigation  
{% for item in data.navigation.footer %}
- [{{ item.name }}]({{ item.url }})
{% endfor %}

## Authors Data

### John's Info
- **Name**: {{ data.authors.john.name }}
- **Bio**: {{ data.authors.john.bio }}
- **Avatar**: {{ data.authors.john.avatar }}
- **Twitter**: {{ data.authors.john.social.twitter }}
- **GitHub**: {{ data.authors.john.social.github }}
- **Email**: {{ data.authors.john.social.email }}

### Jane's Info
- **Name**: {{ data.authors.jane.name }}
- **Bio**: {{ data.authors.jane.bio }}
- **Avatar**: {{ data.authors.jane.avatar }}
- **Twitter**: {{ data.authors.jane.social.twitter }}
- **Instagram**: {{ data.authors.jane.social.instagram }}
- **Email**: {{ data.authors.jane.social.email }}