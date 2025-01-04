import typescript from '@rollup/plugin-typescript';
import commonjs from '@rollup/plugin-commonjs';
import copy from 'rollup-plugin-copy';
const config = [
	{
		input: 'index.ts',
		plugins: [
			typescript(),
			commonjs(),
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
	},
	{
		input: 'index.ts',
		plugins: [
			typescript(),
			commonjs(),
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
	},
];

export default config;
