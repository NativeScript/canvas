// Only wrap the native Worker if it exists. Avoid clobbering existing Worker polyfills.
if (typeof Worker !== 'undefined') {
	const BaseWorker: any = Worker;
	const WorkerImpl = class extends BaseWorker {
		constructor(url) {
			if (typeof url === 'string' && url.startsWith('blob:nativescript/')) {
				const path = (URL as any).InternalAccessor?.getPath?.(url) ?? url;
				super(path as any);
			} else {
				super(url as any);
			}
		}
	};

	// Node-API hosts' runtimes take paths only: blob URLs go through the file they are written to.
	if (typeof (global as any).Worker === 'undefined' || (!__ANDROID__ && !__APPLE__)) {
		(global as any).Worker = WorkerImpl;
	}
}
