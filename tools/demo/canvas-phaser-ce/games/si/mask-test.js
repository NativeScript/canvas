import {Game} from '@nativescript/canvas-phaser-ce';
export function maskTest(canvas){

    let game;
    function preload() {
        game.load.image('pic', '~/examples/core-example/assets/questar.png');
        game.load.image('mask','~/examples/core-example/assets/mask-test2.png');
    }

    function create() {
       game.stage.backgroundColor =  0x4d4d4d;

        game.add.text(64, 10, 'Source image', { font: '16px Arial', fill: '#ffffff' })
        game.add.image(64, 32, 'pic');

        game.add.text(400, 10, 'Alpha mask', { font: '16px Arial', fill: '#ffffff' })
        game.add.image(400, 32, 'mask');

        //	Create a new bitmap data the same size as our picture
        var bmd = game.make.bitmapData(320, 256);

        //	And create an alpha mask image by combining pic and mask from the cache
        bmd.alphaMask('pic', 'mask');

        //	A BitmapData is just a texture. You need to apply it to a sprite or image
        //	to actually display it:
        game.add.image(game.world.centerX, 320, bmd).anchor.set(0.5, 0);

    }

    game = Game({ canvas, renderer: 1 ,state: { preload, create }});
}
