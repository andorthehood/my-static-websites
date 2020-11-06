const fs = require('fs').promises;

module.exports = async folders => {
	return Promise.all(
		folders.map(async folder => {
			await fs.mkdir(`public/${folder}`, { recursive: true });
		})
	);
};