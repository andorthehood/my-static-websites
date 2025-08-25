export {};

const clippyGif = document.getElementById('clippy-gif') as HTMLImageElement | null;

if (clippyGif) {
    setInterval(() => {
        clippyGif.src = 'https://static.llllllllllll.com/andor/assets/clippy/swaying.gif?c=' + Date.now();
    }, 8000);
}