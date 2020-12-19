import React from 'react';
import Default from './Default.jsx';
import styled, { css } from 'styled-components';
import { format } from 'date-fns';

const Post = styled.div`
	color: var(--text-color);

	p {
		margin: 0;
		padding: 40px 0;
	}

	img {
		width: 100%;
	}

	${({ randomOffsets }) =>
		randomOffsets
			.map((offset, index) => {
				return `
				p:nth-of-type(${index}) {
					padding-left: ${offset}%;
					padding-right: ${10 - offset}%;
					width: 90%;
				}
			`;
			})
			.join('\n')}
`;

const Title = styled.h2`
	font-size: 14px;
	color: var(--title-color);
`;

export default ({ posts, pagination: { currentPage, nextPage, previousPage, pageCount }, ...props }) => {
	return (
		<Default {...props}>
			{posts.map(({ html, fileName, attributes: { title, date, location } }, index) => {
				const max = 10;
				const min = 0;
				const randomOffsets = new Array(10).fill(0).map(() => Math.floor(Math.random() * (max - min) + min));
				console.log(randomOffsets);
				return (
					<Post key={index} randomOffsets={randomOffsets}>
						<Title>
							<a href={`/posts/${fileName}`}>{title}</a>
						</Title>
						<span>{format(new Date(date), 'MMMM d, y')} &middot;</span>
						<span>{location}</span>
						<div dangerouslySetInnerHTML={{ __html: html }}></div>
					</Post>
				);
			})}
			<div>
				{previousPage && <a href={`/pages/${previousPage}`}>Previous</a>}
				&middot; {`Page: ${currentPage} of ${pageCount}`} &middot;
				{nextPage && <a href={`/pages/${nextPage}`}>Next</a>}
			</div>
		</Default>
	);
};
