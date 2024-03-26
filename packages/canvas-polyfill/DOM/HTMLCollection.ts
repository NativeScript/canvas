export class HTMLCollection extends Array {
	item(index: number) {
		return this[index];
	}

	namedItem(nameOrId: string) {
		// todo
		return null;
	}
}
