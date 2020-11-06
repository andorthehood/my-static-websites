import React from 'react';
import { Helmet } from 'react-helmet';
import Default from './Default';

export default ({ post }) => {
	return (
		<Default>
			<Helmet>
				<title>{post.attributes.title}</title>
			</Helmet>
			<div dangerouslySetInnerHTML={{ __html: post.html }}></div>
		</Default>
	);
};
