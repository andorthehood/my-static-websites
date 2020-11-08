require('@babel/register')({
	presets: ['@babel/react'],
	extensions: ['.jsx'],
	cache: true,
});

const fs = require('fs').promises;
const fsOld = require('fs');
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

const renderPages = async (pages, globalVariables, extractedCss = '') => {
	await Promise.all(
		pages.map(async page => {
			const { html, css } = renderHtml(page.Component, globalVariables);
			await fs.writeFile(`public/${page.fileName.toLowerCase()}.html`, html);
			extractedCss += css;
		})
	);

	return extractedCss;
};

const renderPostPages = async (postPages, globalVariables, extractedCss = '') => {
	await Promise.all(
		postPages.map(async (posts, index) => {
			const pageNumber = index + 1;
			const pagination = {
				nextPage: posts[index + 1] ? `${pageNumber + 1}.html` : undefined,
				previousPage: posts[index - 1] ? `${pageNumber - 1}.html` : undefined,
			};
			const { html, css } = renderHtml(Index, { ...globalVariables, posts, pagination });
			extractedCss += css;
			await fs.writeFile(`public/${config.folders.pages}/${pageNumber}.html`, html);
		})
	);

	return extractedCss
};

const renderPosts = async (posts, globalVariables, extractedCss = '') => {
	await Promise.all(
		posts.map(async ({ html: post }) => {
			const { html, css } = renderHtml(Post, { ...globalVariables, post });
			extractedCss += css;
			await fs.writeFile(`public/${config.folders.posts}/${post.fileName}.html`, html);
		})
	);

	return extractedCss
};

const build = async function () {
	const renderTime = Date.now();
	const posts = await getPosts();
	const pages = await getPages();
	const postPages = splitToEqualChunks(posts, config.postsPerPage);

	await createFolders(Object.values(config.folders));

	const globalVariables = {
		config,
		posts,
		renderTime,
	};

	const extractedCss = await Promise.all([
		renderPages(pages, globalVariables),
		renderPostPages(postPages, globalVariables),
		renderPosts(posts, globalVariables),
	]);

	const { styles } = new CleanCSS({ level: 2 }).minify(extractedCss.join(''));
	fs.writeFile(`public/style.css`, styles);
};

build();

console.log('Watching...');

fsOld.watch('./_layouts', { recursive: true }, (eventType, filename) => {
	console.log(eventType, filename);
	build();
});
