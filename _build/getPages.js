const fs = require('fs').promises;
const asyncRequire = require('./asyncRequire');
const path = require('path');

const PAGES_DIR = '_pages';

module.exports = async () => {
	const pageFiles = await fs.readdir(PAGES_DIR);

	return Promise.all(
		pageFiles.map(async file => {
			const page = await asyncRequire('../' + PAGES_DIR + '/' + file);
			return {
				Component: page.default,
				fileName: path.basename(file, path.extname(file)),
			};
		})
	);
};
