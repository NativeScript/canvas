const webpack = require('@nativescript/webpack');
const { resolve } = require('path');

module.exports = (env) => {

  webpack.init(env);
  webpack.useConfig('typescript');

  webpack.chainWebpack((config) => {
    // shared demo code
    config.resolve.alias.set('@demo/shared', resolve(__dirname, '..', '..', 'tools', 'demo'));
  });

  webpack.Utils.addCopyRule('**/*.svg')
  webpack.Utils.addCopyRule('**/*.mp4')
  webpack.Utils.addCopyRule('**/*.so')
  webpack.Utils.addCopyRule('**/*.wgsl')

  webpack.Utils.addCopyRule({
    from: '../../../tools/demo/canvas/webgpu/shaders', 
		to: 'webgpu/shaders',
    context: webpack.Utils.project.getProjectFilePath('node_modules')
  });


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

  webpack.Utils.addCopyRule({
    from: '../../../tools/demo/canvas-three/x-jet/hdr512', 
		to: 'assets/x-jet/hdr512',
    context: webpack.Utils.project.getProjectFilePath('node_modules')
  });

  webpack.Utils.addCopyRule({
    from: '../../../tools/demo/canvas-three/x-jet/Drone.glb', 
		to: 'assets/x-jet',
    context: webpack.Utils.project.getProjectFilePath('node_modules')
  });


  webpack.Utils.addCopyRule({
    from: '../../../tools/demo/canvas-three/x-jet/TriniShip.glb', 
		to: 'assets/x-jet',
    context: webpack.Utils.project.getProjectFilePath('node_modules')
  });

  webpack.Utils.addCopyRule({
    from: '../../../tools/demo/canvas-three/x-jet/explosion.png', 
		to: 'assets/x-jet',
    context: webpack.Utils.project.getProjectFilePath('node_modules')
  });


  webpack.Utils.addCopyRule({
    from: '../../../tools/demo/canvas-three/x-jet/waternormals.jpg', 
		to: 'assets/x-jet',
    context: webpack.Utils.project.getProjectFilePath('node_modules')
  });




  return webpack.resolveConfig();
};
