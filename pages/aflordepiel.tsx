import { NextPage } from 'next';
import Head from 'next/head';

const Post: NextPage = function Post() {
	return (
		<>
			<Head>
				<title>A Flor de Piel</title>
			</Head>
			<div className="video-container video-container-3-2">
                <iframe title="vimeo-player" src="https://player.vimeo.com/video/765250781?h=c2372131b3" width="100%" height="100%" frameBorder="0" allowFullScreen></iframe>
            </div>
		</>
	);
};

export default Post;