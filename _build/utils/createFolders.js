const fs = require('fs').promises;

module.exports = async (folders, basePath) => {
	return Promise.all(
		folders.map(async folder => {
			await fs.mkdir(`${basePath}/${folder}`, { recursive: true });
		})
	);
};
