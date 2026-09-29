// query-selector probes the global `document` for old IE as it is evaluated. Another DOM's document
// (undom-ng under DOMiNATIVE) fails that probe and the import throws, so ./querySelector hides it
// until query-selector has loaded. Nothing it does later relies on the probe.
const descriptor = Object.getOwnPropertyDescriptor(globalThis, 'document');
let hidden = false;

if (descriptor?.configurable) {
	delete (globalThis as any).document;
	hidden = true;
} else if (descriptor?.writable) {
	(globalThis as any).document = undefined;
	hidden = true;
}

export function restoreDocument() {
	if (!hidden) {
		return;
	}
	hidden = false;
	if (descriptor.configurable) {
		Object.defineProperty(globalThis, 'document', descriptor);
	} else {
		(globalThis as any).document = descriptor.value;
	}
}
