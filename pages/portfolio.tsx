import { NextPage } from 'next';
import Head from 'next/head';

const baseUrl = "https://static.llllllllllll.com/andor/";

const images = [
	'20211211_24_marcelo_portra800_m6_8200i.jpg',
	'20220115_18_julia_leo_portra800_m6_8200i.jpg',
	'20211101_43_mad_rollei32_umax400_8200i.jpg',
	'20211104_44_peyton_m6_portra800_8200i.jpg',
	'20200526_6_texel_portra800_rollei.jpg',
	'20220130_16_femke_portra800_m6_8200i.jpg',
	'20220522_148_montville_portra400_ma.jpg',
	'20201202_shibari_6_m6_portra800.jpg',
	'20210919_22_david_m8_portra800_8200i.jpg',
	'20211104_19_peyton_m6_portra800_8200i.jpg',
	'2019_2_summer_portra_160_contaxg2_zp45f2.jpg',
	'20210807_1_goma_m8_portra800_8200i.jpg',
	'20220308_DSC_7269_elyisan_julia_terp.jpg',
	'20210801_11_noa_m8_portra800_8200i.jpg',
	'20220526_41_manya_portra800_ma.jpg',
	'20210911_P1019333_frida.jpg',
	'20220522_93_montville_portra800_ma.jpg',
	'20220430_29_emma_portra800_ma.jpg',
	'20210919_35_david_m8_portra800_8200i.jpg'
];

const Portfolio: NextPage = function Portfolio() {
	return (
		<>
			<Head>
				<title>Portfolio</title>
			</Head>
			<div>
				{images.map((imageFile) => 
					<p><img key={imageFile} src={`${baseUrl}${imageFile}`} alt="" width={"100%"} height="100%" /></p>
				)}
            </div>
		</>
	);
};

export default Portfolio;
