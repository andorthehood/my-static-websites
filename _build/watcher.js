const chokidar = require('chokidar');
const devserver = require('./devserver');
const debounce = require('lodash.debounce');
const { exec } = require('child_process');

console.log('Watching...');

chokidar.watch(['./_layouts', './_pages', './_posts']).on(
	'all',
	debounce((action, file) => {
		console.log(action, file);
		exec('node _build/index.js', (error, stdout, stderr) => {
			if (error) {
				console.log(error.message);
				return;
			}
			if (stderr) {
				console.log(stderr);
				return;
			}
			console.log(stdout);
		});
	}, 1000)
);

devserver();
