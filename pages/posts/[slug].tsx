import { GetStaticPaths, GetStaticProps, GetStaticPropsResult, NextPage } from 'next';
import { collectPosts, PostProps } from '../../scripts/collectPosts';
import Head from 'next/head';
import Link from 'next/link';
import { ReactElement } from 'react';
import DefaultLayout from '../../components/layout/DefaultLayout';
import { NextPageWithLayout } from '../_app';
import { Fragment } from 'react';

type PublicPostProps = {
	slug: string;
	title: string;
	emojis?: string;
	date: string;
};

interface PostPageProps {
	post: PostProps;
	posts: PublicPostProps[];
}

function groupPostsByYear(posts: PublicPostProps[]) {
	return posts.reduce((previous, current) => {
		const year = new Date(current.date).getFullYear().toString();
		if (!previous[year]) {
			previous[year] = [];
		}
		previous[year].push(current)
		return previous
	}, {} as Record<string, PublicPostProps[]>);
}

const Post: NextPageWithLayout<PostPageProps> = function Post({ post, posts }) {
	const exif = [
		'🎞️ film: ' + post.attributes.film,
		'🔎 lens: ' + post.attributes.lens,
		'⚡ flash: ' + post.attributes.flash,
		'📷 camera: ' + post.attributes.camera,
		'🖨️ ' + post.attributes.scan
	].filter((element) => !element.includes('undefined'));

	const groupsByYear = groupPostsByYear(posts);

	return (
		<div className="post-page">
			<Head>
				<title>{post.attributes.title}</title>
			</Head>
			<ul className="postlist">
				{Object.keys(groupsByYear).reverse().map((year) => {
					return groupsByYear[year].map((post, index) => {
						return (
							<Fragment key={index}>
								{index === 0 && <span className="date">{year}</span>}
								<li>
									<a href={`/posts/${post.slug}`}>{post.title}</a>
								</li>
							</Fragment>
						);
					})
				})}
			</ul>
			<div dangerouslySetInnerHTML={{ __html: post.content }}></div>
			{exif.length > 0 && <p>{exif.join(', ')}</p>}
			{post.attributes.people && <p>The people captured in the photos are: {post.attributes.people?.map((name) => {
				return <Fragment key={name}><a href={"https://instagram.com/" + name.substring(1)} target="_blank" rel="noreferrer noopener nofollow">{name}</a>{', '}</Fragment>;
			})}</p>}
			<p>
				{post.nextSafePost && post.nextSafePost.slug !== post.nextPost?.slug && (
					<>
						Next safe post:{' '}
						<Link href={'/posts/' + post.nextSafePost?.slug}>{post.nextSafePost?.attributes.title}</Link>{' '}
						{post.nextSafePost?.attributes?.emojis}
					</>
				)}
				{post.nextPost && (
					<>
						<br />
						Next post: <Link href={'/posts/' + post.nextPost?.slug}>{post.nextPost?.attributes.title}</Link>{' '}
						{post.nextPost?.attributes?.emojis}
					</>
				)}
			</p>
		</div>
	);
};

export const getStaticPaths: GetStaticPaths = async function getStaticPaths() {
	const posts = await collectPosts('./_posts/');
	const postFiles = Object.values(posts).map(({ slug }) => slug);
	const paths = postFiles.map(file => ({ params: { slug: file } }));
	return {
		paths,
		fallback: false,
	};
};

export const getStaticProps: GetStaticProps = async function (context): Promise<GetStaticPropsResult<PostPageProps>> {
	const posts = await collectPosts('./_posts/');
	const post = posts.find(({ slug }) => slug === context.params?.slug) || posts[0];
	return {
		props: {
			post, posts: posts.map(post => ({
				title: post.attributes.title,
				slug: post.slug,
				emojis: post.attributes.emojis || '',
				date: post.attributes.date
			}))
		},
	};
};

Post.getLayout = function getLayout(page: ReactElement) {
	return (
		<DefaultLayout>{page}</DefaultLayout>
	)
}

export default Post;
