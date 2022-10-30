import Head from 'next/head';
import { FC } from 'react';

const Layout: FC = function ({ children }) {
    return (
        <div className="container">
			<Head>
				<title>Andor Polgar&apos;s visual journal</title>
			</Head>
			<div>
                {children}
			</div>
		</div>
    )
  }

  export default Layout;