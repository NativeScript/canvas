let LAF = 0;

export function rainbowOctopus(canvas) {


  var ctx = canvas.getContext('2d') /* canvas context */,
    w /* canvas height */, h /* canvas height */,

    t = 0,

    max = Math.max,
    pow = Math.pow, sqrt = Math.sqrt,
    PI = Math.PI,
    sin = Math.sin, cos = Math.cos /* just me being lazy */,

    /* spiral vars */
    /* https://twitter.com/jackrugile/status/420459385505079296 */
    n = 8 /* shades */,
    m = 4 /* shade repetitions */,
    p = 32 /* dots on each branch */,
    r,
    beta /* branch specific */, gamma /* dot specific */,
    x0, y0, x1, y1,
    hue,
    t_step = 1 / 60,
    requestID;

  /* FUNCTIONS */
  var trimUnit = function (input_str, unit) {
    return parseInt(input_str.split(unit)[0], 10);
  };

  var spiral = function () {
    ctx.clearRect(0, 0, w, h);

    for (var i = 0; i < n * m; i++) {
      beta = i * 2 * PI / (n * m);
      x0 = 0;

      /* Begin the path up here */
      ctx.beginPath();
      hue = i * 360 / n;
      ctx.translate(w / 2, h / 2);
      ctx.rotate(t / 3);
      /* only need to set the fillstyle once up here now */
      ctx.fillStyle = 'hsl(' + hue + ', 100%, 65%)';

      for (var j = 0; j < p; j++) {
        gamma = j * 2 * PI / p;
        r = max(1, pow(2 * (j * (p - j)), .43) - 10);

        x0 += 3.4 * r;
        y0 = x0 * sin(gamma + 2 * t + x0 / 99) / 5;

        /* change of coordinates */
        x1 = x0 * cos(beta) - y0 * sin(beta);
        y1 = x0 * sin(beta) + y0 * cos(beta);

        /* move it to the position of the arc */
        /* (remove this for a cool effect) */
        ctx.moveTo(x1, y1);
        /* setup the arc path here */
        ctx.arc(x1, y1, r, 0, 2 * PI);
      }

      /* close the 1 path that now is a combination of all the arcs */
      ctx.closePath();
      /* fill the whole path only once now */
      ctx.fill();
      /*
       * reason for moving the fill out of the inner loop:
       * see https://twitter.com/loktar00/status/420369245378076672
       * (thanks!)
       */
      ctx.rotate(-t / 3);
      ctx.translate(-w / 2, -h / 2);
    }

    t += t_step;

    requestID = requestAnimationFrame(spiral)
    LAF = requestID;
  };

  var initCanvas = function () {
    var s /* canvas style set via CSS */;

    setTimeout(function () {
      w = canvas.getMeasuredWidth();
      h = canvas.getMeasuredHeight();

      /* if resizing, make sure to stop the previous animation
       * before starting a new one */
      /* cancelAnimationFrame(requestID) should be
       * the requestID returned by requestAnimationFrame
       * thanks @FWeinb! */
      if (requestID) {
        cancelAnimationFrame(requestID);
      }
      spiral();
    }, 0);
  };

  /* STEPS */
  initCanvas();


}


export function cancelRainbowOctopus() {
  cancelAnimationFrame(LAF);
  LAF = 0;
}
