const webpack = require('@nativescript/webpack');
const { resolve } = require('path');

module.exports = (env) => {
	webpack.init(env);
	webpack.useConfig('typescript');

	webpack.chainWebpack((config) => {
		// packages/ ship every platform's sources; they were never type-checked together.
		config.plugins.delete('ForkTsCheckerWebpackPlugin');
		// The plugins are linked from packages/: keep one @nativescript/core, the app's.
		config.resolve.alias.set('@nativescript/core', resolve(__dirname, 'node_modules', '@nativescript', 'core'));
	});

	return webpack.resolveConfig();
};
