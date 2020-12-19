const fs = require('fs').promises;
const asyncRequire = require('../utils/asyncRequire');
const path = require('path');

module.exports = async pagesDir => {
	const pageFiles = await fs.readdir(pagesDir);

	return Promise.all(
		pageFiles.map(async file => {
			const page = await asyncRequire(`../../${pagesDir}/${file}`);
			const { mtimeMs } = await fs.stat(`${pagesDir}/${file}`);

			return {
				Component: page.default,
				fileName: path.basename(file, path.extname(file)),
				mtime: Math.floor(mtimeMs / 1000),
			};
		})
	);
};
