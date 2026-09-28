import { EventData, Page, WebView } from '@nativescript/core';
import { launchArgs } from '../launch-args';

export function navigatingTo(args: EventData) {
	// no binding context needed
}

/**
 * Runs the *original* swarm workload (1500 particles, O(n^2) neighbour search, one
 * beginPath/strokeStyle/stroke per in-range pair) against the system WebView's own
 * canvas 2D, on the same device and GPU as the NativeScript canvas.
 *
 * This is the control for "it was fine on the web": it isolates our canvas
 * implementation from the device, because everything else about the workload is
 * identical. The result is painted into the page so it can be read off a
 * screenshot -- no bridge needed.
 */
const PAGE = `
<!DOCTYPE html>
<html>
<head><meta name="viewport" content="width=device-width,initial-scale=1,user-scalable=no"></head>
<body style="margin:0;background:#000;overflow:hidden">
<canvas id="c" style="width:100vw;height:100vh;display:block"></canvas>
<div id="out" style="position:fixed;left:0;right:0;top:0;color:#0f0;font:16px monospace;background:rgba(0,0,0,.75);padding:8px;white-space:pre"></div>
<script>
(function () {
  var canvas = document.getElementById('c');
  var out = document.getElementById('out');
  var dpr = window.devicePixelRatio || 1;
  var W = canvas.clientWidth * dpr;
  var H = canvas.clientHeight * dpr;
  canvas.width = W;
  canvas.height = H;

  var ctx = canvas.getContext('2d');
  ctx.scale(dpr, dpr);

  var particleCount = 1500, particles = [], minDist = 50, dist;

  function paintCanvas() {
    ctx.fillStyle = 'black';
    ctx.fillRect(0, 0, W, H);
  }

  function Particle() {
    this.x = Math.random() * W;
    this.y = Math.random() * H;
    this.vx = -1 + Math.random() * 2;
    this.vy = -1 + Math.random() * 2;
    this.radius = 4;
    this.draw = function () {
      ctx.fillStyle = 'white';
      ctx.beginPath();
      ctx.arc(this.x, this.y, this.radius, 0, Math.PI * 2, false);
      ctx.fill();
    };
  }

  for (var i = 0; i < particleCount; i++) particles.push(new Particle());

  var p = new Particle();
  var calls = 0;

  function draw() {
    paintCanvas();
    for (var i = 0; i < particles.length; i++) { p = particles[i]; p.draw(); }
    update();
  }

  function update() {
    for (var i = 0; i < particles.length; i++) {
      p = particles[i];
      p.x += p.vx; p.y += p.vy;
      if (p.x + p.radius > W) p.x = p.radius;
      else if (p.x - p.radius < 0) p.x = W - p.radius;
      if (p.y + p.radius > H) p.y = p.radius;
      else if (p.y - p.radius < 0) p.y = H - p.radius;
      for (var j = i + 1; j < particles.length; j++) distance(p, particles[j]);
    }
  }

  function distance(p1, p2) {
    var dist, dx = p1.x - p2.x, dy = p1.y - p2.y;
    dist = Math.sqrt(dx * dx + dy * dy);
    if (dist <= minDist) {
      ctx.beginPath();
      ctx.strokeStyle = 'rgba(255,255,255,' + (1.2 - dist / minDist) + ')';
      ctx.moveTo(p1.x, p1.y);
      ctx.lineTo(p2.x, p2.y);
      ctx.stroke();
      ctx.closePath();
      calls += 6;
      var ax = dx / 2000, ay = dy / 2000;
      p1.vx -= ax; p1.vy -= ay;
      p2.vx += ax; p2.vy += ay;
    }
  }

  var durations = [];
  var TARGET = 180;

  function loop() {
    if (durations.length >= TARGET) { report(); return; }
    requestAnimationFrame(loop);
    calls = 0;
    var s = performance.now();
    draw();
    durations.push(performance.now() - s);
  }

  function report() {
    var steady = durations.slice(20).sort(function (a, b) { return a - b; });
    var med = steady[Math.round(0.5 * (steady.length - 1))];
    var p95 = steady[Math.round(0.95 * (steady.length - 1))];
    out.textContent =
      'WEBVIEW BASELINE (original algorithm)\\n' +
      'canvas ' + W + 'x' + H + ' dpr ' + dpr + '\\n' +
      'frames ' + durations.length + '\\n' +
      'median ' + med.toFixed(2) + ' ms\\n' +
      'p95    ' + p95.toFixed(2) + ' ms\\n' +
      'lineCalls/frame ~' + calls;
    document.title = 'median=' + med.toFixed(2);
  }

  requestAnimationFrame(loop);
})();
</script>
</body>
</html>
`;

export function webLoaded(args) {
	const view = <WebView>args.object;

	// `--es suite starwarp` runs the PixiJS star-warp example against the WebView's
	// own WebGL/WebGPU instead, as the control for the NativeScript canvas-pixi run:
	//
	//   adb shell am start -n org.nativescript.plugindemo/com.tns.NativeScriptActivity \
	//     --es demo canvas-webview-compare --es suite starwarp
	//   adb logcat -d | grep 'WVFPS|'
	//
	// It loads from the app folder rather than an HTML string so the page can pull
	// in pixi.min.js and the same star.png the native demo uses.
	if (launchArgs.suite === 'starwarp') {
		view.src = '~/assets/pixi/webview/starwarp.html';
		return;
	}

	// `--es suite callbound` is the call-cost twin: cheap ops, almost no pixels.
	if (launchArgs.suite === 'callbound') {
		view.src = '~/assets/pixi/webview/callbound.html';
		return;
	}

	view.src = PAGE;
}
