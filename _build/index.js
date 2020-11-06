require('@babel/register')({
	presets: ['@babel/react'],
	extensions: ['.jsx'],
	cache: true,
});

const fs = require('fs').promises;
const renderHtml = require('./renderHtml');
const Posts = require('../_layouts/Posts.jsx').default;
const Post = require('../_layouts/Post.jsx').default;
const Index = require('../_pages/Index.jsx').default;
const getPosts = require('./getPosts');
const getPages = require('./getPages');
const config = require('./config');
const splitToEqualChunks = require('./utils/splitToEqualChunks');
const createFolders = require('./utils/createFolders');

(async function () {
	const posts = await getPosts();
	const pages = await getPages();
	const postPages = splitToEqualChunks(posts, config.postsPerPage);

	await createFolders(Object.values(config.folders));

	const globalVariables = {
		config,
		posts
	};

	pages.forEach(page => {
		const html = renderHtml(page.Component, globalVariables);
		fs.writeFile(`public/${page.fileName.toLowerCase()}.html`, html);
	});

	postPages.forEach(async (posts, index) => {
		const pageNumber = index + 1;
		const pagination = {
			nextPage: posts[index + 1] ? `${pageNumber + 1}.html` : undefined,
			previousPage: posts[index - 1] ? `${pageNumber - 1}.html` : undefined,
		};
		const html = renderHtml(Index, { ...globalVariables, posts, pagination });
		fs.writeFile(`public/${config.folders.pages}/${pageNumber}.html`, html);
	});

	posts.forEach(post => {
		const html = renderHtml(Post, { ...globalVariables, post });
		fs.writeFile(`public/${config.folders.posts}/${post.fileName}.html`, html);
	});
})();
