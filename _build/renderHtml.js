const ReactDOMServer = require('react-dom/server');
const React = require('react');
const Helmet = require('react-helmet').default;
const { ServerStyleSheet } = require('styled-components');

module.exports = function (Component, props) {
	const sheet = new ServerStyleSheet();
	const body = ReactDOMServer.renderToStaticMarkup(sheet.collectStyles(React.createElement(Component, props)));
	const helmet = Helmet.renderStatic();

	return {
		html: `
			<!DOCTYPE html>
				<html ${helmet.htmlAttributes.toString()}>
				<head>
					${helmet.title.toString()}
					${helmet.meta.toString()}
					${helmet.link.toString()}
					${sheet.getStyleTags()}
				</head>
				<body ${helmet.bodyAttributes.toString()}>
					<div id="root">
						${body}
					</div>
				</body>
			</html>
		`,
		css: sheet.instance.toString()
	};
};
