---
title: Nested Render Test
---

# Testing Nested Renders

This page tests nested render functionality:

{% render 'components/buttons/cta' text:"Get Started" %}

{% render layout/sidebar title:"Navigation" %}

{% render 'components/card' title:"Welcome" description:"This is a test card" %}