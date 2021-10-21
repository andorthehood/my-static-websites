import { promises as fs } from 'fs';
import path from 'path';
import fm from 'front-matter';
import { Liquid } from 'liquidjs';
import marked from 'marked';
import { HtmlProps } from 'next/dist/shared/lib/utils';

const engine = new Liquid({
	cache: true,
	root: ['./_includes/', './_layouts'],
	dynamicPartials: false,
});

interface PostAttributes {
    emojis?: string;
    title: string;
    date: string;
    categories?: string;
    location: string;
    layout?: string;
    tags?: string;
}

interface PostProps {
    content: string;
    attributes: PostAttributes;
    fileName: string;
}

export default async function parsePostFile(
	file: string,
	globalVariables = {}
): Promise<{ content: string; attributes: PostAttributes; fileName: string }> {
	const data = await fs.readFile(file, 'utf8');
	const { body, attributes } = fm(data);
	const liquidified = await engine.parseAndRender(body, { ...globalVariables, page: attributes });
	const content = marked(liquidified);
	return {
		content,
		attributes: attributes as PostAttributes,
		fileName: path.basename(file, path.extname(file)),
	};
}
