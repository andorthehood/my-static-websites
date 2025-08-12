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

(function () {
	const links = document.querySelectorAll('a');
	const content = document.querySelector('.content');
	const head = document.querySelector('head');

	window.addEventListener('popstate', function (event) {
		const json = location.pathname === '/' ? '/index.json' : this.location.pathname + '.json';
		fetch(location.origin + json)
			.then(response => response.json())
			.then(data => {
				content.innerHTML = data.content;
			});
	});

	links.forEach(link => {
		link.addEventListener('click', function (event) {
			// If the link is external, don't do anything.
			if (link.href.startsWith('http')) {
				return;
			}

			event.preventDefault();
			fetch(link.href + '.json')
				.then(response => response.json())
				.then(data => {
					window.history.pushState({}, '', link.href);
					if (data.css) {
						const style = document.createElement('link');
						style.rel = 'stylesheet';
						style.href = '/assets/' + data.css;
						head.appendChild(style);
						style.onload = () => {
							content.innerHTML = data.content;
						};
					} else {
						content.innerHTML = data.content;
					}
				});
		});
	});
})();