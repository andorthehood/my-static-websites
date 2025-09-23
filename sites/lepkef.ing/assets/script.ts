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

const pageSpecificStyleTags = [];
const pageCache = new Map();

function handleStyleTags(data) {
	pageSpecificStyleTags.forEach(style => style.remove());

	return new Promise((resolve) => {
		if (!data.css) {
			resolve();
		} else {
			const head = document.querySelector('head');
			const style = document.createElement('link');
			style.rel = 'stylesheet';
			style.href = '/assets/' + data.css;
			head.appendChild(style);
			pageSpecificStyleTags.push(style);

			style.onload = () => {
				resolve();
			};
		}
	});
}

function replaceContent(data) {
	const content = document.querySelector('.content');
	handleStyleTags(data)
		.then(() => {
			content.innerHTML = data.content;
			registerLinkHandlers();
		});
}

function handleLinkClick(event) {
	const link = event.currentTarget;
	const href = link.getAttribute('href');

	// If the link is external, don't do anything.
	if (href.startsWith('http')) {
		return;
	}

	// If the link is a hash, don't do anything.
	if (href.startsWith('#')) {
		return;
	}

	event.preventDefault();

	const content = document.querySelector('.content');

	const json = (href === '/' || href === '') ? '/index.json' : href + '.json';
	
	// Check cache first
	if (pageCache.has(json)) {
		const data = pageCache.get(json);
		window.history.pushState({}, '', link.href);
		replaceContent(data);
		return;
	}

	document.body.classList.remove('collapsed');
	content.innerHTML = 'Loading...';
	fetch(json)
		.then(response => response.json())
		.then(data => {
			// Cache the response
			pageCache.set(json, data);
			
			window.history.pushState({}, '', link.href);
			replaceContent(data);
		});
}

function registerLinkHandlers() {
	const links = document.querySelectorAll('a');
	links.forEach(link => {
		link.removeEventListener('click', handleLinkClick);
		link.addEventListener('click', handleLinkClick);
	});
}

(function () {
	window.addEventListener('popstate', function () {
		if (location.hash !== '') {
			return;
		}
		
		const pathname = location.pathname;
		const json = (pathname === '/' || pathname === '') ? '/index.json' : pathname + '.json';
		const content = document.querySelector('.content');
		
		// Check cache first
		if (pageCache.has(json)) {
			const data = pageCache.get(json);
			replaceContent(data);
			return;
		}
		
		document.body.classList.remove('collapsed');
		content.innerHTML = 'Loading...';
		fetch(location.origin + json)
			.then(response => response.json())
			.then(data => {
				// Cache the response
				pageCache.set(json, data);
				replaceContent(data);
			});
	});

	registerLinkHandlers();
})();