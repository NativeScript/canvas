import {Screen} from '@nativescript/core';

let LAF = 0;

export function particlesColor(canvas, width?, height?, nativeCanvas?) {
  var ctx = canvas.getContext ? canvas.getContext('2d') : canvas,
    particles = [],
    patriclesNum = 100,
    w = Screen.mainScreen.widthPixels,
    h = Screen.mainScreen.heightPixels,
    colors = ['#f35d4f', '#f36849', '#c0d988', '#6ddaf1', '#f1e85b'];
  // const w = width || canvas.getMeasuredWidth(),
  //     h = height || canvas.getMeasuredHeight();

  // canvas.style.left = (canvas.measuredWidth() - 500)/2+'px';

  //if(window.innerHeight>500)
  //   canvas.style.top = (window.innerHeight - 500)/2+'px';

  function Factory() {
    this.x = Math.round(Math.random() * w);
    this.y = Math.round(Math.random() * h);
    this.rad = Math.round(Math.random()) + 1;
    this.rgba = colors[Math.round(Math.random() * 3)];
    this.vx = Math.round(Math.random() * 3) - 1.5;
    this.vy = Math.round(Math.random() * 3) - 1.5;
  }

  function draw() {
    ctx.clearRect(0, 0, w, h);
    ctx.globalCompositeOperation = 'lighter';
    for (var i = 0; i < patriclesNum; i++) {
      var temp = particles[i];
      var factor = 1;

      for (var j = 0; j < patriclesNum; j++) {

        var temp2 = particles[j];
        ctx.linewidth = 0.5;

        if (temp.rgba == temp2.rgba && findDistance(temp, temp2) < 50) {
          ctx.strokeStyle = temp.rgba;
          ctx.beginPath();
          ctx.moveTo(temp.x, temp.y);
          ctx.lineTo(temp2.x, temp2.y);
          ctx.stroke();
          factor++;
        }
      }


      ctx.fillStyle = temp.rgba;
      ctx.strokeStyle = temp.rgba;

      ctx.beginPath();
      ctx.arc(temp.x, temp.y, temp.rad * factor, 0, Math.PI * 2, true);
      ctx.fill();
      ctx.closePath();

      ctx.beginPath();
      ctx.arc(temp.x, temp.y, (temp.rad + 5) * factor, 0, Math.PI * 2, true);
      ctx.stroke();
      ctx.closePath();


      temp.x += temp.vx;
      temp.y += temp.vy;

      if (temp.x > w) temp.x = 0;
      if (temp.x < 0) temp.x = w;
      if (temp.y > h) temp.y = 0;
      if (temp.y < 0) temp.y = h;
    }
  }

  function findDistance(p1, p2) {
    return Math.sqrt(Math.pow(p2.x - p1.x, 2) + Math.pow(p2.y - p1.y, 2));
  }

  var requestAnimFrame = requestAnimationFrame;

  (function init() {
    for (var i = 0; i < patriclesNum; i++) {
      particles.push(new Factory);
    }
  })();

  (function loop() {
    LAF = requestAnimFrame(loop);
    draw();
  })();

}

export function cancelParticlesColor() {
  cancelAnimationFrame(LAF);
  LAF = 0;
}
