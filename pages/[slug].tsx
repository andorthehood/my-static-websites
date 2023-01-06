import { GetStaticPaths, GetStaticProps, GetStaticPropsResult, NextPage } from 'next';
import { collectPosts, PostProps } from '../scripts/collectPosts';
import Head from 'next/head';
import { ReactElement } from 'react';
import DefaultLayout from '../components/layout/DefaultLayout';
import FlorDePielLayout from '../components/layout/FlorDePielLayout';
import { NextPageWithLayout } from './_app';

const Post: NextPageWithLayout<PostProps> = function Post(props) {
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

Post.getLayout = function getLayout(page: ReactElement, props: PostProps) {
	if (props.attributes.layout === 'flordepiel') {
		return <FlorDePielLayout>{page}</FlorDePielLayout>
	}

	return (
		<DefaultLayout>{page}</DefaultLayout>
	)
}

export default Post;
