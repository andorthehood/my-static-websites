import type { NextPage } from 'next';
import { GetStaticProps, GetStaticPropsResult } from 'next';
import Head from 'next/head';
import { collectPosts } from '../scripts/collectPosts';

type PostProps = {
	slug: string;
	title: string;
	emojis?: string;
	date: string;
};

interface HomeProps {
	posts: PostProps[];
}

function groupPostsByYear(posts: PostProps[]) {
	return posts.reduce((previous, current) => {
		const year = new Date(current.date).getFullYear().toString();
		if (!previous[year]) {
			previous[year] = [];
		}
		previous[year].push(current) 
		return previous
	}, {} as Record<string, PostProps[]>);
}

const Home: NextPage<HomeProps> = ({ posts }) => {
	const groupsByYear = groupPostsByYear(posts);

	return (
		<div>
			<Head>
				<link rel="icon" href="/favicon.ico" />
			</Head>
			<ul className="postlist">
				{Object.keys(groupsByYear).reverse().map((year) => {
					return groupsByYear[year].map((post, index) => {
						return (
							<>
								{index === 0 && <span className="date">{year}</span>}
								<li key={index}>
									<a href={`/posts/${post.slug}`}>{post.title}</a> {post.emojis}
								</li>
							</>
						);
					})
				})}
			</ul>
		</div>
	);
};

export const getStaticProps: GetStaticProps = async function (): Promise<GetStaticPropsResult<HomeProps>> {
	const posts = await collectPosts('./_posts/');

	return {
		props: {
			posts: posts.map(post => ({
				title: post.attributes.title,
				slug: post.slug,
				emojis: post.attributes.emojis || '',
				date: post.attributes.date
			})),
		},
	};
};

export default Home;
