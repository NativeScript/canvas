import { knownFolders, File as NSFile, isIOS, path, Folder, Application } from '@nativescript/core';
const BLOB_PATH = 'blob:nativescript/';
const BLOB_DIR = 'ns_blobs';
const BLOB_KEYS = 'org.nativescript.canvas.blob.keys';

let sharedPreferences;

interface BlobItem {
	blob: Blob;
	path?: string;
	type?: string;
	ext?: string;
}
const BLOB_STORE = new Map<string, BlobItem>();

export class URL {
	_native: NSURLComponents;
	constructor(url: string, base?: string | URL) {
		let baseUrl: NSURL;
		let nativeURL: NSURL;
		if (base instanceof URL) {
			baseUrl = base._native.URL;
		} else {
			baseUrl = NSURL.URLWithString(base);
		}

		if (baseUrl) {
			nativeURL = NSURL.URLWithStringRelativeToURL(url, baseUrl);
		} else {
			nativeURL = NSURL.URLWithString(url);
		}

		this._native = NSURLComponents.componentsWithString(nativeURL.absoluteString);
	}

	get native() {
		return this._native.URL;
	}

	get hash() {
		const hash = this.native.fragment;
		return hash ? `#${hash}` : '';
	}

	set hash(value: string) {
		this._native.fragment = value;
	}

	get host() {
		return this.native.host;
	}

	set host(value: string) {
		this._native.host = value;
	}

	get hostname() {
		return this.native.host;
	}

	set hostname(value: string) {
		this._native.host = value;
	}

	get href() {
		return this._native.URL.absoluteString;
	}

	set href(value: string) {
		this._native = NSURLComponents.componentsWithString(value);
	}

	get origin() {
		return `${this.native.scheme}${this.native.host}`;
	}

	get password() {
		return this.native.password;
	}

	set password(value: string) {
		this._native.password = value;
	}

	get pathname() {
		return this.native.path;
	}

	set pathname(value: string) {
		this._native.path = value;
	}

	get port() {
		return String(this.native.port);
	}

	set port(value: string) {
		this._native.port = +value;
	}

	get protocol() {
		return this.native.scheme + ':';
	}

	set protocol(value: string) {
		this._native.scheme = value;
	}

	get search() {
		const query = this.native.query;
		return query ? `?${query}` : '';
	}

	set search(value: string) {
		this._native.query = value;
	}

	get username() {
		return this.native.user;
	}

	set username(value: string) {
		this._native.user = value;
	}

	toJSON() {
		return this.native.toString();
	}

	toString() {
		return this.native.toString();
	}

	public static createObjectURL(object: any, options = null): string {
		//	const buf = (Blob as any).InternalAccessor.getBuffer(object);
		if (object instanceof Blob || object instanceof File) {
			const id = this.getUUID();
			const ret = `blob:nativescript/${id}`;
			BLOB_STORE.set(ret, {
				blob: object,
				type: object?.type,
				ext: options?.ext,
			});
			return ret;

			// const id = this.getUUID();
			// const exists = Folder.exists(path.join(knownFolders.documents().path, BLOB_DIR));
			// if (!exists) {
			// 	Folder.fromPath(path.join(knownFolders.documents().path, BLOB_DIR));
			// }
			// let fileName = id;
			// // todo get type from magic bytes
			// if (options?.appendExt) {
			// 	fileName = `${fileName}.${options.ext}`;
			// }

			// const filePath = path.join(knownFolders.documents().path, BLOB_DIR, fileName);

			// NSFile.fromPath(filePath).writeSync(NSData.dataWithData(buf));

			// URL.putItem(id, fileName);
			// return `${BLOB_PATH}${id}`;
		}
		return null;
	}

	public static createObjectURLLegacy(object: any, options = null): string {
		return this.createObjectURLLegacyWithId(this.getUUID(), object, options);
	}

	static createObjectURLLegacyWithId(id: string, object: any, options = null): string {
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

			URL.putItem(id, fileName);
			return `${BLOB_PATH}${id}`;
		}
		return null;
	}

	public static revokeObjectURL(url: string) {
		if (typeof url === 'string') {
			const blob = BLOB_STORE.get(url);
			if (blob.path) {
				if (NSFile.exists(blob.path)) {
					const file = NSFile.fromPath(blob.path);
					file.removeSync();
				}
			}
			BLOB_STORE.delete(url);

			// const id = url.replace(BLOB_PATH, '');
			// const realPath = URL.getItem(id);
			// if (NSFile.exists(realPath)) {
			// 	const file = NSFile.fromPath(realPath);
			// 	file.removeSync();
			// }
		}
	}

	public static InternalAccessor = class {
		public static getPath(url: string) {
			const blob = BLOB_STORE.get(url);
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

			const created = URL.createObjectURLLegacyWithId(id, blob.blob, {
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

			// if (typeof url === 'string') {
			// 	const id = url.replace(BLOB_PATH, '');
			// 	return URL.getItem(id);
			// }
			// return '';
		}

		public static getData(url: string) {
			return BLOB_STORE.get(url);
		}
	};

	private static getUUID() {
		return NSUUID.UUID().UUIDString;
	}

	private static putItem(key: string, value: string) {
		if (!sharedPreferences) {
			sharedPreferences = NSUserDefaults.alloc().initWithSuiteName(BLOB_KEYS);
		}

		(<NSUserDefaults>sharedPreferences).setObjectForKey(value, key);
	}

	private static getItem(key: string) {
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
	}
}
