import { knownFolders, File as NSFile, isIOS, path, Folder, Application, Utils } from '@nativescript/core';
const BLOB_PATH = 'blob:nativescript/';
const BLOB_DIR = 'ns_blobs';
const BLOB_KEYS = 'org.nativescript.canvas.blob.keys';
declare const org;

const MIME_TO_EXT = {
	'image/jpeg': '.jpg',
	'image/png': '.png',
	'image/gif': '.gif',
};

function get_mime_ext(value): string {
	return '.' + MIME_TO_EXT[value] ?? '';
}

let sharedPreferences;
export class URL {
	_native: java.net.URI;
	_isBlobURL = false;
	constructor(url: string, base?: string | URL) {
		if (url?.startsWith?.('blob:')) {
			this._isBlobURL = true;
		}
		let baseUrl: java.net.URL;
		if (typeof url === 'string' && url.startsWith('blob:')) {
			this._native = new java.net.URI(url);
		} else {
			if (base instanceof URL) {
				baseUrl = base._native.toURL();
			} else if (base) {
				try {
					baseUrl = new java.net.URL(base);
				} catch (e) {
					throw new TypeError(`Failed to construct 'URL': Invalid base URL`);
				}
			}
			try {
				if (baseUrl) {
					this._native = new java.net.URL(baseUrl, url).toURI();
				} else {
					this._native = new java.net.URL(url).toURI();
				}
			} catch (e) {
				throw new TypeError(`Failed to construct 'URL': Invalid URL`);
			}
		}
	}

	get native() {
		return this._native.toURL();
	}

	private _updateURL() {
		const currentURL = this.native;
		this._native = new java.net.URI(this._protocol || currentURL.getProtocol(), `${this._username}${this.password ? ':' : ''}${this._password}` || currentURL.getUserInfo(), this._hostname || currentURL.getHost(), this._port || currentURL.getPort(), this._pathname || currentURL.getPath(), this._search || currentURL.getQuery(), this._hash || currentURL.getRef());
	}

	get hash() {
		const hash = this._native.getFragment();
		return hash ? `#${hash}` : '';
	}

	_hash = '';
	set hash(value: string) {
		this._hash = value;
		this._updateURL();
	}

	get host() {
		const port = this._native.getPort();
		const scheme = this.protocol;
		let returnPort = true;
		if (scheme === 'https' && port === 443) {
			returnPort = false;
		}
		return `${this._native.getHost()}${returnPort && port != -1 ? ':' : ''}${returnPort && port != -1 ? port : ''}`;
	}

	set host(value: string) {
		const url = new java.net.URL(value);
		this._port = url.getPort();
		this._hostname = url.getHost();
		this._updateURL();
	}

	get hostname() {
		return this._native.getHost();
	}

	_hostname = '';
	set hostname(value: string) {
		this._hostname = value;
		this._updateURL();
	}

	get href() {
		return this._native.toString();
	}

	set href(value: string) {
		this._native = new java.net.URI(value);
	}

	get origin() {
		let url = this._native;
		if (this._isBlobURL) {
			url = new java.net.URI(this._native.toString().replace('blob:', ''));
		}

		return `${url.getScheme()}://${url.getHost()}`;
	}

	_password = '';
	get password() {
		const user = this._native.getUserInfo() || '';
		return user.split(':')[1] || '';
	}

	set password(value: string) {
		this._password = value;
		this._updateURL();
	}

	get pathname() {
		return this._native.getPath();
	}

	_pathname = '';
	set pathname(value: string) {
		this._pathname = value;
		this._updateURL();
	}

	get port() {
		const port = this._native.getPort();
		if (port === -1) {
			return '';
		}
		return `${port}`;
	}

	_port = -1;
	set port(value: string) {
		this._port = +value;
		this._updateURL();
	}

	get protocol() {
		return this._native.getScheme() + ':';
	}

	_protocol = '';
	set protocol(value: string) {
		this._protocol = value;
		this._updateURL();
	}

	get search() {
		const query = this._native.getQuery();
		return query ? `?${query}` : '';
	}

	_search = '';
	set search(value: string) {
		this._search = value;
		this._updateURL();
	}

	get username() {
		const user = this._native.getUserInfo() || '';
		return user.split(':')[0] || '';
	}

	_username = '';
	set username(value: string) {
		this.username = value;
		this._updateURL();
	}

	toJSON() {
		return this._native.toString();
	}

	toString() {
		return this._native.toString();
	}

	public static createObjectURL(object: any, options = null): string {
		const buf = (Blob as any).InternalAccessor.getBuffer(object);
		if (buf || object instanceof Blob || object instanceof File) {
			return org.nativescript.canvas.polyfill.Utils.createObjectURL(Utils.android.getApplicationContext(), buf, buf.byteOffset, options?.type ?? null);
		}
		return null;
	}

	public static revokeObjectURL(url: string) {
		org.nativescript.canvas.polyfill.Utils.revokeObjectURL(Utils.android.getApplicationContext(), url ?? null);
	}

	public static InternalAccessor = class {
		public static getPath(url: string) {
			return org.nativescript.canvas.polyfill.Utils.getPath(Utils.android.getApplicationContext(), url ?? null);
		}
	};
}
