import { GetStaticProps, GetStaticPropsResult } from 'next';
import { NextPageWithLayout } from './_app';
import fs from "fs";
import Image from 'next/image';
import { useCallback, useEffect, useState } from 'react';

interface GridProps {
	images: string[];
}

const Grid: NextPageWithLayout<GridProps> = function Grid(props) {

	const [backgroundColor, setBackgroundColor] = useState('#ffffff');

	const onClick = useCallback(() => {
		setBackgroundColor(backgroundColor === '#000000' ? '#ffffff' : '#000000')
	}, [backgroundColor]);

	useEffect(() => {
		document.body.style.backgroundColor = backgroundColor;
	}, [backgroundColor])

	return (
		<>
			<style jsx global>{`
				#container {
					width: 50%;
					margin: 0 auto;
					display: grid;
					grid-template-columns: auto auto auto;
					gap: 1px;
				}

        		.cell {
					aspect-ratio: 1;
					position: relative;
				}
			`}</style>
			<div id="container" onClick={onClick}>
				<div className="cell"><Image src="https://static.llllllllllll.com/andor/images/20210911_P1019333_frida_original.jpg" alt="" fill style={{ objectFit: 'cover'}} /></div>
				{props.images.map((src) => <div className="cell" key={src}><Image src={src} alt="" fill /></div>)}
			</div>
		</>
	);
};

export const getStaticProps: GetStaticProps = async function (): Promise<GetStaticPropsResult<GridProps>> {
	if (!fs.existsSync('./public/grid')) {
		return {
			props: { images: [] }
		}
	}
	
	const images = fs.readdirSync('./public/grid').map((file) => '/grid/' + file).reverse();

	return {
		props: { images },
	};
};

export default Grid;
