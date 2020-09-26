import {isIOS, isAndroid} from '@nativescript/core/platform';

export class Utils {
	public static toJSArray(array) {
		if (isIOS) {
			if (array instanceof NSArray) {
				const jsArray = [];
				const count = array.count;
				for (let i = 0; i < count; i++) {
					jsArray.push(array.objectAtIndex(i));
				}
				return jsArray;
			}
		}

		if (isAndroid) {
			const jsArray = [];
			if (array instanceof java.util.ArrayList) {
				const count = array.size();
				for (let i = 0; i < count; i++) {
					jsArray.push(array.get(i));
				}
				return jsArray;
			}

			if (array instanceof native.Array) {
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
}
