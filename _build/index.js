require('@babel/register')({
	presets: ['@babel/react'],
	extensions: ['.jsx'],
	cache: true,
});

const fs = require('fs').promises;
const getPosts = require('./getPosts');
const getPages = require('./getPages');
const config = require('./config');
const splitToEqualChunks = require('./utils/splitToEqualChunks');
const createFolders = require('./utils/createFolders');
const CleanCSS = require('clean-css');
const chokidar = require('chokidar');
const renderPages = require('./renderPages');
const renderPostPages = require('./renderPostPages');
const renderPosts = require('./renderPosts');

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

chokidar.watch('./_layouts').on('all', (event, path) => {
	console.log(event, path);
	build();
});
