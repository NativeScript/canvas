import { dispatchToMainThread } from './mainthread-helper';

let scheduled = false;

let macroTaskQueue: Array<Function> = [];

function drainMacrotaskQueue() {
	const currentQueue = macroTaskQueue;
	macroTaskQueue = [];
	scheduled = false;
	currentQueue.forEach((task) => {
		try {
			task();
		} catch (err) {
			//	const msg = err ? err.stack || err : err;
		}
	});
}

export function queueMacrotask(task: () => void) {
	macroTaskQueue.push(task);
	if (!scheduled) {
		scheduled = true;
		dispatchToMainThread(drainMacrotaskQueue);
	}
}
