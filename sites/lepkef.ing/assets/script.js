document.getElementById('button-collapse').addEventListener('click', function (event) {
    document.body.classList.toggle('collapsed');
});
document.getElementById('button-close').addEventListener('click', function () {
    document.querySelectorAll('.window').forEach((window) => window.remove());
});
document.getElementById('button-back').addEventListener('click', function () {
    window.history.back();
});
document.getElementById('button-forward').addEventListener('click', function () {
    window.history.forward();
});
document.getElementById('button-reload').addEventListener('click', function () {
    window.location.reload();
});
document.getElementById('button-home').addEventListener('click', function () {
    window.location.href = '/';
});
document.getElementById('button-print').addEventListener('click', function () {
    window.location.href = '/prints';
});

(function() {
    const dvdLogo = document.getElementById('dvd-logo');
    let x = 50;
    let y = 50;
    let dx = 2;
    let dy = 1.5;
    
    function animate() {
        const rect = dvdLogo.getBoundingClientRect();
        const maxX = window.innerWidth - rect.width;
        const maxY = window.innerHeight - rect.height;
        
        x += dx;
        y += dy;
        
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
    
    animate();
    
    window.addEventListener('resize', function() {
        const rect = dvdLogo.getBoundingClientRect();
        const maxX = window.innerWidth - rect.width;
        const maxY = window.innerHeight - rect.height;
        x = Math.min(x, maxX);
        y = Math.min(y, maxY);
    });
})();

(function() {
    const clippyGif = document.getElementById('clippy-gif');

    setInterval(function() {
        clippyGif.src = 'https://static.llllllllllll.com/andor/assets/clippy/swaying.gif?c=' + Date.now();
    }, 8000);
})();