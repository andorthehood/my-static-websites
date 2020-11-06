import React from 'react';
import Default from './Default';

export default ({ posts }) => {
	return posts.map(({ html }, index) => {
		return (
			<Default>
				<div dangerouslySetInnerHTML={{ __html: html }} key={index}></div>
			</Default>
		);
	});
};
