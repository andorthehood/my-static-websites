import { NextPage } from 'next';
import Head from 'next/head';

const Post: NextPage = function Post() {
	return (
		<div className="container">
			<Head>
				<title>A Flor de Piel</title>
			</Head>
			<p>A Flor de Piel is an avant-garde post-porn film shot entirely on an analog 35mm rangefinder camera.</p>
			<p>The title is a Spanish idiomatic expression that literally translates to “like the flower of skin”, the phrase conveys an overt display of emotion.</p>
			<div style={{ padding: '18px 0'}}>
				<div className="video-container video-container-3-2">
                	<iframe title="vimeo-player" src="https://player.vimeo.com/video/765250781?h=c2372131b3" width="100%" height="100%" frameBorder="0" allowFullScreen></iframe>
            	</div>
			</div>
			<p>Directed and produced by Andor Polgar; Cast: Sick Ducks, Ju GomA; Music and mixing: Andrew Visser (Oneven)</p>
			<p>Some technical details: shot on 30 rolls of Ilford HP5 Plus film, developed in Ilford ID-11. Lenses used: Zeiss ZM 50mm F/2.0 Planar T* and Zeiss ZM Zeiss 35mm F/2.0 Biogon T*, Camera: Leica M6 TTL (1984), the frames were re-photographed from negative using a Nikon D780 DSLR camera with a Sigma 105mm f/2.8 EX DG OS HSM macro lens. The individual frames where edited in Lightroom, and then stitched together with FFMPEG.</p>
		</div>
	);
};

export default Post;