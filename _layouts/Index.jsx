import React from 'react';
import Default from './Default.jsx';
import styled from 'styled-components';
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
`;

const Title = styled.h2`
	font-size: 14px;
	color: var(--title-color);
`;

export default ({ posts, pagination: { currentPage, nextPage, previousPage, pageCount }, ...props }) => {
	return (
		<Default {...props}>
			{posts.map(({ html, fileName, attributes: { title, date, location } }, index) => {
				return (
					<Post key={index}>
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
