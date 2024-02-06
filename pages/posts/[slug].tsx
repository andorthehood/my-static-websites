import { GetStaticPaths, GetStaticProps, GetStaticPropsResult, NextPage } from 'next';
import { collectPosts, PostProps } from '../../scripts/collectPosts';
import Head from 'next/head';
import Link from 'next/link';
import { ReactElement } from 'react';
import DefaultLayout from '../../components/layout/DefaultLayout';
import { NextPageWithLayout } from '../_app';
import { Fragment } from 'react';

interface PostPageProps {
	post: PostProps;
}

const Post: NextPageWithLayout<PostPageProps> = function Post({ post }) {
	const exif = [
		'🎞️ film: ' + post.attributes.film,
		'🔎 lens: ' + post.attributes.lens,
		'⚡ flash: ' + post.attributes.flash,
		'📷 camera: ' + post.attributes.camera,
		'🖨️ ' + post.attributes.scan
	].filter((element) => !element.includes('undefined'));

	return (
		<div className="post-page">
			<Head>
				<title>{post.attributes.title}</title>
			</Head>
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
			post
		},
	};
};

Post.getLayout = function getLayout(page: ReactElement) {
	return (
		<DefaultLayout>{page}</DefaultLayout>
	)
}

export default Post;
