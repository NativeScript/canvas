/**
 *
 * @param {function(): void} func
 */
function dispatchToMainThread(func) {
	NSOperationQueue.mainQueue.addOperationWithBlock(func);
}

/**
 *
 * @returns {boolean}
 */
function isMainThread() {
	return NSThread.isMainThread;
}

/**
 *
 * @param {function():void} func
 */
function dispatchToUIThread(func) {
	const runloop = CFRunLoopGetMain();
	if (runloop && func) {
		CFRunLoopPerformBlock(runloop, kCFRunLoopDefaultMode, func);
		CFRunLoopWakeUp(runloop);
	} else if (func) {
		func();
	}
}

module.exports.dispatchToMainThread = dispatchToMainThread;
module.exports.isMainThread = isMainThread;
module.exports.dispatchToUIThread = dispatchToUIThread;
