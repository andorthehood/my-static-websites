import React from 'react';

export default ({ posts }) => {
	return (
		<div>
			{posts.map(({ html }, index) => {
				return <div key={index} dangerouslySetInnerHTML={{ __html: html }}></div>;
			})}
		</div>
	);
};
