import typescript from '@rollup/plugin-typescript';
import commonjs from '@rollup/plugin-commonjs';
import resolve from '@rollup/plugin-node-resolve';
import copy from 'rollup-plugin-copy';

const config = [
	{
		input: 'index.ts',
		plugins: [
			typescript(),
			commonjs({
				ignoreGlobal: true,
			}),
			copy({
				targets: [
					{ src: 'package.json', dest: 'dist' },
					{ src: 'js-bindings.js', dest: 'dist' },
					{ src: 'js-bindings.d.ts', dest: 'dist' },
				],
			}),
		],
		output: [
			{
				format: 'cjs',
				file: 'dist/index.cjs',
				indent: '\t',
			},
		],
		external: ['js-bindings.js', '@nativescript/foundation', '@nativescript/macos-node-api'],
	},
	{
		input: 'index.ts',
		plugins: [
			typescript(),
			commonjs({
				ignoreGlobal: true,
			}),
			copy({
				targets: [
					{ src: 'package.json', dest: 'dist' },
					{ src: 'js-bindings.js', dest: 'dist' },
					{ src: 'js-bindings.d.ts', dest: 'dist' },
				],
			}),
		],
		output: [
			{
				format: 'es',
				file: 'dist/index.mjs',
				indent: '\t',
			},
		],
		external: ['js-bindings.js', '@nativescript/foundation', '@nativescript/macos-node-api'],
	},
];

export default config;
