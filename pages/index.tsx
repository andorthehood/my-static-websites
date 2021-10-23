import type { NextPage } from 'next';
import { promises as fs } from 'fs';
import { GetStaticProps, GetStaticPropsResult } from 'next';
import Head from 'next/head';
import parsePostFile, { PostProps } from '../lib/parsePostFile';

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
	const postFiles = await fs.readdir('./_posts');
	const posts = await Promise.all(postFiles.map(postFile => parsePostFile('./_posts/' + postFile)));
	posts.sort((a, b) => (a.timestamp > b.timestamp ? -1 : a.timestamp < b.timestamp ? 1 : 0));
	return {
		props: { posts },
		revalidate: 1,
	};
};

export default Home;
