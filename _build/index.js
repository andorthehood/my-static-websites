require('@babel/register')({
	presets: ['@babel/react'],
	extensions: ['.jsx'],
	cache: true,
});

const fs = require('fs').promises;
const getPosts = require('./data/getPosts');
const getPages = require('./data/getPages');
const config = require('./config');
const splitToEqualChunks = require('./utils/splitToEqualChunks');
const createFolders = require('./utils/createFolders');
const CleanCSS = require('clean-css');
const chokidar = require('chokidar');
const renderPages = require('./render/renderPages');
const renderIndexPages = require('./render/renderIndexPages');
const devserver = require('./devserver');
const Post = require('../_layouts/Post.jsx').default;
const debounce = require('lodash.debounce');

const build = async function () {
	const renderTime = Date.now();
	const posts = (await getPosts()).map(post => ({ ...post, Component: Post }));
	const pages = await getPages();
	const indexPages = splitToEqualChunks(posts, config.postsPerPage);

	await createFolders(Object.values(config.folders));

	const globalVariables = {
		config,
		posts,
		renderTime,
	};

	let extractedCss = '';

	await renderPages(pages, globalVariables, async ({ html, css, page: { fileName } }) => {
		await fs.writeFile(`public/${fileName.toLowerCase()}.html`, html);
		extractedCss += css;
	});
	await renderIndexPages(indexPages, globalVariables, async ({ pageNumber, html, css }) => {
		await fs.writeFile(`public/${config.folders.pages}/${pageNumber}.html`, html);
		extractedCss += css;
	});
	await renderPages(posts, globalVariables, async ({ html, css, page: { fileName } }) => {
		await fs.writeFile(`public/${config.folders.posts}/${fileName}.html`, html);
		extractedCss += css;
	});

	const { styles } = new CleanCSS({ level: 2 }).minify(extractedCss);
	fs.writeFile(`public/style.css`, styles);

	console.log((Date.now()-renderTime) + 'ms');
};

build();

console.log('Watching...');

chokidar.watch('./_layouts').on('all', debounce((event, path) => {
	build();
}, 1000));

devserver();
