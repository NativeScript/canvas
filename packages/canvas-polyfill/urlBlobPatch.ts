const BLOB_STORE = new Map();
if (__ANDROID__) {
	(<any>URL).createObjectURL = function (object, options: { ext?: string } | null = null) {
		try {
			console.log('createObjectURL', object);
			if (object instanceof Blob || object instanceof File) {
				const id = java.util.UUID.randomUUID().toString();
				const ret = `blob:nativescript/${id}`;
				BLOB_STORE.set(ret, {
					blob: object,
					type: object?.type,
					ext: options?.ext,
				});
				return ret;
			}
		} catch (error) {
			console.log('error', error);
			return null;
		}
		return null;
	};
}
URL.revokeObjectURL = function (url) {
	BLOB_STORE.delete(url);
};
const InternalAccessor = class {
	static getData = function (url) {
		console.log('getData', url);
		return BLOB_STORE.get(url);
	};
	static InternalAccessor() {
		return InternalAccessor;
	}
};

// Object.defineProperty(URL.prototype, 'searchParams', {
// 	get() {
// 		if (this._searchParams == null) {
// 			this._searchParams = new URLSearchParams(this.search);
// 			Object.defineProperty(this._searchParams, '_url', {
// 				enumerable: false,
// 				writable: false,
// 				value: this,
// 			});
// 			this._searchParams._append = this._searchParams.append;
// 			this._searchParams.append = function (name, value) {
// 				this._append(name, value);
// 				this._url.search = this.toString();
// 			};
// 			this._searchParams._delete = this._searchParams.delete;
// 			this._searchParams.delete = function (name) {
// 				this._delete(name);
// 				this._url.search = this.toString();
// 			};
// 			this._searchParams._set = this._searchParams.set;
// 			this._searchParams.set = function (name, value) {
// 				this._set(name, value);
// 				this._url.search = this.toString();
// 			};
// 			this._searchParams._sort = this._searchParams.sort;
// 			this._searchParams.sort = function () {
// 				this._sort();
// 				this._url.search = this.toString();
// 			};
// 		}
// 		return this._searchParams;
// 	},
// });
