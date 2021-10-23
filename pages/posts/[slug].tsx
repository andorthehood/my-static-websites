import { promises as fs } from 'fs';
import { GetStaticPaths, GetStaticProps, GetStaticPropsResult, NextPage } from 'next';
import path from 'path';
import parsePostFile, { PostProps } from '../../lib/parsePostFile';

const Post: NextPage<PostProps> = function Post(props) {
	return <div dangerouslySetInnerHTML={{ __html: props.content}}></div>;
};

export const getStaticPaths: GetStaticPaths = async function getStaticPaths() {
	const postFiles = await fs.readdir('./_posts');
	const paths = postFiles.map(file => ({ params: { slug: path.basename(file, '.md') } }));
	return {
		paths,
		fallback: false,
	};
};

export const getStaticProps: GetStaticProps = async function (context): Promise<GetStaticPropsResult<PostProps>> {
	const post = await parsePostFile('./_posts/' + context.params?.slug + '.md');
	return {
		props: post,
		revalidate: 1,
	};
};

export default Post;
