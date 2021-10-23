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
	fileName: string;
	timestamp: number;
}

export default async function parsePostFile(file: string, globalVariables = {}): Promise<PostProps> {
	const data = await fs.readFile(file, 'utf8');
	const { body, attributes } = fm(data) as { body: string; attributes: MarkdownAttributes };
	const liquidified = await engine.parseAndRender(body, { ...globalVariables, page: attributes });
	const content = marked(liquidified);
	return {
		content,
		attributes: attributes,
		timestamp: new Date(attributes.date).getTime(),
		fileName: path.basename(file, path.extname(file)),
	};
}
