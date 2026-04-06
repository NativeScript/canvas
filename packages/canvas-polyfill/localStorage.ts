import { File, knownFolders } from '@nativescript/core';

let file: File;
let localStorageTimeout = null;

const internalSaveData = function (storage: Storage) {
	try {
		file.writeText(JSON.stringify(storage._localStorageData));
	} catch (err) {
		console.log('localStorage: unable to write storage, error: ', err);
	}
};

const saveData = function (storage: Storage) {
	if (localStorageTimeout !== null) {
		clearTimeout(localStorageTimeout);
	}
	localStorageTimeout = setTimeout(() => {
		localStorageTimeout = null;
		internalSaveData(storage);
	}, 250);
};

class Storage {
	_localStorageData: Record<string, string> = {};
	// Track length explicitly to avoid Object.keys() on every read
	private _length: number = 0;

	getItem(name: string): string | null {
		const val = this._localStorageData[name];
		return val !== undefined ? val : null;
	}

	key(id: number): string | null {
		const keys = Object.keys(this._localStorageData);
		return id < keys.length ? keys[id] : null;
	}

	setItemObject(name: string, value: any) {
		if (!(name in this._localStorageData)) {
			this._length++;
		}
		this._localStorageData[name] = value;
		saveData(this);
	}

	setItem(name: string, value: any) {
		if (!(name in this._localStorageData)) {
			this._length++;
		}
		if (value == null) {
			this._localStorageData[name] = value === null ? 'null' : 'undefined';
		} else {
			this._localStorageData[name] = value.toString();
		}
		saveData(this);
	}

	removeItem(name: string) {
		// Use `in` operator — not truthy check — so falsy values (0, '') are handled correctly
		if (name in this._localStorageData) {
			delete this._localStorageData[name];
			this._length--;
			saveData(this);
		}
	}

	clear() {
		this._localStorageData = {};
		this._length = 0;
		saveData(this);
	}

	get length(): number {
		return this._length;
	}
}

class SessionStorage {
	private _storage = new Map<string, string>();

	getItem(name: string): string | null {
		return this._storage.get(name) ?? null;
	}

	key(id: number): string | null {
		const keys = Array.from(this._storage.keys());
		return id < keys.length ? keys[id] : null;
	}

	setItemObject(name: string, value: any) {
		this._storage.set(name, value);
	}

	setItem(name: string, value: any) {
		if (value == null) {
			this._storage.set(name, value === null ? 'null' : 'undefined');
		} else {
			this._storage.set(name, value.toString());
		}
	}

	removeItem(name: string) {
		this._storage.delete(name);
	}

	clear() {
		this._storage.clear();
	}

	get length(): number {
		return this._storage.size;
	}
}

if (!global.Storage) {
	(<any>global).Storage = Storage;
}

if (!global.localStorage || (<any>module).hot) {
	const path = knownFolders.documents().path + '/localStorage.db';
	file = File.fromPath(path);
	localStorageTimeout = null;

	const loadData = function (storage: Storage) {
		if (!File.exists(path)) {
			return;
		}
		if (file.size === 0) {
			return;
		}
		try {
			const textData = file.readTextSync();
			const data = JSON.parse(textData);
			storage._localStorageData = data;
			// Sync the length counter after loading from disk
			(<any>storage)._length = Object.keys(data).length;
		} catch (err) {
			console.log('localStorage: error reading storage, Error: ', err);
		}
	};

	const storage = new Storage();
	loadData(storage);
	global.localStorage = storage;
}

if (!global.sessionStorage) {
	global.sessionStorage = new SessionStorage();
}

export default global.localStorage;

if ((<any>module).hot) {
	(<any>module).hot.accept();
	(<any>module).hot.dispose(() => {
		// Flush any pending writes before dispose
		if (localStorageTimeout !== null) {
			clearTimeout(localStorageTimeout);
			localStorageTimeout = null;
			if (global.localStorage) {
				internalSaveData(global.localStorage as any);
			}
		}
		global.localStorage = undefined;
	});
}
