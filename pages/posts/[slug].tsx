import { GetStaticPaths, GetStaticProps, GetStaticPropsResult, NextPage } from 'next';
import { collectPosts, PostProps } from '../../scripts/collectPosts';
import Head from 'next/head';
import Link from 'next/link';
import { ReactElement } from 'react';
import DefaultLayout from '../../components/layout/DefaultLayout';
import { NextPageWithLayout } from '../_app';

const Post: NextPageWithLayout<PostProps> = function Post(props) {
	const exif = [
		'🎞️ ' + props.attributes.film,
		'🔎 ' + props.attributes.lens,
		'⚡ ' + props.attributes.flash,
		'📷 ' + props.attributes.camera,
		'🖨️ ' + props.attributes.scan
	].filter((element) => !element.includes('undefined'));

	return (
		<>
			<Head>
				<title>{props.attributes.title}</title>
			</Head>
			<div dangerouslySetInnerHTML={{ __html: props.content }}></div>
			{exif.length > 0 && <p>{exif.join(', ')}</p>}
			<p>
				{props.nextSafePost && props.nextSafePost.slug !== props.nextPost?.slug && (
					<>
						Next safe post:{' '}
						<Link href={'/posts/' + props.nextSafePost?.slug}>{props.nextSafePost?.attributes.title}</Link>{' '}
						{props.nextSafePost?.attributes?.emojis}
					</>
				)}
				{props.nextPost && (
					<>
						<br />
						Next post: <Link href={'/posts/' + props.nextPost?.slug}>{props.nextPost?.attributes.title}</Link>{' '}
						{props.nextPost?.attributes?.emojis}
					</>
				)}
			</p>
		</>
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

export const getStaticProps: GetStaticProps = async function (context): Promise<GetStaticPropsResult<PostProps>> {
	const posts = await collectPosts('./_posts/');
	const post = posts.find(({ slug }) => slug === context.params?.slug) || posts[0];
	return {
		props: post,
	};
};

Post.getLayout = function getLayout(page: ReactElement) {
	return (
	  <DefaultLayout>{page}</DefaultLayout>
	)
  }

export default Post;
