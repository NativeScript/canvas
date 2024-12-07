export function dispatchToMainThread(func: () => void): void {
	NSOperationQueue.mainQueue.addOperationWithBlock(func);
}

export function isMainThread(): boolean {
	return NSThread.isMainThread;
}

export function dispatchToUIThread(func: () => void): void {
	const runloop = CFRunLoopGetMain();
	if (runloop && func) {
		CFRunLoopPerformBlock(runloop, kCFRunLoopDefaultMode, func);
		CFRunLoopWakeUp(runloop);
	} else if (func) {
		func();
	}
}
