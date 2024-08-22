class WorkerImpl extends Worker {
	constructor(url) {
		if (typeof url === 'string' && url.startsWith('blob:nativescript/')) {
			const path = (<any>URL).InternalAccessor.getPath(url);
			super(path);
		} else {
			super(url);
		}
	}
}

global.Worker = WorkerImpl;
