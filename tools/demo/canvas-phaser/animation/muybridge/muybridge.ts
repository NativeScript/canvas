declare var Phaser;

export function setupGame(canvas) {
  (global as any).__debug_browser_polyfill_image = true;
  var game = require("@nativescript/canvas-phaser").Game({
    canvas,
    // height: 600,
    type: 2,
    backgroundColor: '#efefef',
    scene: {
      preload: preload,
      create: create
    }
  });
}

function preload() {
  this.load.spritesheet('muybridge', '~/assets/phaser/muybridge/muybridge01.png', {frameWidth: 119, frameHeight: 228});
}

function create() {
  var config = {
    key: 'run',
    frames: 'muybridge',
    frameRate: 15,
    repeat: -1
  };

  this.anims.create(config);

  //  Each frame is 119px wide

  const group = this.add.group();

  group.createMultiple({key: 'muybridge', frame: 0, repeat: 8});

  Phaser.Actions.GridAlign(group.getChildren(), {
    width: 9,
    height: 1,
    cellWidth: 119,
    y: 170
  });

  this.anims.staggerPlay('run', group.getChildren(), -100);
}




