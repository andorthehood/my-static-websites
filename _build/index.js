require('@babel/register')({
	presets: ['@babel/react'],
	extensions: ['.jsx'],
	cache: true,
});

const fs = require('fs').promises;
const renderHtml = require('./renderHtml');
const Posts = require('../_layouts/Posts.jsx').default;
const Post = require('../_layouts/Post.jsx').default;
const getPosts = require('./getPosts');
const getPages = require('./getPages');

(async function () {
	const posts = await getPosts();
	const pages = await getPages();

	const globalVariables = {
		...posts,
	};

	pages.forEach(page => {
		const html = renderHtml(page.Component, globalVariables);
		fs.writeFile('public/' + page.fileName.toLowerCase() + '.html', html);
	});

	await fs.mkdir('public/posts', { recursive: true });

	posts.forEach(post => {
		const html = renderHtml(Post, { post, ...globalVariables });
		fs.writeFile('public/posts/' + post.fileName + '.html', html);
	});
})();
