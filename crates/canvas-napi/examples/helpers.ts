import '@nativescript/macos-node-api';

function readTextSync(file: string): string | null {
	if (!file) {
		return null;
	}
	const path = import.meta.resolve(file);
	console.log(path);
	const data = NSData.dataWithContentsOfFile(path?.replace('file://', ''));
	if (!data) {
		return null;
	}
	return NSString.alloc().initWithDataEncoding(data, NSUTF8StringEncoding).toString();
}

export default {
	readTextSync,
};
