const ReactDOMServer = require('react-dom/server');
const React = require('react');
const Helmet = require('react-helmet').default;

module.exports = function (Component, props) {
	const body = ReactDOMServer.renderToStaticMarkup(React.createElement(Component, props));
	const helmet = Helmet.renderStatic();

	return `
		<!DOCTYPE html>
			<html ${helmet.htmlAttributes.toString()}>
			<head>
				${helmet.title.toString()}
				${helmet.meta.toString()}
				${helmet.link.toString()}
			</head>
			<body ${helmet.bodyAttributes.toString()}>
				<div id="root">
					${body}
				</div>
			</body>
		</html>
	`;
}
