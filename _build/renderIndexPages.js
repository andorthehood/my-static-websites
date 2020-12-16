const renderHtml = require('./renderHtml');
const Index = require('../_layouts/Index.jsx').default;

module.exports = async (postPages, globalVariables, callback) => {
	await Promise.all(
		postPages.map(async (posts, index) => {
			const pageNumber = index + 1;
			const pagination = {
				nextPage: posts[index + 1] ? `${pageNumber + 1}.html` : undefined,
				previousPage: posts[index - 1] ? `${pageNumber - 1}.html` : undefined,
			};
			const { html, css } = renderHtml(Index, { ...globalVariables, posts, pagination });
			await callback({ html, css, pageNumber });
		})
	);
};
