import type { AppProps } from 'next/app';
import Link from 'next/link';
import Head from 'next/head';
import '../style.scss';

function MyApp({ Component, pageProps }: AppProps) {
	return (
		<div className="container">
			<Head>
				<title>Andor Polgar&apos;s visual journal</title>
				<meta
					name="viewport"
					content="width=device-width, initial-scale=1, maximum-scale=1, minimum-scale=1, user-scalable=no"
				/>
			</Head>
			<h1>
				<Link href="/">Andor Polgar&apos;s visual journal</Link>
			</h1>
			<div>
				<Component {...pageProps} />
			</div>
			<footer>
				<p>
					Instagram: <Link href="https://instagram.com/andorcover">@andorcover</Link> |{' '}
					<Link href="https://www.youtube.com/channel/UCF_EgDIkYFIeu-19KleLnFA">YouTube</Link> | {' '}
					<Link href="/links">Other links</Link> |{' '}
					My website doesn&apos;t collect your personal information, nor does it send any of it to
					third-party analytics services. 🍪 You don&apos;t have to believe me, you can see it for yourself in
					the source code on <Link href="https://github.com/andormade/andor.cool">github</Link>.
				</p>
			</footer>
		</div>
	);
}
export default MyApp;
