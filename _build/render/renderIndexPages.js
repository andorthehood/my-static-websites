const renderHtml = require('./renderHtml');
const Index = require('../../_layouts/Index.jsx').default;

module.exports = async (postPages, globalVariables, callback) => {
	await Promise.all(
		postPages.map(async (posts, index) => {
			const pageNumber = index + 1;
			const pagination = {
				nextPage: postPages[index + 1] ? pageNumber + 1 : undefined,
				previousPage: postPages[index - 1] ? pageNumber - 1 : undefined,
				pageCount: postPages.length,
				currentPage: pageNumber
			};
			const { html, css } = renderHtml(Index, { ...globalVariables, posts, pagination });
			await callback({ html, css, pageNumber });
		})
	);
};
