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