import React from 'react';
import { Helmet } from 'react-helmet';
import Default from './Default';

export default ({page}) => {
	return (
		<Default>
			<Helmet>
				<title>{page.attributes.title}</title>
			</Helmet>
			<div dangerouslySetInnerHTML={{ __html: page.html }}></div>
		</Default>
	);
};
