const fs = require('fs').promises;
const parseLiquidTemplateWithFrontMatter = require('./templateParser');

module.exports = async postsDir => {
	const postFiles = await fs.readdir(postsDir);
	const posts = await Promise.all(
		postFiles.map(async file => {
			const data = await parseLiquidTemplateWithFrontMatter(`${postsDir}/${file}`);
			const { mtimeMs } = await fs.stat(`${postsDir}/${file}`);

			return {
				...data,
				mtime: Math.floor(mtimeMs / 1000),
				ctime: data.attributes.date ? new Date(data.attributes.date).getTime() / 1000 : 0,
			};
		})
	);

	return posts.sort((a, b) => (a.ctime > b.ctime ? -1 : a.ctime < b.ctime ? 1 : 0));
};
