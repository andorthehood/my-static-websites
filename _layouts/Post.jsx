import React from 'react';
import { Helmet } from 'react-helmet';

export default ({ post }) => {
	return (
		<>
			<Helmet>
				<title>{post.attributes.title}</title>
			</Helmet>
			<div dangerouslySetInnerHTML={{ __html: post.html }}></div>
		</>
	);
};
