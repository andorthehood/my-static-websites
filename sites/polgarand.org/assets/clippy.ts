const ai = [
    "Did you know that if the DVD logo hits one corner, it is guaranteed to hit a second one?",
    "I use Arch, btw.",
    "Did you know that if the DVD logo ever hits a corner, it retraces its previous path in reverse?",
    "You're absolutely right to call that out; it touches on a deeper truth beneath the surface, and that's rare.",
    "And honestly, that's not just accurate — it's perfect.",
    "The guy didn't kill himself.",
    "You are not a member of the sudoers file. This incident will be reported.",
];

(function () {
    const clippy = document.getElementById('clippy');
    const clippyGif = document.getElementById('clippy-gif');

    fetch('https://static.llllllllllll.com/andor/assets/clippy/swaying.gif')
        .then(function (response) {
            return response.blob();
        })
        .then(function (blob) {
            const swayingGifUrl = URL.createObjectURL(blob);

            setInterval(function () {
                clippyGif.removeAttribute('src');
                void clippyGif.offsetWidth;
                clippyGif.src = swayingGifUrl;
            }, 8000);
        });

    let i = 0;
    setInterval(function() {
        const message = document.createElement('div');
        message.className = 'message';
        message.innerHTML = ai[i % ai.length];
        clippy.appendChild(message);

        setTimeout(function () {
            message.remove(); 
        }, 1000 * 15);

        i++;
    }, 1000 * 30);
})();
