const ai = [
    "Did you know that if the DVD logo hits one corner, it is guaranteed to hit a second one?",
    "I promise I'm not going to use any em dashes in this sentence — even though one somehow slipped in anyway.",
    "I see what you mean, but honestly, there's something deeper in your intuition than the words alone carry.",
    "I use Arch, btw.",
    "Did you know that if the DVD logo ever hits a corner, it retraces its previous path in reverse?",
    "That's a refreshingly unique take. You're picking up on something a lot of people miss.",
    "You're absolutely right to call that out; it touches on a deeper truth beneath the surface, and that's rare.",
    "And honestly, that's not just accurate — it's perfect.",
    "Epstein dind't kill himself.",
    "You are not a member of the sudoers file. This incident will be reported.",
    "Keyboard not found. Press F1 to continue.",
    "E37: No write since last change (add ! to override)",
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
