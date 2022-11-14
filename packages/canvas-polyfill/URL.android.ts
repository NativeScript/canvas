import { knownFolders, File as NSFile, isIOS, path, Folder, Application } from '@nativescript/core';
const BLOB_PATH = 'blob:nativescript/';
const BLOB_DIR = 'ns_blobs';
const BLOB_KEYS = 'org.nativescript.canvas.blob.keys';

let sharedPreferences;
export class URL {
	#native: java.net.URI;
	#isBlobURL = false;
	constructor(url: string, base?: string | URL) {
		if (url?.startsWith?.('blob:')) {
			this.#isBlobURL = true;
		}
		let baseUrl: java.net.URL;
		if (typeof url === 'string' && url.startsWith('blob:')) {
			this.#native = new java.net.URI(url);
		} else {
			if (base instanceof URL) {
				baseUrl = base.#native.toURL();
			} else if (base) {
				try {
					baseUrl = new java.net.URL(base);
				} catch (e) {
					throw new TypeError(`Failed to construct 'URL': Invalid base URL`);
				}
			}
			try {
				if (baseUrl) {
					this.#native = new java.net.URL(baseUrl, url).toURI();
				} else {
					this.#native = new java.net.URL(url).toURI();
				}
			} catch (e) {
				throw new TypeError(`Failed to construct 'URL': Invalid URL`);
			}
		}
	}

	get native() {
		return this.#native.toURL();
	}

	private _updateURL() {
		const currentURL = this.native;
		this.#native = new java.net.URI(this.#protocol || currentURL.getProtocol(), `${this.#username}${this.password ? ':' : ''}${this.#password}` || currentURL.getUserInfo(), this.#hostname || currentURL.getHost(), this.#port || currentURL.getPort(), this.#pathname || currentURL.getPath(), this.#search || currentURL.getQuery(), this.#hash || currentURL.getRef());
	}

	get hash() {
		const hash = this.#native.getFragment();
		return hash ? `#${hash}` : '';
	}

	#hash = '';
	set hash(value: string) {
		this.#hash = value;
		this._updateURL();
	}

	get host() {
		const port = this.#native.getPort();
		const scheme = this.protocol;
		let returnPort = true;
		if (scheme === 'https' && port === 443) {
			returnPort = false;
		}
		return `${this.#native.getHost()}${returnPort && port != -1 ? ':' : ''}${returnPort && port != -1 ? port : ''}`;
	}

	set host(value: string) {
		const url = new java.net.URL(value);
		this.#port = url.getPort();
		this.#hostname = url.getHost();
		this._updateURL();
	}

	get hostname() {
		return this.#native.getHost();
	}

	#hostname = '';
	set hostname(value: string) {
		this.#hostname = value;
		this._updateURL();
	}

	get href() {
		return this.#native.toString();
	}

	set href(value: string) {
		this.#native = new java.net.URI(value);
	}

	get origin() {
		let url = this.#native;
		if (this.#isBlobURL) {
			url = new java.net.URI(this.#native.toString().replace('blob:', ''));
		}

		return `${url.getScheme()}://${url.getHost()}`;
	}

	#password = '';
	get password() {
		const user = this.#native.getUserInfo() || '';
		return user.split(':')[1] || '';
	}

	set password(value: string) {
		this.#password = value;
		this._updateURL();
	}

	get pathname() {
		return this.#native.getPath();
	}

	#pathname = '';
	set pathname(value: string) {
		this.#pathname = value;
		this._updateURL();
	}

	get port() {
		const port = this.#native.getPort();
		if (port === -1) {
			return '';
		}
		return `${port}`;
	}

	#port = -1;
	set port(value: string) {
		this.#port = +value;
		this._updateURL();
	}

	get protocol() {
		return this.#native.getScheme() + ':';
	}

	#protocol = '';
	set protocol(value: string) {
		this.#protocol = value;
		this._updateURL();
	}

	get search() {
		const query = this.#native.getQuery();
		return query ? `?${query}` : '';
	}

	#search = '';
	set search(value: string) {
		this.#search = value;
		this._updateURL();
	}

	get username() {
		const user = this.#native.getUserInfo() || '';
		return user.split(':')[0] || '';
	}

	#username = '';
	set username(value: string) {
		this.username = value;
		this._updateURL();
	}

	toJSON() {
		return this.#native.toString();
	}

	toString() {
		return this.#native.toString();
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
				const file = new java.io.File(filePath);
				// faster write
				const channel = new java.io.FileOutputStream(file).getChannel();
				channel.write(buf);
				// fos.write(Array.from(buf) as any);
				// fos.flush();
				// fos.close();
			} catch (e) {
				return null;
			}


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
