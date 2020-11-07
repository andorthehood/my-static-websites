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
const CleanCSS = require('clean-css');

(async function () {
	const renderTime = Date.now();
	const posts = await getPosts();
	const pages = await getPages();
	const postPages = splitToEqualChunks(posts, config.postsPerPage);
	let globalCss = '';

	await createFolders(Object.values(config.folders));

	const globalVariables = {
		config,
		posts,
		renderTime,
	};

	pages.forEach(page => {
		const { html, css } = renderHtml(page.Component, globalVariables);
		fs.writeFile(`public/${page.fileName.toLowerCase()}.html`, html);
		globalCss += css;
	});

	postPages.forEach(async (posts, index) => {
		const pageNumber = index + 1;
		const pagination = {
			nextPage: posts[index + 1] ? `${pageNumber + 1}.html` : undefined,
			previousPage: posts[index - 1] ? `${pageNumber - 1}.html` : undefined,
		};
		const { html, css } = renderHtml(Index, { ...globalVariables, posts, pagination });
		fs.writeFile(`public/${config.folders.pages}/${pageNumber}.html`, html);
		globalCss += css;
	});

	posts.forEach(post => {
		const { html, css } = renderHtml(Post, { ...globalVariables, post });
		fs.writeFile(`public/${config.folders.posts}/${post.fileName}.html`, html);
		globalCss += css;
	});

	const { styles } = new CleanCSS({ level: 2 }).minify(globalCss);
	fs.writeFile(`public/style.css`, styles);
})();
