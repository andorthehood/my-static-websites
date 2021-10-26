import { promises as fs } from 'fs';
import path from 'path';
import fm from 'front-matter';
import { Liquid } from 'liquidjs';
import marked from 'marked';

const engine = new Liquid({
	cache: true,
	root: ['./_includes/', './_layouts'],
	dynamicPartials: false,
});

interface MarkdownAttributes {
	emojis?: string;
	title: string;
	date: string;
	categories?: string;
	location: string;
	layout?: string;
	tags?: string;
}

export interface PostProps {
	content: string;
	attributes: MarkdownAttributes;
	slug: string;
	timestamp: number;
	nextPost?: PostProps;
	previousPost?: PostProps;
}

async function parsePostFile(file: string, globalVariables = {}): Promise<PostProps> {
	const data = await fs.readFile(file, 'utf8');
	const { body, attributes } = fm(data) as { body: string; attributes: MarkdownAttributes };
	const liquidified = await engine.parseAndRender(body, { ...globalVariables, page: attributes });
	const content = marked(liquidified);
	return {
		content,
		attributes: attributes,
		timestamp: new Date(attributes.date).getTime(),
		slug: path.basename(file, path.extname(file)),
	};
}

async function collectPosts(): Promise<PostProps[]> {
	const postFiles = await fs.readdir('./_posts');
	const posts = await Promise.all(postFiles.map(postFile => parsePostFile('./_posts/' + postFile)));
	posts.sort((a, b) => (a.timestamp > b.timestamp ? -1 : a.timestamp < b.timestamp ? 1 : 0));
	return posts.map((post, index) => {
		return {
			...post,
			nextPost: posts[index + 1],
			previousPost: posts[index - 1],
		};
	});
}

(async function () {
	await fs.writeFile('posts.json', JSON.stringify(await collectPosts()));
})();
