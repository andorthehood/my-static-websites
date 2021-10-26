import { promises as fs } from 'fs';
import { GetStaticPaths, GetStaticProps, GetStaticPropsResult, NextPage } from 'next';
import path from 'path';
import { PostProps } from '../../scripts/collectPosts';
import Head from 'next/head';
import styled from 'styled-components';
import Link from 'next/link';
import posts from '../../posts.json';

const Button = styled(Link)`
	font-size: var(--font-size);
	color: var(--text-color);
	text-decoration: underline;
	&:hover {
		text-decoration: line-through;
	}
`;

const Post: NextPage<PostProps> = function Post(props) {
	return (
		<>
			<Head>
				<title>{props.attributes.title}</title>
			</Head>
			<div dangerouslySetInnerHTML={{ __html: props.content }}></div>
			{props.nextPost && (
				<p>
					Next post: <Button href={'/posts/' + props.nextPost?.fileName}>{props.nextPost?.attributes.title}</Button>{' '}
					{props.nextPost?.attributes?.emojis}
				</p>
			)}
		</>
	);
};

export const getStaticPaths: GetStaticPaths = async function getStaticPaths() {
	const postFiles = Object.values(posts).map(({ fileName }) => fileName);
	const paths = postFiles.map(file => ({ params: { slug: file } }));
	return {
		paths,
		fallback: false,
	};
};

export const getStaticProps: GetStaticProps = async function (context): Promise<GetStaticPropsResult<PostProps>> {
	const post = posts.find(({ fileName }) => fileName === context.params?.slug) || posts[0];
	return {
		props: post,
		revalidate: 1,
	};
};

export default Post;
