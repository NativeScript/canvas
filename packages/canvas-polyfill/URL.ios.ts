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

/*
export class URLSearchParams {
	_native = new Map();
	constructor(search: string | URLSearchParams) {
		if (search instanceof URLSearchParams) {
			this._native = new Map(search._native);
		} else if (typeof search === 'string') {
			this._native = NSURLComponents.alloc().initWithString(search);
		} else {
			this._native = NSURLComponents.alloc().initWithString((<any>search)?.toString?.());
		}
	}

	append(name: string, value: string) {
		let queryItems: NSMutableArray<NSURLQueryItem> = this._native.queryItems.mutableCopy();
		if (!queryItems) {
			queryItems = NSMutableArray.array();
		}

		if (value) {
			queryItems.addObject(NSURLQueryItem.queryItemWithNameValue(name, value));
		}

		this._native.queryItems = queryItems;
	}

	set(name: string, value: string) {
		let queryItems: NSMutableArray<NSURLQueryItem> = this._native.queryItems.mutableCopy();
		if (!queryItems) {
			queryItems = NSMutableArray.array();
		}

		queryItems.filterUsingPredicate(NSPredicate.predicateWithFormatArgumentArray('name != %@', NSArray.arrayWithObject(value)));

		if (value) {
			queryItems.addObject(NSURLQueryItem.queryItemWithNameValue(name, value));
		}

		this._native.queryItems = queryItems;
	}

	get(name: string) {
		const queryItems = this._native.queryItems;
		const count = queryItems.count;
		for (let i = 0; i < count; i++) {
			const queryItem = queryItems.objectAtIndex(i);
			if (queryItem.name === name) {
				return queryItem.value;
			}
		}
		return null;
	}

	delete(name: string, value?: string) {
		const queryItems: NSMutableArray<NSURLQueryItem> = this._native.queryItems.mutableCopy();
		if (!queryItems) {
			return;
		}

		if (!value) {
			queryItems.filterUsingPredicate(NSPredicate.predicateWithFormatArgumentArray('name != %@', NSArray.arrayWithObject(value)));
			this._native.queryItems = queryItems;
			return;
		}

		queryItems.filterUsingPredicate(NSPredicate.predicateWithFormatArgumentArray('name != %@ OR value != %@', NSArray.arrayWithArray([name, value])));

		this._native.queryItems = queryItems;
	}

	toString() {
		return this._native.string;
	}
}
*/
import './url-search';

/*
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

	get searchParams() {
		return new URLSearchParams(this.native.absoluteString);
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
*/


export class URL {
	_native;
	_isBlobURL = false;
	constructor(url: string, base?: string | URL) {
		if (url?.startsWith?.('blob:')) {
			this._isBlobURL = true;
		}
		let baseUrl: string;
		if (typeof url === 'string' && url.startsWith('blob:')) {
			this._native = new global.CanvasModule.URL(url);
		} else {
			if (base instanceof URL) {
				baseUrl = base._native.toString();
			} else if (base) {
				try {
					baseUrl = base.toString();
				} catch (e) {
					throw new TypeError(`Failed to construct 'URL': Invalid base URL`);
				}
			}
			try {
				if (baseUrl) {
					this._native = new global.CanvasModule.URL(url, baseUrl);
				} else {
					this._native = new global.CanvasModule.URL(url);
				}
			} catch (e) {
				throw new TypeError(`Failed to construct 'URL': Invalid URL`);
			}
		}
	}

	get native() {
		return this._native;
	}

	get hash() {
		return this.native.hash;
	}

	set hash(value: string) {
		this.native.hash = value;
	}

	get host() {
		return this.native.host;
	}

	set host(value: string) {
		this.native.host = value;
	}

	get hostname() {
		return this.native.hostname;
	}

	set hostname(value: string) {
		this.native.hostname = value;
	}

	get href() {
		return this.native.href;
	}

	set href(value: string) {
		this.native.href = value;
	}

	get origin() {
		// let url = this._native;
		// if (this._isBlobURL) {
		// 	url = new java.net.URI(this._native.toString().replace('blob:', ''));
		// }

		// return `${url.getScheme()}://${url.getHost()}`;

		return this.native.origin;
	}

	get password() {
		return this.native.password;
	}

	set password(value: string) {
		this.native.password = value;
	}

	get pathname() {
		return this.native.pathname;
	}

	set pathname(value: string) {
		this.native.pathname = value;
	}

	get port() {
		return this.native.port;
	}

	set port(value: string) {
		this.native.port = value;
	}

	get protocol() {
		return this.native.protocol;
	}

	set protocol(value: string) {
		this.native.protocol = value;
	}

	get search() {
		return this.native.search;
	}

	set search(value: string) {
		this.native.search = value;
	}

	get searchParams() {
		return new URLSearchParams(this.native.toString());
	}

	get username() {
		return this.native.username;
	}

	set username(value: string) {
		this.native.username = value;
	}

	toJSON() {
		return this.native.toString();
	}

	toString() {
		return this.native.toString();
	}

	public static canParse(url, base) {
		let ret = false;
		if (url?.startsWith?.('blob:')) {
			ret = true;
		}
		let baseUrl: string;
		if (typeof url === 'string' && url.startsWith('blob:')) {
			ret = true;
		} else {
			if (base instanceof URL) {
				baseUrl = base._native.toString();
			} else if (base) {
				try {
					baseUrl = base.toString();
				} catch (e) {
					throw new TypeError(`Failed to construct 'URL': Invalid base URL`);
				}
			}
			try {
				if (baseUrl) {
					ret = global.CanvasModule.URL.canParse(url, baseUrl);
				} else {
					ret = global.CanvasModule.URL.canParse(url);
				}
			} catch (e) {}
		}

		return ret;
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



class WorkerImpl extends Worker {
	constructor(url) {
		if (typeof url === 'string' && url.startsWith('blob:nativescript/')) {
			const path = URL.InternalAccessor.getPath(url);
			super(path);
		} else {
			super(url);
		}
	}
}

global.Worker = WorkerImpl;
