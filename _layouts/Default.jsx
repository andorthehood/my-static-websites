import React from 'react';
import { Helmet } from 'react-helmet';
import styled, { createGlobalStyle, ThemeProvider } from 'styled-components';
import ColorScheme from './ColorScheme';

const GlobalStyle = createGlobalStyle`
	body, html {
		color: var(--text-color);
		line-height: 1.5;
		background: var(--background-color);
		font-family: "Roboto Mono",Courier,monospace;
		font-size: var(--font-size);
		margin: 0;
		padding: 0;
	}

	h1 {
		margin: 0;
		padding: 10px 0;
	}

	a {
		color: var(--text-color);
		text-decoration: none;
	}

	ul {
		list-style-position: inside;
		list-style-type: hiragana;
		margin: 0;
		padding: 10px 0;
	}

	a:hover {
		text-decoration: line-through;
	}
`;

const Title = styled.h1`
	color: var(--text-color);
	font-size: var(--font-size);
`;

const Container = styled.div`
	margin: 0 auto;
	padding: 10px 0;
	width: 50%;

	@media (max-width: 1024px) {
		width: 80%;
	}

	@media (max-width: 700px) {
		width: 100%;
	}

	img {
		display: block;
		width: 100%;
	}

	p {
		margin: 0;
		padding: 20px 0;
	}
`;

const Footer = styled.div``;

export default ({ children, ...props }) => {
	return (
		<ThemeProvider theme={{}}>
			<Container>
				<ColorScheme />
				<GlobalStyle />
				<Helmet>
					<meta charset="utf-8" />
					<meta
						name="viewport"
						content="width=device-width, initial-scale=1, maximum-scale=1, minimum-scale=1, user-scalable=no"
					/>
					<link
						href="https://fonts.googleapis.com/css?family=Roboto+Slab:300|Roboto:900&subset=latin-ext"
						rel="stylesheet"
					/>
					<link href="https://fonts.googleapis.com/css?family=Roboto+Mono" rel="stylesheet" />
					<link rel="alternate" type="application/atom+xml" title="" href="/atom.xml" />
					<link rel="canonical" href="" />
					<link href={`/style.css?c=${props.renderTime}`} rel="stylesheet" />
					<title>Andor Polgar's photo journal</title>
				</Helmet>
				<Title>
					<a href="/">Andor Polgar's photo journal</a>
				</Title>
				<div>{children}</div>
				<Footer>
					<p>
						Instagram:{' '}
						<a href="https://instagram.com/andorcover" target="_blank">
							@andorcover
						</a>{' '}
						|{' '}
						<a href="https://www.youtube.com/channel/UCF_EgDIkYFIeu-19KleLnFA" target="_blank">
							YouTube
						</a>{' '}
						| The source code of this blog:{' '}
						<a href="https://github.com/andormade/andor.cool" target="_blank">
							github
						</a>
					</p>
				</Footer>
			</Container>
		</ThemeProvider>
	);
};
