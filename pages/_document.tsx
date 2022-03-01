import Document, { Html, Head, Main, NextScript } from 'next/document';

export default class MyDocument extends Document {
	render() {
		return (
			<Html>
				<Head>
					<meta charSet="utf-8" />
					<link rel="alternate" type="application/atom+xml" title="" href="/atom.xml" />
					<link rel="alternate" type="application/rss+xml" href="/rss.xml" title="RSS 2.0" />
					<link rel="canonical" href="" />
					<link href="https://fonts.googleapis.com/css?family=Roboto+Mono" rel="stylesheet" />
				</Head>
				<body>
					<Main />
					<NextScript />
				</body>
			</Html>
		);
	}
}
