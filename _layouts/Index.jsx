import React from 'react';
import Default from './Default.jsx';
import styled from 'styled-components';
import { format } from 'date-fns';

const Post = styled.div``;

const Title = styled.h2`
	font-size: 14px;
`;

export default ({ posts, pagination: { currentPage, nextPage, previousPage, pageCount }, ...props }) => {
	return (
		<Default {...props}>
			{posts.map(({ html, attributes: { title, date, location } }, index) => {
				return (
					<Post key={index}>
						<Title>{title}</Title>
						<span>{format(new Date(date), 'MMMM d, y')} &middot;</span>
						<span>{location}</span>
						<div dangerouslySetInnerHTML={{ __html: html }}></div>
					</Post>
				);
			})}
			<div>
				{previousPage && <a href={`/pages/${previousPage}.html`}>Previous</a>}
				&middot; {`Page: ${currentPage} of ${pageCount}`} &middot;
				{nextPage && <a href={`/pages/${nextPage}.html`}>Next</a>}
			</div>
		</Default>
	);
};
