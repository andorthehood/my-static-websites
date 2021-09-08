import { createGlobalStyle } from 'styled-components';

export default createGlobalStyle`
    :root {
        --background-color: #fff0ee;
        --font-size: 18px;
        --text-color: #333399;
        --title-color: #333399;

        // @media (prefers-color-scheme: dark) {
        //     --background-color: #111111;
        //     --text-color: #999999;
        //     --title-color: #666666;
        // }
    }
`;
