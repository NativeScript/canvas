import { test } from 'node:test';
import assert from 'node:assert/strict';
import { fileURLToPath } from 'node:url';
import { build } from 'esbuild';
import { DOMParser } from '@xmldom/xmldom';

// As undom-ng installs under DOMiNATIVE (solid-js): a document whose elements have no
// getElementsByTagName, which query-selector's import-time probe calls.
function installForeignDocument() {
	const foreign = {
		createElement: (tag) => ({ nodeName: tag.toUpperCase(), childNodes: [], appendChild(node) { this.childNodes.push(node); return node; } }),
		createComment: (data) => ({ nodeType: 8, data }),
	};
	globalThis.document = foreign;
	return foreign;
}

const packageDir = fileURLToPath(new URL('..', import.meta.url));

/** `source` bundled the way an app bundler would, from this package. */
async function load(source) {
	const { outputFiles } = await build({
		stdin: { contents: source, resolveDir: packageDir, loader: 'ts' },
		bundle: true,
		format: 'esm',
		write: false,
		platform: 'neutral',
		mainFields: ['module', 'main'],
	});
	return import('data:text/javascript;base64,' + Buffer.from(outputFiles[0].text).toString('base64') + '#' + Math.random());
}

const svg = new DOMParser().parseFromString('<svg xmlns="http://www.w3.org/2000/svg"><defs><linearGradient id="g"/></defs><rect class="a"/><rect/></svg>', 'image/svg+xml');

test('query-selector on its own fails beside a foreign document', async () => {
	installForeignDocument();
	await assert.rejects(load("export { default } from 'query-selector';"), /getElementsByTagName/);
	delete globalThis.document;
});

test('the polyfill loads query-selector beside a foreign document and puts it back', async () => {
	const foreign = installForeignDocument();
	const { default: querySelector } = await load("export { default } from './DOM/querySelector';");
	assert.equal(globalThis.document, foreign);
	assert.deepEqual(querySelector('#g', svg.documentElement).map((node) => node.nodeName), ['linearGradient']);
	assert.equal(querySelector('rect', svg.documentElement).length, 2);
	assert.equal(querySelector('rect.a', svg.documentElement).length, 1);
	delete globalThis.document;
});

test('without a document nothing is installed', async () => {
	delete globalThis.document;
	await load("export { default } from './DOM/querySelector';");
	assert.equal('document' in globalThis, false);
});
