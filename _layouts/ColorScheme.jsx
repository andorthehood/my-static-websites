import { createGlobalStyle } from 'styled-components';

export default createGlobalStyle`
    :root {
        --background-color: #ffffff;
        --font-size: 14px;
        --text-color: #999999;
        --title-color: #cccccc;

        // @media (prefers-color-scheme: dark) {
        //     --background-color: #111111;
        //     --text-color: #999999;
        //     --title-color: #666666;
        // }
    }
`;
