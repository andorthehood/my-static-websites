import type { NextPage } from 'next';
import { GetStaticProps, GetStaticPropsResult } from 'next';
import Head from 'next/head';
import posts from '../posts.json';

interface HomeProps {
	posts: {
		slug: string;
		title: string;
		emojis?: string;
	}[];
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
							<a href={`/posts/${post.slug}`}>{post.title}</a> {post.emojis}
						</li>
					);
				})}
			</ul>
		</div>
	);
};

export const getStaticProps: GetStaticProps = async function (): Promise<GetStaticPropsResult<HomeProps>> {
	return {
		props: {
			posts: posts.map(post => ({
				title: post.attributes.title,
				slug: post.slug,
				emojis: post.attributes.emojis,
			})),
		},
		revalidate: 1,
	};
};

export default Home;
