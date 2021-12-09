import { Feed } from 'feed';
import { promises as fs } from 'fs';
import { collectPosts } from './collectPosts';

(async function () {
	const feed = new Feed({
		title: "Andor Polgar's photo journal",
		description: "Andor Polgar's photo journal",
		id: 'https://andor.cool/',
		link: 'https://andor.cool/',
		language: 'en',
		copyright: 'All rights reserved 2021, Andor Polgar',
		updated: new Date(),
		feedLinks: {
			atom: 'https://andor.cool/atom.xml',
			rss2: 'https://anfor.cool/feed.xml.',
		},
		author: {
			name: 'Andor Polgar',
		},
	});

	const posts = await collectPosts();

	posts.forEach(post => {
		feed.addItem({
			title: post.attributes.title,
			id: 'https://andor.cool/posts/' + post.slug,
			link: 'https://andor.cool/posts/' + post.slug,
			description: post.attributes.title,
			content: post.content,
			author: [
				{
					name: 'Andor Polgar',
				},
			],
			date: new Date(post.attributes.date),
			image: post.images[0] || '',
		});
	});

	await fs.mkdir('./public', { recursive: true });
	await fs.writeFile('./public/feed.xml', feed.rss2());
	await fs.writeFile('./public/atom.xml', feed.atom1());
})();
