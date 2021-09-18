import React from 'react';
import { Helmet } from 'react-helmet';
import Default from './Default';
import styled from 'styled-components';

const Button = styled.a`
	font-size: var(--font-size);
	color: var(--text-color);
	text-decoration: underline;

	&:hover {
		text-decoration: line-through;
	}
`;

export default ({ page }) => {
	return (
		<Default>
			<Helmet>
				<title>{page.attributes.title}</title>
			</Helmet>
			<div dangerouslySetInnerHTML={{ __html: page.html }}></div>
			{page.nextPost && (
				<p>
					Next post:{' '}
					<Button href={'/posts/' + page.nextPost?.fileName}>
						{page.nextPost?.attributes.title} {page.nextPost?.attributes?.emojis}
					</Button>
				</p>
			)}
		</Default>
	);
};
