(function () {
    const elephant = document.getElementById("templeos-elephant");
    const elephantWidth = elephant.offsetWidth;
    const elephantHeight = elephant.offsetHeight;
    const speed = 0.75;
    let direction = -1;
    let x = Math.max(0, window.innerWidth - elephantWidth);
    let y = Math.max(0, window.innerHeight - elephantHeight);

    function updateElephantPosition() {
        const facing = direction === -1 ? 1 : -1;
        elephant.style.transform = `translate3d(${x}px, ${y}px, 0) scaleX(${facing})`;
    }

    function animate() {
        const maxX = Math.max(0, window.innerWidth - elephantWidth);

        if (maxX === 0) {
            x = 0;
            updateElephantPosition();
            requestAnimationFrame(animate);
            return;
        }

        x += speed * direction;

        if (x <= 0 || x >= maxX) {
            x = Math.max(0, Math.min(x, maxX));
            direction *= -1;
        }

        updateElephantPosition();
        requestAnimationFrame(animate);
    }

    window.addEventListener("resize", function () {
        const maxX = Math.max(0, window.innerWidth - elephantWidth);
        x = Math.min(x, maxX);
        y = Math.max(0, window.innerHeight - elephantHeight);
        updateElephantPosition();
    });

    updateElephantPosition();
    animate();
})();
