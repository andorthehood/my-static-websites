const fs = require('fs').promises;
const path = require('path');
const fm = require('front-matter');
const { Liquid } = require('liquidjs');
const engine = new Liquid({
	cache: true,
	root: ['./_includes/', './_layouts'],
	dynamicPartials: false,
});
const marked = require('marked');

const INCLUDES_DIR = '_includes';

module.exports = async function parseLiquidTemplateWithFrontMatter(file, globalVariables = {}) {
	const data = await fs.readFile(file, 'utf8');
	const { body, attributes } = fm(data);
	const liquidified = await engine.parseAndRender(body, { ...globalVariables, page: attributes });
	const html = marked(liquidified);
	return {
		html,
		attributes,
		fileName: path.basename(file, path.extname(file)),
	};
};
