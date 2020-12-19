import React from 'react';
import { Helmet } from 'react-helmet';
import styled, { createGlobalStyle } from 'styled-components';

const GlobalStyle = createGlobalStyle`
	body, html {
		padding: 0;
		margin: 0;
		font-family: "Roboto Mono",Courier,monospace;
		font-size: 14px;
	}
`;

const Container = styled.div`
	width: 50%;
	margin: 0 auto;

	@media (max-width: 1024px) {
		width: 80%;
	}

	@media (max-width: 700px) {
		width: 100%;
	}

	img {
		width: 100%;
	}
`;

export default ({ children, ...props }) => {
	return (
		<Container>
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
			<div>{children}</div>
		</Container>
	);
};
