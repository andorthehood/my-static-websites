import type { AppProps } from 'next/app';
import styled, { createGlobalStyle } from 'styled-components';
import Head from 'next/head';

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

    :root {
        --background-color: #fff0ee;
        --font-size: 18px;
        --text-color: #333399;
        --title-color: #333399;
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

const Footer = styled.div`
	a {
		text-decoration: underline;
		&:hover {
			text-decoration: line-through;
		}
	}
`;

function MyApp({ Component, pageProps }: AppProps) {
	return (
		<Container>
			<GlobalStyle />
			<Head>
				<meta charSet="utf-8" />
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
				<title>Andor Polgar's photo journal</title>
			</Head>
			<Title>
				<a href="/">Andor Polgar's photo journal</a>
			</Title>
			<div>
				<Component {...pageProps} />
			</div>
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
					| My website doesn't collect your personal information, nor does it send any of it to third-party analytics
					services. 🍪 You don't have to believe me, you can see it for yourself in the source code on{' '}
					<a href="https://github.com/andormade/andor.cool" target="_blank">
						github
					</a>
					.
				</p>
			</Footer>
		</Container>
	);
}
export default MyApp;
