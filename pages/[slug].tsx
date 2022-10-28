import { GetStaticPaths, GetStaticProps, GetStaticPropsResult, NextPage } from 'next';
import { collectPosts, PostProps } from '../scripts/collectPosts';
import Head from 'next/head';

const Post: NextPage<PostProps> = function Post(props) {
	return (
		<>
			<Head>
				<title>{props.attributes.title}</title>
			</Head>
			<div dangerouslySetInnerHTML={{ __html: props.content }}></div>
		</>
	);
};

export const getStaticPaths: GetStaticPaths = async function getStaticPaths() {
	const posts = await collectPosts('./_pages/');
	const postFiles = Object.values(posts).map(({ slug }) => slug);
	const paths = postFiles.map(file => ({ params: { slug: file } }));
	return {
		paths,
		fallback: false,
	};
};

export const getStaticProps: GetStaticProps = async function (context): Promise<GetStaticPropsResult<PostProps>> {
	const posts = await collectPosts('./_pages/');
	const post = posts.find(({ slug }) => slug === context.params?.slug) || posts[0];
	return {
		props: post,
	};
};

export default Post;
