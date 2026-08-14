(function () {
    const elephant = document.getElementById("templeos-elephant");
    const elephantWidth = elephant.offsetWidth;
    const elephantHeight = elephant.offsetHeight;
    const speed = 0.75;
    const bounceHeight = 60;
    const gravity = 0.1;
    let direction = -1;
    let x = Math.max(0, window.innerWidth - elephantWidth);
    let y = Math.max(0, window.innerHeight - elephantHeight);
    let verticalVelocity = getBounceVelocity(y);

    function getBounceVelocity(floor) {
        const availableBounceHeight = Math.min(bounceHeight, floor);
        return -Math.sqrt(2 * gravity * availableBounceHeight);
    }

    function updateElephantPosition() {
        const facing = direction === -1 ? 1 : -1;
        elephant.style.transform = `translate3d(${x}px, ${y}px, 0) scaleX(${facing})`;
    }

    function animate() {
        const maxX = Math.max(0, window.innerWidth - elephantWidth);
        const floor = Math.max(0, window.innerHeight - elephantHeight);

        if (maxX === 0) {
            x = 0;
        } else {
            x += speed * direction;

            if (x <= 0 || x >= maxX) {
                x = Math.max(0, Math.min(x, maxX));
                direction *= -1;
            }
        }

        if (floor === 0) {
            y = 0;
            verticalVelocity = 0;
        } else {
            verticalVelocity += gravity;
            y += verticalVelocity;

            if (y >= floor) {
                y = floor;
                verticalVelocity = getBounceVelocity(floor);
            }
        }

        updateElephantPosition();
        requestAnimationFrame(animate);
    }

    window.addEventListener("resize", function () {
        const maxX = Math.max(0, window.innerWidth - elephantWidth);
        const floor = Math.max(0, window.innerHeight - elephantHeight);
        x = Math.min(x, maxX);
        y = Math.max(0, Math.min(y, floor));
        updateElephantPosition();
    });

    updateElephantPosition();
    animate();
})();
