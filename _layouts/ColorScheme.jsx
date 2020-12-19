import { createGlobalStyle } from 'styled-components';

export default createGlobalStyle`
    :root {
        --title-color: #cccccc;
        --background-color: #ffffff;
        --text-color: #999999;

        @media (prefers-color-scheme: dark) {
            --title-color: #666666;
            --background-color: #000000;
            --text-color: #999999;
        }
    }
`;
