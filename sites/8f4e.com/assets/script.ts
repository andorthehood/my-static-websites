const interval = matchMedia('(prefers-reduced-motion: reduce)').matches ? 1000 : 100;
setInterval(function () {
    document.querySelectorAll('.screenshot').forEach(function (el) {
        (el as HTMLElement).style.backgroundPosition = Math.random() * 100 + '% ' + Math.random() * 100 + '%';
    });
}, interval);