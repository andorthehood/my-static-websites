document.getElementById('button-collapse').addEventListener('click', function (event) {
	document.body.classList.toggle('collapsed');
});
document.getElementById('button-close').addEventListener('click', function () {
	document.body.classList.add('closed');
	window.history.pushState({}, '', '/');
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

const pageCache = new Map();

function getJsonPath(path) {
	return (path === '/' || path === '') ? '/index.json' : path + '.json';
}

function resetUiState() {
	document.body.classList.remove('collapsed');
	document.body.classList.remove('closed');
}

function showLoadingState() {
	const content = document.querySelector('.content');
	if (content) {
		content.innerHTML = 'Loading...';
	}
}

function loadAndReplaceContent(json, fetchUrl?: string) {
	// Check cache first
	if (pageCache.has(json)) {
		const data = pageCache.get(json);
		replaceContent(data);
		return;
	}

	fetch(fetchUrl || json)
		.then(response => response.json())
		.then(data => {
			// Cache the response
			pageCache.set(json, data);
			replaceContent(data);
		});
}

function navigateToJson(json, fetchUrl?: string) {
	resetUiState();
	showLoadingState();
	loadAndReplaceContent(json, fetchUrl);
}

function handleStyleTags(data): Promise<void> {
	const pageSpecificStyleTags = document.querySelectorAll('link.page-specific-css');
	pageSpecificStyleTags.forEach(style => style.remove());

	return new Promise((resolve) => {
		if (!data.css) {
			resolve();
		} else {
			const head = document.querySelector('head');
			const style = document.createElement('link');
			style.rel = 'stylesheet';
			style.href = '/assets/' + data.css;
			style.classList.add('page-specific-css');
			head.appendChild(style);

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

	window.history.pushState({}, '', link.href);

	const json = getJsonPath(href);
	navigateToJson(json);
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
		const json = getJsonPath(pathname);
		navigateToJson(json, location.origin + json);
	});

	registerLinkHandlers();
})();
