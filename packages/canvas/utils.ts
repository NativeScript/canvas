export class Utils {
	static _SUPPORTED_TYPED_ARRAYS_VERSION = '8.2.0';
	static _IS_SUPPORTED_VERSION = false;
	static _CHECKED_FOR_SUPPORT = false;
	public static toJSArray(array) {
		if (global.isIOS) {
			if (array instanceof NSArray) {
				const jsArray = [];
				const count = array.count;
				for (let i = 0; i < count; i++) {
					jsArray.push(array.objectAtIndex(i));
				}
				return jsArray;
			}
		}

		if (global.isAndroid) {
			const jsArray = [];
			if (array instanceof java.util.ArrayList) {
				const count = array.size();
				for (let i = 0; i < count; i++) {
					jsArray.push(array.get(i));
				}
				return jsArray;
			}

			if (array instanceof androidNative.Array) {
				const count = array.length;
				for (let i = 0; i < count; i++) {
					jsArray.push(array[i]);
				}
			}
		}

		if (!Array.isArray(array)) {
			return [];
		} else {
			return array;
		}
	}

	public static get IS_SUPPORTED_TYPED_ARRAYS_VERSION() {
		if (!this._CHECKED_FOR_SUPPORT) {
			const version = (<any>global).__runtimeVersion;
			this._IS_SUPPORTED_VERSION = version.substring(0, version.indexOf('-')) >= this._SUPPORTED_TYPED_ARRAYS_VERSION;
			this._CHECKED_FOR_SUPPORT = true;
		}
		return this._IS_SUPPORTED_VERSION;
	}

	static isTypedArray(value){
		return value instanceof Uint8ClampedArray || value instanceof Uint8Array || value instanceof Int8Array || value instanceof Uint16Array || value instanceof Int16Array || value instanceof Uint32Array || value instanceof Int32Array || value instanceof Float32Array
	}
}


export default function lazy<T>(action: () => T): () => T {
	let _value: T;

	return () => _value || (_value = action());
}
