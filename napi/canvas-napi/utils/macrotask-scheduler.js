const { dispatchToMainThread } = require('./mainthread-helper');

let scheduled = false;
/**
 * @type {Array<Function>}
 */
let macroTaskQueue = [];

function drainMacrotaskQueue() {
	const currentQueue = macroTaskQueue;
	macroTaskQueue = [];
	scheduled = false;
	currentQueue.forEach((task) => {
		try {
			task();
		} catch (err) {
			const msg = err ? err.stack || err : err;
		}
	});
}

/**
 *
 * @param {function(): void} task
 */
function queueMacrotask(task) {
	macroTaskQueue.push(task);
	if (!scheduled) {
		scheduled = true;
		dispatchToMainThread(drainMacrotaskQueue);
	}
}

module.exports.queueMacrotask = queueMacrotask;
