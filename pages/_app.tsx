import type { AppProps } from 'next/app';
import styled, { createGlobalStyle } from 'styled-components';
import Link from 'next/link';
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
		text-decoration: underline;
	}

	ul {
		list-style-position: inside;
		list-style-type: hiragana;
		margin: 0;
		padding: 10px 0;
	}

	a:hover, a:visited {
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
	a {
		text-decoration: none;
	}
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
			<Head>
				<title>Andor Polgar&apos;s photo journal</title>
				<meta
					name="viewport"
					content="width=device-width, initial-scale=1, maximum-scale=1, minimum-scale=1, user-scalable=no"
				/>
			</Head>
			<GlobalStyle />
			<Title>
				<Link href="/">Andor Polgar&apos;s photo journal</Link>
			</Title>
			<div>
				<Component {...pageProps} />
			</div>
			<Footer>
				<p>
					Instagram: <Link href="https://instagram.com/andorcover">@andorcover</Link> |{' '}
					<Link href="https://www.youtube.com/channel/UCF_EgDIkYFIeu-19KleLnFA">YouTube</Link> | My website doesn&apos;t
					collect your personal information, nor does it send any of it to third-party analytics services. 🍪 You
					don&apos;t have to believe me, you can see it for yourself in the source code on{' '}
					<Link href="https://github.com/andormade/andor.cool">github</Link>.
				</p>
			</Footer>
		</Container>
	);
}
export default MyApp;
