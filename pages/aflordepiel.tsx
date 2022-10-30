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
			<p>It is strongly recommended to watch it on a 4K high-definition TV.</p>
			<p>Directed and produced by Andor Polgar; Cast: Sick Ducks, Ju GomA; Music and mixing: Andrew Visser (Oneven)</p>
			<p>Some technical details: the entire motion picture was shot on 30 rolls of Ilford HP5 Plus film, they were developed by hand in Ilford ID-11. Most rolls were push processed by 1 or 2 exposure values.</p>
			<p>Lenses used: Zeiss ZM 50mm F/2.0 Planar T* and Zeiss ZM Zeiss 35mm F/2.0 Biogon T*, Camera: Leica M6 TTL (1984).</p>
			<p>The frames were digitized by re-photographing them from the negatives using a Nikon D780 DSLR camera with a Sigma 105mm f/2.8 EX DG OS HSM macro lens.</p>
			<p>The individual frames where edited in Lightroom, and then stitched together with FFmpeg. The excessive amount of dust and dirt was transferred to the film during the digitization process.</p>
		</div>
	);
};

export default Post;