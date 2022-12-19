import Link from 'next/link';
import Head from 'next/head';
import { FC } from 'react';

const Layout: FC = function ({ children }) {
    return (
        <div className="container">
			<Head>
				<title>Andor Polgar&apos;s visual journal</title>
			</Head>
			<h1>
				<Link href="/">Andor Polgar&apos;s visual journal</Link>
			</h1>
			<div>
                {children}
			</div>
			<footer>
				<p>
					<Link href="https://andor.cool/aflordepiel">A Flor de Piel (2022) 🔞</Link> | Social media: <Link href="https://instagram.com/andorcover">@andorcover</Link>,{' '}
					<Link href="https://www.youtube.com/channel/UCF_EgDIkYFIeu-19KleLnFA">YouTube</Link>,{' '}
					<Link href="/links">Other links</Link> |{' '}
					My website doesn&apos;t collect your personal information, nor does it send any of it to
					third-party analytics services. 🍪 You don&apos;t have to believe me, you can see it for yourself in
					the source code on <Link href="https://github.com/andormade/andor.cool">github</Link>.
				</p>
			</footer>
		</div>
    )
  }

  export default Layout;