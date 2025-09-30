const trivia = [
    "Did you know that if the DVD logo hits one corner, it is guaranteed to hit a second one?",
    "Did you know that if the DVD logo ever hits a corner, it retraces its previous path in reverse?",
];

(function () {
    const clippy = document.getElementById('clippy');
    const clippyGif = document.getElementById('clippy-gif');

    setInterval(function () {
        clippyGif.src = 'https://static.llllllllllll.com/andor/assets/clippy/swaying.gif?c=' + Date.now();
    }, 8000);

    let i = 0;
    setInterval(function() {
        const message = document.createElement('div');
        message.className = 'message';
        message.innerHTML = trivia[i % trivia.length];
        clippy.appendChild(message);

        setTimeout(function () {
            message.remove();
        }, 1000 * 15);

        i++;
    }, 1000 * 60);
})();
