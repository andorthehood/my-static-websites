const renderHtml = require('./renderHtml');

module.exports = async (pages, globalVariables, callback) => {
	await Promise.all(
		pages.map(async page => {
			const { html, css } = renderHtml(page.Component, { ...globalVariables, page });
			await callback({ html, css, page });
		})
	);
};
