import { Folder, knownFolders, path, Utils, File as NSFile } from '@nativescript/core';

const BLOB_PATH = 'blob:nativescript/';
const BLOB_DIR = 'ns_blobs';
const BLOB_KEYS = 'org.nativescript.canvas.blob.keys';

let sharedPreferences;

// replace the internal blob handling
const BLOB_STORE = new Map();
URL.createObjectURL = function (object, options = null) {
	try {
		if (object instanceof Blob || object instanceof File) {
			// todo use faster method
			const id = crypto.randomUUID();
			const ret = `blob:nativescript/${id}`;
			BLOB_STORE.set(ret, {
				blob: object,
				type: object?.type,
				ext: options?.ext,
			});
			return ret;
		}
	} catch (error) {
		return null;
	}
	return null;
};
URL.revokeObjectURL = function (url) {
	BLOB_STORE.delete(url);
};

const InternalAccessor = class {};
(<any>InternalAccessor).getData = function (url) {
	return BLOB_STORE.get(url);
};
(<any>URL).InternalAccessor = InternalAccessor;

if (__ANDROID__) {
	(<any>URL).InternalAccessor.getPath = (url: string) => {
		const blob = (<any>URL).InternalAccessor.getData(url);
		if (!blob) {
			return '';
		}
		if (blob.path) {
			return blob.path;
		}
		const buf = (Blob as any).InternalAccessor.getBuffer(blob.blob);
		const path = (<any>org).nativescript.canvas.polyfill.Utils.getItemOrCreateAndReturn(Utils.android.getApplicationContext(), url, buf, buf.byteOffset, blob?.type ?? null, blob?.ext ?? null);
		blob.path = path;
		BLOB_STORE.set(url, blob);
		return path;
	};
}

if (__IOS__) {
	const putItem = (key: string, value: string) => {
		if (!sharedPreferences) {
			sharedPreferences = NSUserDefaults.alloc().initWithSuiteName(BLOB_KEYS);
		}

		(<NSUserDefaults>sharedPreferences).setObjectForKey(value, key);
	};

	const createObjectURLLegacyWithId = (id: string, object: any, options = null) => {
		const buf = (Blob as any).InternalAccessor.getBuffer(object);
		if (buf || object instanceof Blob || object instanceof File) {
			const exists = Folder.exists(path.join(knownFolders.documents().path, BLOB_DIR));
			if (!exists) {
				Folder.fromPath(path.join(knownFolders.documents().path, BLOB_DIR));
			}
			let fileName = id;
			// todo get type from magic bytes
			if (options?.ext) {
				fileName = `${fileName}.${options.ext}`;
			}

			const filePath = path.join(knownFolders.documents().path, BLOB_DIR, fileName);

			NSFile.fromPath(filePath).writeSync(NSData.dataWithData(buf));

			putItem(id, fileName);
			return `${BLOB_PATH}${id}`;
		}
		return null;
	};

	const getItem = (key: string) => {
		const fileDir = Folder.fromPath(path.join(knownFolders.documents().path, BLOB_DIR));
		let fileName = null;

		if (!sharedPreferences) {
			sharedPreferences = NSUserDefaults.alloc().initWithSuiteName(BLOB_KEYS);
		}

		if (!(<NSUserDefaults>sharedPreferences).objectForKey(key)) {
			return null;
		}

		fileName = (<NSUserDefaults>sharedPreferences).stringForKey(key);

		if (fileName) {
			return path.join(fileDir.path, fileName);
		}
		return null;
	};

	(<any>URL).InternalAccessor.getPath = (url: string) => {
		const blob = (<any>URL).InternalAccessor.getData(url);
		if (!blob) {
			return '';
		}
		if (blob.path) {
			return blob.path;
		}
		//const buf = (Blob as any).InternalAccessor.getBuffer(blob.blob);

		const id = url.replace(BLOB_PATH, '');

		if (id === '') {
			return '';
		}

		const created = createObjectURLLegacyWithId(id, blob.blob, {
			type: blob?.type,
			ext: blob?.ext,
		});

		if (!created) {
			return '';
		}

		let fileName = id;

		if (blob?.ext) {
			fileName = `${fileName}.${blob?.ext}`;
		}

		const filePath = path.join(knownFolders.documents().path, BLOB_DIR, fileName);

		blob.path = filePath;

		BLOB_STORE.set(url, blob);
		return filePath;
	};
}
