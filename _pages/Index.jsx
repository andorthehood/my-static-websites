import React from 'react';
import Default from '../_layouts/Default.jsx';

export default ({ posts }) => {
	return (
		<Default>
			{posts.map(({ html }, index) => {
				return <div key={index} dangerouslySetInnerHTML={{ __html: html }}></div>;
			})}
		</Default>
	);
};
