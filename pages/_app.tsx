import type { AppProps } from 'next/app';
import '../style.scss';
import { NextPage } from 'next/types';
import { ReactElement, ReactNode } from 'react';
import { PostProps } from '../scripts/collectPosts';

export type NextPageWithLayout<P = {}, IP = P> = NextPage<P, IP> & {
	getLayout?: (page: ReactElement, props: PostProps) => ReactNode
}

type AppPropsWithLayout = AppProps & {
	Component: NextPageWithLayout
}

function MyApp({ Component, pageProps }: AppPropsWithLayout) {
	const getLayout = Component.getLayout ?? ((page) => page)
	return getLayout(<Component {...pageProps} />, pageProps);
}

export default MyApp;
