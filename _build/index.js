require('@babel/register')({
	presets: ['@babel/react'],
	extensions: ['.jsx'],
	cache: true
});

const fs = require('fs').promises;
const renderHtml = require('./renderHtml');
const Posts = require('../_layouts/Posts.jsx').default;
const Post = require('../_layouts/Post.jsx').default;
const getPosts = require('./getPosts');

(async function () {
	const posts = await getPosts();

	await fs.mkdir('public/posts', { recursive: true });

	posts.forEach(post => {
		const html = renderHtml(Post, { post });
		fs.writeFile('public/posts/' + post.fileName + '.html', html);
	});
})();
