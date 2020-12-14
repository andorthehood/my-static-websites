const renderHtml = require('./renderHtml');
const fs = require('fs').promises;
const config = require('./config');
const Post = require('../_layouts/Post.jsx').default;

module.exports = async (posts, globalVariables, extractedCss = '') => {
	await Promise.all(
		posts.map(async ({ html: post }) => {
			const { html, css } = renderHtml(Post, { ...globalVariables, post });
			extractedCss += css;
			await fs.writeFile(`public/${config.folders.posts}/${post.fileName}.html`, html);
		})
	);

	return extractedCss;
};
