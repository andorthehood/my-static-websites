import Head from 'next/head';
import { FC } from 'react';

const Layout: FC = function ({ children }) {
  return (
      <div className="container">
        <Head>
          <title>A Flor de Piel</title>
        </Head>
        <style global jsx>{`
            html {
              background: linear-gradient(0deg, rgba(255,204,204,1) 0%, rgba(255,255,255,1) 100%);
              background-attachment: fixed;
            }

            h1 {
              font-size: 54px;
            }

            .laurel {
              background: url(https://static.llllllllllll.com/andor/assets/laurel.svg);
              width: 100px;
              height: 100px;
            }
          `}</style>
        <div>
          <h1>A Flor de Piel</h1>
          {children}
        </div>
      </div>
  )
}

export default Layout;