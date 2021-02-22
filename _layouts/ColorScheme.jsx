import { createGlobalStyle } from 'styled-components';

export default createGlobalStyle`
    :root {
        --font-size: 14px;

        --title-color: #cccccc;
        --background-color: #ffffff;
        --text-color: #999999;

        @media (prefers-color-scheme: dark) {
            --title-color: #666666;
            --background-color: #111111;
            --text-color: #666666;
        }
    }
`;
