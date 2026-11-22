import { test } from 'node:test';
import assert from 'node:assert/strict';
import { fileURLToPath } from 'node:url';
import { build } from 'esbuild';

// The screen reports its defaults until the app window exists, then the window's size (as core does
// on Windows and Linux); orientationChanged is raised on every window resize there.
const fakeCore = `
export const Screen = { mainScreen: { widthDIPs: 1920, heightDIPs: 1080, scale: 1 } };
const listeners = [];
export const Application = {
	orientationChangedEvent: 'orientationChanged',
	on(name, fn) {
		if (name === 'orientationChanged') listeners.push(fn);
	},
	raise(newValue) {
		for (const fn of listeners) fn({ newValue });
	},
};
`;

async function loadPolyfill(define) {
	delete globalThis.__TNS_BROWSER_POLYFILL_RESIZE;
	delete globalThis.screen;
	globalThis.window = globalThis;
	const entry = fileURLToPath(new URL('../resize.ts', import.meta.url));
	const { outputFiles } = await build({
		stdin: {
			contents: `export * from ${JSON.stringify(entry)}; export { Screen, Application } from '@nativescript/core';`,
			resolveDir: fileURLToPath(new URL('.', import.meta.url)),
			loader: 'ts',
		},
		bundle: true,
		format: 'esm',
		write: false,
		platform: 'neutral',
		define,
		plugins: [
			{
				name: 'fake-core',
				setup(b) {
					b.onResolve({ filter: /^@nativescript\/core$/ }, () => ({ path: 'core', namespace: 'fake' }));
					b.onLoad({ filter: /.*/, namespace: 'fake' }, () => ({ contents: fakeCore, loader: 'js' }));
				},
			},
		],
	});
	return import('data:text/javascript;base64,' + Buffer.from(outputFiles[0].text).toString('base64') + '#' + Math.random());
}

test('desktop: innerWidth and innerHeight follow the window after a resize', async () => {
	const { Screen, Application } = await loadPolyfill({ __WINDOWS__: 'true' });
	Screen.mainScreen.widthDIPs = 1536;
	Screen.mainScreen.heightDIPs = 864;
	Application.raise('landscape');
	assert.equal(window.innerWidth, 1536);
	assert.equal(window.innerHeight, 864);
	assert.equal(window.clientWidth, 1536);
	assert.equal(window.clientHeight, 864);
});

test('desktop: a size the page assigned stays through a resize', async () => {
	const { Application } = await loadPolyfill({ __LINUX__: 'true' });
	window.innerWidth = 1136.8;
	window.innerHeight = 554.4;
	Application.raise('landscape');
	assert.equal(window.innerWidth, 1136.8);
	assert.equal(window.innerHeight, 554.4);
});

test('mobile: a rotation swaps the screen size', async () => {
	const { Application } = await loadPolyfill({});
	Application.raise('portrait');
	assert.equal(window.innerWidth, 1920);
	assert.equal(window.innerHeight, 1080);
	Application.raise('landscape');
	assert.equal(window.innerWidth, 1080);
	assert.equal(window.innerHeight, 1920);
	assert.equal(window.orientation, 90);
});
