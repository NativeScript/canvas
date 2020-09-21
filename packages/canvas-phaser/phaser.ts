require('@nativescript/canvas-polyfill');

function initPhaser() {
/*  if ((global as any).Phaser) {
    const version = parseFloat((global as any).Phaser.VERSION);
    if (!Number.isNaN(version) && version < 3) {
      (global as any).PIXI = (global as any).window.PIXI = undefined;
      (global as any).p2 = (global as any).window.p2 = undefined;
      (global as any).window.Phaser = (global as any).Phaser = undefined;
    }
  }*/
  (global as any).window.Phaser = (global as any).Phaser = (global as any).Phaser || require('phaser');
  return (global as any).Phaser;
}

export default initPhaser();
