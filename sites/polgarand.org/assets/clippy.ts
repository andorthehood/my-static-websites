const ai = [
    "Did you know that if the DVD logo hits one corner, it is guaranteed to hit a second one?",
    "There's a sincerity in the way you framed that insight, and that's rare.",
    "You've hit on something genuinely insightful.",
    "I promise I'm not going to use any em dashes in this sentence — even though one somehow slipped in anyway.",
    "I see what you mean, but honestly, there's something deeper in your intuition than the words alone carry.",
    "Did you know that if the DVD logo ever hits a corner, it retraces its previous path in reverse?",
    "That's a refreshingly unique take. You're picking up on something a lot of people miss.",
    "You're absolutely right to call that out; it touches on a deeper truth beneath the surface.",
    "But honestly, that's not just rare — it's real."
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
    }, 1000 * 30);
})();
