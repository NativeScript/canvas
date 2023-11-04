import { File, knownFolders } from '@nativescript/core';

let file: File;
let localStorageTimeout = null;

const internalSaveData = function (storage: Storage) {
	try {
		file.writeText(JSON.stringify(storage._localStorageData));
	} catch (err) {
		// This should never happen on normal data, but if they tried to put non JS stuff it won't serialize
		console.log('localStorage: unable to write storage, error: ', err);
	}
};

const saveData = function (storage: Storage) {
	if (localStorageTimeout !== null) {
		clearTimeout(localStorageTimeout);
	}
	localStorageTimeout = setTimeout(() => {
		internalSaveData(storage);
	}, 250);
};

class Storage {
	_localStorageData = {};
	getItem(name) {
		if (this._localStorageData.hasOwnProperty(name)) {
			return this._localStorageData[name];
		}
		return null;
	}
	key(id) {
		const keys = Object.keys(this._localStorageData);
		if (id >= keys.length) {
			return null;
		}
		return keys[id];
	}

	setItemObject(name, value) {
		this._localStorageData[name] = value;
		saveData(this);
	}
	// Revamp this to be "String" only
	// https://github.com/NathanaelA/nativescript-localstorage/issues/17
	setItem(name, value) {
		if (value == null) {
			if (value === null) {
				this._localStorageData[name] = 'null';
			} else {
				this._localStorageData[name] = 'undefined';
			}
		} else {
			this._localStorageData[name] = value.toString();
		}
		saveData(this);
	}
	removeItem(name) {
		if (this._localStorageData[name]) {
			delete this._localStorageData[name];
			saveData(this);
		}
	}

	clear() {
		this._localStorageData = {};
		saveData(this);
	}

	get length() {
		return Object.keys(this._localStorageData).length;
	}
}

class SessionStorage {
	private _storage = new Map();
	getItem(name) {
		return this._storage.get(name) ?? null;
	}
	key(id) {
		const keys = Array.from(this._storage.keys());
		if (id >= keys.length) {
			return null;
		}
		return keys[id];
	}

	setItemObject(name, value) {
		this._storage.set(name, value);
	}

	setItem(name, value) {
		if (value == null) {
			if (value === null) {
				this._storage.set(name, 'null');
			} else {
				this._storage.set(name, 'undefined');
			}
		} else {
			this._storage.set(name, value.toString());
		}
	}

	removeItem(name) {
		this._storage.delete(name);
	}
	clear() {
		this._storage.clear();
	}

	get length() {
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

		let data;
		if (file.size === 0) {
			return;
		}
		try {
			let textData = file.readTextSync();
			data = JSON.parse(textData);
			storage._localStorageData = data;
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
		global.localStorage = undefined;
	});
}
