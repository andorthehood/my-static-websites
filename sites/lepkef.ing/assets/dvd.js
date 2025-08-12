(function () {
    const dvdLogo = document.getElementById('dvd-logo');
    let x = 50;
    let y = 50;
    let dx = 2;
    let dy = 1.5;
    let frameCount = 0;
    const TARGET_FRAMES = 3600; // 1 minute at 60fps

    function traceBackFromBottomLeftCorner(targetFrames) {
        const rect = dvdLogo.getBoundingClientRect();
        const maxX = window.innerWidth - rect.width;
        const maxY = window.innerHeight - rect.height;

        // Bottom-left corner coordinates
        const corner = { x: 0, y: maxY };

        // For bottom-left corner, the approach velocity must be negative X, positive Y
        // (coming from top-right direction)
        const approachVx = -Math.abs(dx);
        const approachVy = Math.abs(dy);

        const result = simulateBackwards(corner.x, corner.y, approachVx, approachVy, targetFrames, maxX, maxY);
        if (result) {
            return {
                x: result.startX,
                y: result.startY,
                dx: result.startDx,
                dy: result.startDy,
                frames: targetFrames
            };
        }

        return null;
    }

    function simulateBackwards(cornerX, cornerY, approachVx, approachVy, targetFrames, maxX, maxY) {
        // Start from the corner and work backwards
        let x = cornerX;
        let y = cornerY;
        // The velocity we use for backwards simulation should be the APPROACH velocity
        // because we want to trace the path that LED TO this corner
        let vx = approachVx;
        let vy = approachVy;

        for (let frame = 0; frame < targetFrames; frame++) {
            // Move backwards one frame (reverse of x += vx, y += vy)
            x -= vx;
            y -= vy;

            // Handle wall bounces exactly like the forward simulation but in reverse
            if (x <= 0 || x >= maxX) {
                vx = -vx;
                x = Math.max(0, Math.min(x, maxX));
            }

            if (y <= 0 || y >= maxY) {
                vy = -vy;
                y = Math.max(0, Math.min(y, maxY));
            }
        }

        // Verify the position is valid
        if (x >= 0 && x <= maxX && y >= 0 && y <= maxY) {
            return {
                startX: x,
                startY: y,
                startDx: vx, // The velocity at the starting position (same as what we ended with)
                startDy: vy
            };
        }

        return null;
    }

    function animate() {
        const rect = dvdLogo.getBoundingClientRect();
        const maxX = window.innerWidth - rect.width;
        const maxY = window.innerHeight - rect.height;

        x += dx;
        y += dy;
        frameCount++;

        // Check for corner collision BEFORE applying bounces
        const isAtCorner = (x <= 0 || x >= maxX) && (y <= 0 || y >= maxY);

        if (isAtCorner) {
            console.log(`🎉 CORNER HIT! Frame ${frameCount} at position (${x.toFixed(1)}, ${y.toFixed(1)})`);
        }

        if (x <= 0 || x >= maxX) {
            dx = -dx;
            x = Math.max(0, Math.min(x, maxX));
        }

        if (y <= 0 || y >= maxY) {
            dy = -dy;
            y = Math.max(0, Math.min(y, maxY));
        }

        dvdLogo.style.left = x + 'px';
        dvdLogo.style.top = y + 'px';

        requestAnimationFrame(animate);
    }

    // Find and set starting position for 1-minute bottom-left corner hit
    const optimalStart = traceBackFromBottomLeftCorner(TARGET_FRAMES);
    if (optimalStart) {
        x = optimalStart.x;
        y = optimalStart.y;
        dx = optimalStart.dx;
        dy = optimalStart.dy;
        frameCount = 0; // Reset frame counter

        // Update logo position immediately
        dvdLogo.style.left = x + 'px';
        dvdLogo.style.top = y + 'px';
    }


    animate();

    window.addEventListener('resize', function () {
        const rect = dvdLogo.getBoundingClientRect();
        const maxX = window.innerWidth - rect.width;
        const maxY = window.innerHeight - rect.height;
        x = Math.min(x, maxX);
        y = Math.min(y, maxY);
    });
})();