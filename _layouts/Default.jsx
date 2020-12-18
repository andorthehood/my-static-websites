import React from 'react';
import { Helmet } from 'react-helmet';
import styled from 'styled-components';

const Container = styled.div`
	width: 500px;
	margin: 0 auto;

	img {
		width: 100%;
	}
`;
export default ({ children, ...props }) => {
	return (
		<Container>
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
			</Helmet>
			<div>{children}</div>
		</Container>
	);
};
