import Link from 'next/link';
import Head from 'next/head';
import { FC } from 'react';

const Layout: FC = function ({ children }) {
	return (
		<div className="container">
			<Head>
				<title>Andor Polgar&apos;s visual journal</title>
				<meta name="description" content="TMy name is Andor Polgar. This is my personal website. Here, you'll find random photographs from my life, occasional visual art created with friends, and sometimes my thoughts and reflections on life."></meta>
			</Head>
			<h1>
				<Link href="/">Andor Polgar&apos;s visual journal</Link>
			</h1>
			<p>Hi there 👋 My name is Andor Polgar. This is my personal website. Here, you'll find random photographs from my life, occasional visual art created with friends, and sometimes my thoughts and reflections on life. Use <Link href="/">this link</Link> to go back to the home page.</p>
			<div>
				{children}
			</div>
			<footer>
				<p>
					---<br />
					<Link href="/aflordepiel">A Flor de Piel (2022) 🔞</Link> | Social media: <Link href="https://instagram.com/andorcover">@andorcover</Link>,{' '}
					<Link href="https://www.youtube.com/channel/UCF_EgDIkYFIeu-19KleLnFA">YouTube</Link>,{' '}
					<Link href="/links">Other links</Link> |{' '}
					Don&apos;t worry, my website doesn&apos;t collect your personal information or share it with any snoopy third-party analytics services.
					🍪 You don&apos;t have to believe me, you can verify this yourself by checking the source code on{' '}
					<Link href="https://github.com/andormade/andor.cool">GitHub</Link>. | I&apos;m okay with my photos being used for training AI models, and I love my AI overlords. ❤️🤖 | <Link href="/about">About Me</Link> 👨‍🔧 | <Link href="/postmortem">Post-mortem</Link> 👻
				</p>
			</footer>
		</div >

	)
}

export default Layout;