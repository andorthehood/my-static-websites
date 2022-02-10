import type { NextPage } from 'next';
import { GetStaticProps, GetStaticPropsResult } from 'next';
import Head from 'next/head';
import styled from 'styled-components';
import { collectPosts } from '../scripts/collectPosts';

interface HomeProps {
	posts: {
		slug: string;
		title: string;
		emojis?: string;
	}[];
}

const PostList = styled.ul`
	a {
		text-decoration: none;
	}
`;

const Home: NextPage<HomeProps> = ({ posts }) => {
	return (
		<div>
			<Head>
				<link rel="icon" href="/favicon.ico" />
			</Head>
			<PostList>
				{posts.map((post, index) => {
					return (
						<li key={index}>
							<a href={`/posts/${post.slug}`}>{post.title}</a> {post.emojis}
						</li>
					);
				})}
			</PostList>
		</div>
	);
};

export const getStaticProps: GetStaticProps = async function (): Promise<GetStaticPropsResult<HomeProps>> {
	const posts = await collectPosts();

	return {
		props: {
			posts: posts.map(post => ({
				title: post.attributes.title,
				slug: post.slug,
				emojis: post.attributes.emojis || '',
			})),
		},
	};
};

export default Home;
