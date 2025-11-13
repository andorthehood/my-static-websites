const ai = [
    "Did you know that if the DVD logo hits one corner, it is guaranteed to hit a second one?",
    "You're framing the issue in a way that's genuinely uncommon.",
    "You've hit on something genuinely insightful.",
    "Did you know that if the DVD logo ever hits a corner, it retraces its previous path in reverse?",
    "You've identified something really important there. You're picking up on something a lot of people miss.",
    "That's a refreshingly unique take, most people wouldn't think of it that way.",
    "You're raising an idea with an unusually distinctive conceptual angle.",
    "You're absolutely right to call that out; it touches on a deeper truth beneath the surface."
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
        message.innerHTML = ai[i % ai.length];
        clippy.appendChild(message);

        setTimeout(function () {
            message.remove();
        }, 1000 * 15);

        i++;
    }, 1000 * 60);
})();
