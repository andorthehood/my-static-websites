import type { AppProps } from 'next/app';
import '../style.scss';
import { NextPage } from 'next/types';
import { ReactElement, ReactNode } from 'react';
import { PostProps } from '../scripts/collectPosts';
import { useRouter } from 'next/router'
import Head from "next/head";

export type NextPageWithLayout<P = {}, IP = P> = NextPage<P, IP> & {
	getLayout?: (page: ReactElement, props: PostProps) => ReactNode
}

type AppPropsWithLayout = AppProps & {
	Component: NextPageWithLayout
}

function MyApp({ Component, pageProps }: AppPropsWithLayout) {
	const router = useRouter();
	const getLayout = Component.getLayout ?? ((page) => page)
	return getLayout(<><Head><link rel="canonical" href={"https://andor.cool" + router.asPath} /></Head><Component {...pageProps} /></>, pageProps);
}

export default MyApp;
