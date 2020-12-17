const fs = require('fs').promises;
const parseLiquidTemplateWithFrontMatter = require('./templateParser');

module.exports = async postsDir => {
	const postFiles = await fs.readdir(postsDir);
	return await Promise.all(
		postFiles.map(async file => {
			const data = await parseLiquidTemplateWithFrontMatter(`${postsDir}/${file}`);
			const { mtimeMs } = await fs.stat(`${postsDir}/${file}`);

			return {
				...data,
				mtime: Math.floor(mtimeMs / 1000),
			};
		})
	);
};
