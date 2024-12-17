import typescript from '@rollup/plugin-typescript';
import commonjs from '@rollup/plugin-commonjs';
const config = [
	{
		input: 'index.ts',
		plugins: [typescript(), commonjs()],
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
		plugins: [typescript(), commonjs()],
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
