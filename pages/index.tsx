import type { NextPage } from 'next';
import { GetStaticProps, GetStaticPropsResult } from 'next';
import Head from 'next/head';
import posts from '../posts.json';
import { PostProps } from '../scripts/collectPosts';

interface HomeProps {
	posts: PostProps[];
}

const Home: NextPage<HomeProps> = ({ posts }) => {
	return (
		<div>
			<Head>
				<title></title>
				<meta name="description" content="" />
				<link rel="icon" href="/favicon.ico" />
			</Head>
			<ul>
				{posts.map((post, index) => {
					return (
						<li key={index}>
							<a href={`/posts/${post.fileName}`}>{post.attributes.title}</a> {post.attributes.emojis}
						</li>
					);
				})}
			</ul>
		</div>
	);
};

export const getStaticProps: GetStaticProps = async function (context): Promise<GetStaticPropsResult<HomeProps>> {
	return {
		props: { posts },
		revalidate: 1,
	};
};

export default Home;
