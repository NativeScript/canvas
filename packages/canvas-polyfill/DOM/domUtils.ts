import { eachDescendant } from '@nativescript/core';

function _visitRealizedItems(child, callback) {
	if (child._realizedItems && child._realizedItems.size !== child._childrenCount) {
		for (const key in child._realizedItems) {
			if (Object.prototype.hasOwnProperty.call(child._realizedItems, key)) {
				callback(child._realizedItems[key]);
			}
		}
	}
}

export function domGetElementsByClassName(root, clsName: string): any[] {
	const retVal: any[] = [];
	if (!root) {
		return retVal;
	}

	if (root.classList?.contains(clsName)) {
		retVal.push(root);
	}

	const visit = (child) => {
		if (child.classList?.contains(clsName)) {
			retVal.push(child);
		}
		_visitRealizedItems(child, visit);
		return true;
	};

	eachDescendant(root, visit);
	_visitRealizedItems(root, visit);

	return retVal;
}

export function domGetElementsByTagName(root, tagName: string): any[] {
	const tagNameLC = tagName.toLowerCase();
	const allTags = tagName === '*';
	const retVal: any[] = [];

	if (!root) {
		return retVal;
	}

	if ((root.typeName && root.typeName.toLowerCase() === tagNameLC) || allTags) {
		retVal.push(root);
	}

	const visit = (child) => {
		if ((child.typeName && child.typeName.toLowerCase() === tagNameLC) || allTags) {
			retVal.push(child);
		}
		_visitRealizedItems(child, visit);
		return true;
	};

	eachDescendant(root, visit);
	_visitRealizedItems(root, visit);

	return retVal;
}
