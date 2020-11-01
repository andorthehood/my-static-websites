const fs = require('fs').promises;
const renderHtml = require('./renderHtml');
const Posts = require('../lib/Posts').default;
const Post = require('../lib/Post').default;
const getPosts = require('./getPosts');

(async function () {
	const posts = await getPosts();
	//	const html = renderHtml(Posts, { posts });

	await fs.mkdir('public/posts', { recursive: true });

	posts.forEach(post => {
		const html = renderHtml(Post, { post });
		fs.writeFile('public/posts/' + post.fileName + '.html', html);
	});
})();
