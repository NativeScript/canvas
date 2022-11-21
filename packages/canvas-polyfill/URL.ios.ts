import { knownFolders, File as NSFile, isIOS, path, Folder, Application } from '@nativescript/core';
const BLOB_PATH = 'blob:nativescript/';
const BLOB_DIR = 'ns_blobs';
const BLOB_KEYS = 'org.nativescript.canvas.blob.keys';

let sharedPreferences;
declare const Utils;
export class URL {
	#native: NSURLComponents;
	constructor(url: string, base?: string | URL) {
		let baseUrl: NSURL;
		let nativeURL: NSURL;
		if (base instanceof URL) {
			baseUrl = base.#native.URL;
		} else {
			baseUrl = NSURL.URLWithString(base);
		}

		if (baseUrl) {
			nativeURL = NSURL.URLWithStringRelativeToURL(url, baseUrl);
		} else {
			nativeURL = NSURL.URLWithString(url);
		}

		this.#native = NSURLComponents.componentsWithString(nativeURL.absoluteString);
	}

	get native() {
		return this.#native.URL;
	}

	get hash() {
		const hash = this.native.fragment;
		return hash ? `#${hash}` : '';
	}

	set hash(value: string) {
		this.#native.fragment = value;
	}

	get host() {
		return this.native.host;
	}

	set host(value: string) {
		this.#native.host = value;
	}

	get hostname() {
		return this.native.host;
	}

	set hostname(value: string) {
		this.#native.host = value;
	}

	get href() {
		return this.#native.URL.absoluteString;
	}

	set href(value: string) {
		this.#native = NSURLComponents.componentsWithString(value);
	}

	get origin() {
		return `${this.native.scheme}${this.native.host}`;
	}

	get password() {
		return this.native.password;
	}

	set password(value: string) {
		this.#native.password = value;
	}

	get pathname() {
		return this.native.path;
	}

	set pathname(value: string) {
		this.#native.path = value;
	}

	get port() {
		return String(this.native.port);
	}

	set port(value: string) {
		this.#native.port = +value;
	}

	get protocol() {
		return this.native.scheme + ':';
	}

	set protocol(value: string) {
		this.#native.scheme = value;
	}

	get search() {
		const query = this.native.query;
		return query ? `?${query}` : '';
	}

	set search(value: string) {
		this.#native.query = value;
	}

	get username() {
		return this.native.user;
	}

	set username(value: string) {
		this.#native.user = value;
	}

	toJSON() {
		return this.native.toString();
	}

	toString() {
		return this.native.toString();
	}

	public static createObjectURL(object: any, options = null): string {
		const buf = (Blob as any).InternalAccessor.getBuffer(object);
		if (!!buf || object instanceof Blob || object instanceof File) {
			const id = this.getUUID();
			const exists = Folder.exists(path.join(knownFolders.documents().path, BLOB_DIR));
			if (!exists) {
				Folder.fromPath(path.join(knownFolders.documents().path, BLOB_DIR));
			}
			let fileName = id;
			// todo get type from magic bytes
			if (options?.appendExt) {
				fileName = `${fileName}.${options.ext}`;
			}

			const filePath = path.join(knownFolders.documents().path, BLOB_DIR, fileName);

			try {
				Utils.writeToFileError(buf, filePath);
			} catch (error) {}

			URL.putItem(id, fileName);
			return `${BLOB_PATH}${id}`;
		}
		return null;
	}

	public static revokeObjectURL(url: string) {
		if (typeof url === 'string') {
			const id = url.replace(BLOB_PATH, '');
			const realPath = URL.getItem(id);
			if (NSFile.exists(realPath)) {
				const file = NSFile.fromPath(realPath);
				file.removeSync();
			}
		}
	}

	public static InternalAccessor = class {
		public static getPath(url: string) {
			if (typeof url === 'string') {
				const id = url.replace(BLOB_PATH, '');
				return URL.getItem(id);
			}
			return '';
		}
	};

	private static getUUID() {
		if (isIOS) {
			return NSUUID.UUID().UUIDString;
		}
		return java.util.UUID.randomUUID().toString();
	}

	private static putItem(key: string, value: string) {
		if (global.isAndroid) {
			if (!sharedPreferences) {
				sharedPreferences = (<android.app.Application>Application.getNativeApplication()).getApplicationContext().getSharedPreferences(BLOB_KEYS, 0);
			}

			(<android.content.SharedPreferences>sharedPreferences).edit().putString(key, value).apply();
		} else {
			if (!sharedPreferences) {
				sharedPreferences = NSUserDefaults.alloc().initWithSuiteName(BLOB_KEYS);
			}

			(<NSUserDefaults>sharedPreferences).setObjectForKey(value, key);
		}
	}

	private static getItem(key: string) {
		const fileDir = Folder.fromPath(path.join(knownFolders.documents().path, BLOB_DIR));
		let fileName = null;
		if (global.isAndroid) {
			if (!sharedPreferences) {
				sharedPreferences = (<android.app.Application>Application.getNativeApplication()).getApplicationContext().getSharedPreferences(BLOB_KEYS, 0);
			}

			fileName = (<android.content.SharedPreferences>sharedPreferences).getString(key, null);
		} else {
			if (!sharedPreferences) {
				sharedPreferences = NSUserDefaults.alloc().initWithSuiteName(BLOB_KEYS);
			}

			if (!(<NSUserDefaults>sharedPreferences).objectForKey(key)) {
				return null;
			}

			fileName = (<NSUserDefaults>sharedPreferences).stringForKey(key);
		}

		if (fileName) {
			return path.join(fileDir.path, fileName);
		}
		return null;
	}
}
