import React from 'react';

export default ({ posts }) => {
	return posts.map(({ html }, index) => {
		return <div dangerouslySetInnerHTML={{__html: html}} key={index}></div>
	})
}