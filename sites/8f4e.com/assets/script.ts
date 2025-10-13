function mainThreadLogicRuntime(state, events) {
	let runtime;

	function onInitialized() {
		events.dispatch('runtimeInitialized');
	}

	function onStats(stats: { drift: number; timeToExecute: number }) {
		console.log(stats);
	}

	function onError(error: unknown) {
		console.error('Main thread runtime error:', error);
	}

	function syncCodeAndSettingsWithRuntime() {
		if (!runtime) {
			return;
		}
		runtime.init(
			state.compiler.memoryRef,
			state.project.runtimeSettings[state.project.selectedRuntime].sampleRate,
			state.compiler.codeBuffer
		);
	}

	runtime = RuntimeMainThreadLogic(onInitialized, onStats, onError);
	syncCodeAndSettingsWithRuntime();

	events.on('syncCodeAndSettingsWithRuntime', syncCodeAndSettingsWithRuntime);

	return () => {
		events.off('syncCodeAndSettingsWithRuntime', syncCodeAndSettingsWithRuntime);
		if (runtime) {
			runtime.stop();
			runtime = undefined;
		}
	};
}

(async function () {

	const canvas = document.getElementById('glcanvas');

	canvas.width = window.innerWidth;
	canvas.height = window.innerHeight;

	const editor = await Editor8f4e(canvas, {
		featureFlags: {
			persistentStorage: true,
			infoOverlay: false,
			viewportDragging: false,
			moduleDragging: false,
			contextMenu: false,
		},
		callbacks: {
			requestRuntime: async function () {
				console.log("requestRuntime");
				return mainThreadLogicRuntime;
			},
			loadProjectFromStorage: async function () {
				const response = await fetch("https://static.llllllllllll.com/andor/8f4e/test-project-3.json");
				return await response.json();
			},
		}
	});

	editor.resize(window.innerWidth, window.innerHeight);

	window.addEventListener('resize', () => {
		canvas.width = window.innerWidth;
		canvas.height = window.innerHeight;
		editor.resize(window.innerWidth, window.innerHeight);
	});
})();