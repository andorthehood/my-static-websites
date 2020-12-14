const renderHtml = require('./renderHtml');
const fs = require('fs').promises;

module.exports = async (pages, globalVariables, extractedCss = '') => {
	await Promise.all(
		pages.map(async page => {
			const { html, css } = renderHtml(page.Component, globalVariables);
			await fs.writeFile(`public/${page.fileName.toLowerCase()}.html`, html);
			extractedCss += css;
		})
	);

	return extractedCss;
};
