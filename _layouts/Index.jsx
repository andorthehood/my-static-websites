import React from 'react';
import Default from './Default.jsx';

export default ({ posts, ...props }) => {
	return (
		<Default {...props}>
			{posts.map(({ html }, index) => {
				return <div key={index} dangerouslySetInnerHTML={{ __html: html }}></div>;
			})}
		</Default>
	);
};
