import React from 'react';
import Default from '../_layouts/Default.jsx';

export default props => {
	console.log(Object.keys(props));
	return (
		<Default {...props}>
			<ul>
				{props.posts.map((post, index) => {
					return <li key={index}><a href={`/posts/${post.fileName}`}>{post.attributes.title}</a> {post.attributes.emojis}</li>;
				})}
			</ul>
		</Default>
	);
};
