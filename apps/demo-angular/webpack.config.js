const webpack = require('@nativescript/webpack');
const { resolve } = require('path');

module.exports = (env) => {

  webpack.init(env);
  webpack.useConfig('angular');

  webpack.chainWebpack((config) => {
    // shared demo code
    config.resolve.alias.set('@demo/shared', resolve(__dirname, '..', '..', 'tools', 'demo'));
  });

  webpack.Utils.addCopyRule('**/*.svg')
  webpack.Utils.addCopyRule('**/*.mp4')

  webpack.Utils.addCopyRule({
    from: '../../../tools/demo/canvas/assets', 
		to: 'assets/file-assets',
    context: webpack.Utils.project.getProjectFilePath('node_modules')
  });

  webpack.Utils.addCopyRule({
    from: '../../../tools/demo/canvas-babylon/assets', 
		to: 'assets/babylon',
    context: webpack.Utils.project.getProjectFilePath('node_modules')
  });

  webpack.Utils.addCopyRule({
    from: '../../../tools/demo/canvas-phaser/assets', 
		to: 'assets/phaser',
    context: webpack.Utils.project.getProjectFilePath('node_modules')
  });

  webpack.Utils.addCopyRule({
    from: '../../../tools/demo/canvas-phaser-ce/assets', 
		to: 'assets/phaser-ce',
    context: webpack.Utils.project.getProjectFilePath('node_modules')
  });

  webpack.Utils.addCopyRule({
    from: '../../../tools/demo/canvas-pixi/assets', 
		to: 'assets/pixi',
    context: webpack.Utils.project.getProjectFilePath('node_modules')
  });

  webpack.Utils.addCopyRule({
    from: '../../../tools/demo/canvas-three/assets', 
		to: 'assets/three',
    context: webpack.Utils.project.getProjectFilePath('node_modules')
  });

  return webpack.resolveConfig();
};
