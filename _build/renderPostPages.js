const renderHtml = require('./renderHtml');
const fs = require('fs').promises;
const Index = require('../_pages/Index.jsx').default;
const config = require('./config');

module.exports = async (postPages, globalVariables, extractedCss = '') => {
	await Promise.all(
		postPages.map(async (posts, index) => {
			const pageNumber = index + 1;
			const pagination = {
				nextPage: posts[index + 1] ? `${pageNumber + 1}.html` : undefined,
				previousPage: posts[index - 1] ? `${pageNumber - 1}.html` : undefined,
			};
			const { html, css } = renderHtml(Index, { ...globalVariables, posts, pagination });
			extractedCss += css;
			await fs.writeFile(`public/${config.folders.pages}/${pageNumber}.html`, html);
		})
	);

	return extractedCss;
};
