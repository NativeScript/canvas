import {ImageSource} from '@nativescript/core';

let LAF = 0;

export function solarSystem(canvas) {
  const ctx = canvas.getContext('2d');

  let sun;
  let moon;
  let earth;

  async function init() {
    sun = await ImageSource.fromUrl('https://mdn.mozillademos.org/files/1456/Canvas_sun.png');
    moon = await ImageSource.fromUrl('https://mdn.mozillademos.org/files/1443/Canvas_moon.png');
    earth = await ImageSource.fromUrl('https://mdn.mozillademos.org/files/1429/Canvas_earth.png');
    LAF = requestAnimationFrame(draw);
  }

  function draw() {
    ctx.globalCompositeOperation = 'destination-over';
    ctx.clearRect(0, 0, canvas.getMeasuredWidth(), canvas.getMeasuredHeight());

    ctx.fillStyle = 'rgba(0, 0, 0, 0.4)';
    ctx.strokeStyle = 'rgba(0, 153, 255, 0.4)';
    ctx.save();
    ctx.translate(150, 150);

    // Earth
    let time = new Date();
    ctx.rotate(
      ((2 * Math.PI) / 60) * time.getSeconds() +
      ((2 * Math.PI) / 60000) * time.getMilliseconds()
    );
    ctx.translate(105, 0);
    ctx.fillRect(0, -12, 40, 24);    // Shadow
    ctx.drawImage(earth, -12, -12);

    // Moon
    ctx.save();
    ctx.rotate(
      ((2 * Math.PI) / 6) * time.getSeconds() +
      ((2 * Math.PI) / 6000) * time.getMilliseconds()
    );
    ctx.translate(0, 28.5);
    ctx.drawImage(moon, -3.5, -3.5);

    ctx.restore();
    ctx.restore();

    // Earth orbit arc
    ctx.beginPath();
    ctx.arc(150, 150, 105, 0, Math.PI * 2, false);
    ctx.stroke();

    //
    ctx.drawImage(sun, 0, 0, 300, 300);

    LAF = requestAnimationFrame(draw);
  }

  init();
}

export function cancelSolarSystem() {
  cancelAnimationFrame(LAF);
  LAF = 0;
}
