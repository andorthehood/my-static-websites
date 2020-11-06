const fs = require('fs').promises;
const parseLiquidTemplateWithFrontMatter = require('./templateParser');

const POSTS_DIR = '_posts';

module.exports = async () => {
	const postFiles = await fs.readdir(POSTS_DIR);
	return await Promise.all(
		postFiles.map(async file => {
			return await parseLiquidTemplateWithFrontMatter(`${POSTS_DIR}/${file}`);
		})
	);
}