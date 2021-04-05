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
					<!-- Global site tag (gtag.js) - Google Analytics -->
					<script async src="https://www.googletagmanager.com/gtag/js?id=UA-82349820-1"></script>
					<script>
						window.dataLayer = window.dataLayer || [];
						function gtag(){dataLayer.push(arguments);}
						gtag('js', new Date());
						gtag('config', 'UA-82349820-1');
					</script>
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
		`,
		css: sheet.instance.toString(),
	};
};
