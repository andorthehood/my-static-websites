module.exports = async module => {
	return new Promise(resolve => {
		setImmediate(() => {
			resolve(require(module));
		});
	});
};
