(function () {
    const clippy = document.getElementById('clippy');
    const clippyGif = document.getElementById('clippy-gif');

    setInterval(function () {
        clippyGif.src = 'https://static.llllllllllll.com/andor/assets/clippy/swaying.gif?c=' + Date.now();
    }, 8000);

    setTimeout(function () {
        if (!document.body.classList.contains('collapsed')) {
            return;
        }

        const message = document.createElement('div');
        message.className = 'message';
        message.innerHTML = 'It looks like you\'re waiting for the DVD logo to hit the corner. I\'d love to see it too... If we\'re both patient, something just might happen soon...';
        clippy.appendChild(message);

        setTimeout(function () {
            message.remove();
        }, 1000 * 15);
    }, 1000 * 60);
})();