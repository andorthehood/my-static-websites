---
layout: post
title: 'Lighthouse optimisation checklist'
date: 2026-03-23
unlisted: true
---

<h2>Performance</h2>
<ul>
<li>Move inline scripts to external files — required for CSP enforcement without <code>unsafe-inline</code>, also improves cacheability</li>
<li>Add <code>defer</code> to script tags — prevents the browser from blocking HTML parsing while downloading and executing the script</li>
<li>Add <code>&lt;link rel="preload"&gt;</code> for the LCP image — the browser can't discover CSS background images until the stylesheet is parsed; preloading makes it fetch earlier</li>
<li>Add <code>&lt;link rel="preload"&gt;</code> for the stylesheet — starts fetching the CSS before the parser reaches the <code>&lt;link&gt;</code> tag</li>
<li>Add explicit <code>width</code> and <code>height</code> attributes to <code>&lt;img&gt;</code> elements — lets the browser reserve the correct space before images load, preventing layout shift (CLS)</li>
<li>Add <code>height: auto</code> in CSS to preserve aspect ratio alongside explicit dimensions — explicit <code>height</code> in HTML without <code>auto</code> in CSS causes stretching</li>
<li>Add <code>will-change: background-position</code> on frequently animated elements — hints to the browser to promote the element to its own compositor layer, avoiding repaints</li>
</ul>

<h2>SEO</h2>
<ul>
<li>Fill in <code>&lt;meta name="description"&gt;</code> with meaningful content — used by search engines as the page summary in results</li>
<li>Add a valid <code>robots.txt</code> — tells crawlers which parts of the site they can index; absence is flagged as an error</li>
</ul>

<h2>Accessibility</h2>
<ul>
<li>Add <code>&lt;main&gt;</code> element wrapping the page content — allows screen readers to skip directly to the main content</li>
<li>Add <code>aria-label</code> to distinguish multiple <code>&lt;nav&gt;</code> elements — screen readers announce each nav by label; without it they're indistinguishable</li>
<li>Add <code>aria-label</code> to external links that open in a new tab — warns screen reader users that the link opens a new context</li>
<li>Add <code>aria-label</code> to code blocks describing their content — gives screen reader users context for what the code illustrates</li>
<li>Add <code>alt</code> text to all <code>&lt;img&gt;</code> elements — screen readers read this aloud; without it the image is announced as its filename or skipped entirely</li>
<li>Respect <code>prefers-reduced-motion</code> for animations — frequent motion can cause problems for users with vestibular disorders</li>
</ul>

<h2>Security (also scored by Lighthouse)</h2>
<ul>
<li>Serve CSP as an HTTP header rather than a <code>&lt;meta&gt;</code> tag — HTTP headers are enforced before any HTML is parsed; <code>&lt;meta&gt;</code> CSP is applied later and supports fewer directives</li>
<li>Add <code>default-src</code>, <code>script-src</code>, <code>style-src</code>, <code>font-src</code>, <code>img-src</code>, <code>connect-src</code>, <code>frame-src</code> directives to CSP — restricts which origins each resource type can be loaded from, limiting the blast radius of an XSS attack</li>
<li>Add <code>require-trusted-types-for 'script'</code> to CSP — forces all DOM sink assignments to go through a policy, preventing XSS at the injection point. Only applicable if the site does not use <code>innerHTML</code> or similar DOM sinks directly; enabling it on a site that does will break client-side navigation and dynamic content injection</li>
<li>Set <code>connect-src</code> to the appropriate origin — use <code>'none'</code> only if the page makes no <code>fetch()</code> or <code>XHR</code> requests; use <code>'self'</code> if it fetches from the same origin (e.g. for client-side navigation that loads JSON)</li>
<li>Add <code>Strict-Transport-Security</code> with <code>includeSubDomains</code> and <code>preload</code> — forces HTTPS for all connections including subdomains, and allows the domain to be hardcoded into browsers</li>
<li>Add <code>Cross-Origin-Opener-Policy: same-origin</code> — prevents other sites from getting a reference to your page via <code>window.opener</code>, blocking cross-origin attacks</li>
<li>Add <code>X-Frame-Options: DENY</code> — prevents the page from being embedded in an iframe, blocking clickjacking attacks</li>
<li>Add font preload <code>Link</code> header with <code>crossorigin</code> — starts fetching the font before the HTML is parsed; <code>crossorigin</code> is required to avoid the browser fetching it twice</li>
</ul>
