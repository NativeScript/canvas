import { ImageSource } from '@nativescript/core';
import { Canvas, ImageAsset } from '@nativescript/canvas';
import { Screen } from '@nativescript/core';
import { doesNotReject } from 'assert';
export function fillStyle(canvas) {
	const ctx = canvas.getContext('2d');
	for (let i = 0; i < 6; i++) {
		for (let j = 0; j < 6; j++) {
			ctx.fillStyle = `rgb(
        ${Math.floor(255 - 42.5 * i)},
        ${Math.floor(255 - 42.5 * j)},
        0)`;
			ctx.fillRect(j * 25, i * 25, 25, 25);
		}
	}
}


export function font(canvas) {
	const ctx = canvas.getContext('2d');
	ctx.font = 'bold 48px serif';
	ctx.strokeText('Hello world', 50, 100);
}


export function globalAlpha(canvas) {
	const ctx = canvas.getContext('2d');
	ctx.globalAlpha = 0.5;

	ctx.fillStyle = 'blue';
	ctx.fillRect(10, 10, 100, 100);

	ctx.fillStyle = 'red';
	ctx.fillRect(50, 50, 100, 100);
}

export function globalCompositeOperation(canvas) {
	const ctx = canvas.getContext('2d');
	ctx.globalCompositeOperation = 'xor';

	ctx.fillStyle = 'blue';
	ctx.fillRect(10, 10, 100, 100);

	ctx.fillStyle = 'red';
	ctx.fillRect(50, 50, 100, 100);
}

export function imageSmoothingEnabled(canvas) {
	const ctx = canvas.getContext('2d');
	ctx.font = '16px sans-serif';
	ctx.textAlign = 'center';
	const src = ImageSource.fromUrl('https://interactive-examples.mdn.mozilla.net/media/examples/star.png');
	src.then(img => {
		const w = img.width,
			h = img.height;

		ctx.fillText('Source', w * .5, 20);
		ctx.drawImage(img, 0, 24, w, h);

		ctx.fillText('Smoothing = TRUE', w * 2.5, 20);
		ctx.imageSmoothingEnabled = true;
		ctx.drawImage(img, w, 24, w * 3, h * 3);

		ctx.fillText('Smoothing = FALSE', w * 5.5, 20);
		ctx.imageSmoothingEnabled = false;
		ctx.drawImage(img, w * 4, 24, w * 3, h * 3);
	});
}

export function imageSmoothingQuality(canvas) {
	const ctx = canvas.getContext('2d');
	ImageSource.fromUrl('https://mdn.mozillademos.org/files/222/Canvas_createpattern.png')
		.then(function (img) {
			ctx.imageSmoothingQuality = 'low';
			ctx.drawImage(img.ios, 0, 0, 300, 150);
		});
}

export function imageBlock(canvas) {
	const ctx = canvas.getContext('2d');
	ctx.save();
	const asset = new ImageAsset();
	asset.loadFromUrlAsync('https://mdn.mozillademos.org/files/5397/rhino.jpg')
		.then(done => {
			for (var i = 0; i < 4; i++) {
				for (var j = 0; j < 3; j++) {
					ctx.drawImage(asset, (j * 50) * Screen.mainScreen.scale, (i * 38) * Screen.mainScreen.scale, 50 * Screen.mainScreen.scale, 38 * Screen.mainScreen.scale);
				}
			}
		});
}

export function lineCap(canvas) {
	const ctx = canvas.getContext('2d');
	ctx.beginPath();
	ctx.moveTo(20, 20);
	ctx.lineWidth = 15;
	ctx.lineCap = 'round';
	ctx.lineTo(100, 100);
	ctx.stroke();
}


export function lineDashOffset(canvas) {
	const ctx = canvas.getContext('2d');
	ctx.setLineDash([4, 16]);

	// Dashed line with no offset
	ctx.beginPath();
	ctx.moveTo(0, 50);
	ctx.lineTo(300, 50);
	ctx.stroke();

	// Dashed line with offset of 4
	ctx.beginPath();
	ctx.strokeStyle = 'red';
	ctx.lineDashOffset = 4;
	ctx.moveTo(0, 100);
	ctx.lineTo(300, 100);
	ctx.stroke();

}


export function lineJoin(canvas) {
	const ctx = canvas.getContext('2d');
	ctx.lineWidth = 20;
	ctx.lineJoin = 'round';
	ctx.beginPath();
	ctx.moveTo(20, 20);
	ctx.lineTo(190, 100);
	ctx.lineTo(280, 20);
	ctx.lineTo(280, 150);
	ctx.stroke();
}

export function lineWidth(canvas) {
	const ctx = canvas.getContext('2d');
	ctx.lineWidth = 15;

	ctx.beginPath();
	ctx.moveTo(20, 20);
	ctx.lineTo(130, 130);
	ctx.rect(40, 40, 70, 70);
	ctx.stroke();
}

export function shadowBlur(canvas) {
	const ctx = canvas.getContext('2d');
	ctx.shadowBlur = 3;
	ctx.shadowColor = 'rgba(0, 0, 0, 0.5)';
	ctx.shadowOffsetX = 3;
	ctx.shadowOffsetY = 6;

	ctx.beginPath();
	ctx.arc(50, 50, 30, 0, 2 * Math.PI);
	ctx.stroke();

	ctx.fillStyle = 'red';
	ctx.fillRect(100, 20, 30, 30);
}

export function miterLimit(canvas) {
	const ctx = canvas.getContext('2d');
	// Clear canvas
	ctx.clearRect(0, 0, 150, 150);

	// Draw guides
	ctx.strokeStyle = '#09f';
	ctx.lineWidth = 2;
	ctx.strokeRect(-5, 50, 160, 50);

	// Set line styles
	ctx.strokeStyle = '#000';
	ctx.lineWidth = 10;
	ctx.miterLimit = 10;

	// Draw lines
	ctx.beginPath();
	ctx.moveTo(0, 100);
	for (let i = 0; i < 24; i++) {
		var dy = i % 2 == 0 ? 25 : -25;
		ctx.lineTo(Math.pow(i, 1.5) * 2, 75 + dy);
	}
	ctx.stroke();
}

export function filterBlur(canvas){
const ctx = canvas.getContext('2d');

ctx.filter = 'blur(4px)';
ctx.font = '48px serif';
ctx.fillText('Hello world', 50, 100);
}


export function shadowColor(canvas) {
	const ctx = canvas.getContext('2d');
	// Shadow
	ctx.shadowColor = 'red';
	ctx.shadowOffsetX = 10;
	ctx.shadowOffsetY = 10;

	// remove after release
	ctx.shadowBlur = 1;

	// Filled rectangle
	ctx.fillRect(20, 20, 100, 100);

	// Stroked rectangle
	ctx.lineWidth = 6;
	ctx.strokeRect(170, 20, 100, 100);
}

export function shadowOffsetX(canvas) {
	const ctx = canvas.getContext('2d');
	// Shadow
	ctx.shadowColor = 'red';
	ctx.shadowOffsetX = 25;
	ctx.shadowBlur = 10;

	// Rectangle
	ctx.fillStyle = 'blue';
	ctx.fillRect(20, 20, 150, 100);
}


export function shadowOffsetY(canvas) {
	const ctx = canvas.getContext('2d');
	// Shadow
	ctx.shadowColor = 'red';
	ctx.shadowOffsetY = 25;
	ctx.shadowBlur = 10;

	// Rectangle
	ctx.fillStyle = 'blue';
	ctx.fillRect(20, 20, 150, 80);
}

export function strokeStyle(canvas) {
	const ctx = canvas.getContext('2d');
	ctx.strokeStyle = 'blue';
	ctx.strokeRect(10, 10, 100, 100);
}

export function multiStrokeStyle(canvas) {
	const ctx = canvas.getContext('2d');
	for (let i = 0; i < 6; i++) {
		for (let j = 0; j < 6; j++) {
			ctx.strokeStyle = `rgb(
        0,
        ${Math.floor(255 - 42.5 * i)},
        ${Math.floor(255 - 42.5 * j)})`;
			ctx.beginPath();
			ctx.arc(12.5 + j * 25, 12.5 + i * 25, 10, 0, Math.PI * 2, true);
			ctx.stroke();
		}
	}
}

export function textAlign(canvas) {
	const ctx = canvas.getContext('2d');
	canvas.width = 350;
	const x = canvas.width / 2;

	ctx.beginPath();
	ctx.moveTo(x, 0);
	ctx.lineTo(x, canvas.height);
	ctx.stroke();

	ctx.font = '30px serif';

	ctx.textAlign = 'left';
	ctx.fillText('left-aligned', x, 40);

	ctx.textAlign = 'center';
	ctx.fillText('center-aligned', x, 85);

	ctx.textAlign = 'right';
	ctx.fillText('right-aligned', x, 130);
}

export function arc(canvas) {
	const ctx = canvas.getContext('2d');
	ctx.beginPath();
	ctx.arc(100, 75, 50, 0, 2 * Math.PI);
	ctx.stroke();
}

export function arcMultiple(canvas) {
	const ctx = canvas.getContext('2d');
	// Draw shapes
	for (let i = 0; i <= 3; i++) {
		for (let j = 0; j <= 2; j++) {
			ctx.beginPath();
			let x = 25 + j * 50;                 // x coordinate
			let y = 25 + i * 50;                 // y coordinate
			let radius = 20;                          // Arc radius
			let startAngle = 0;                           // Starting point on circle
			let endAngle = Math.PI + (Math.PI * j) / 2; // End point on circle
			let anticlockwise = i % 2 == 1;                  // Draw anticlockwise

			ctx.arc(x, y, radius, startAngle, endAngle, anticlockwise);

			if (i > 1) {
				ctx.fill();
			} else {
				ctx.stroke();
			}
		}
	}
}

export function arcTo(canvas) {
	const ctx = canvas.getContext('2d');
	// Tangential lines
	ctx.beginPath();
	ctx.strokeStyle = 'gray';
	ctx.moveTo(200, 20);
	ctx.lineTo(200, 130);
	ctx.lineTo(50, 20);
	ctx.stroke();

	// Arc
	ctx.beginPath();
	ctx.strokeStyle = 'black';
	ctx.lineWidth = 5;
	ctx.moveTo(200, 20);
	ctx.arcTo(200, 130, 50, 20, 40);
	ctx.stroke();

	// Start point
	ctx.beginPath();
	ctx.fillStyle = 'blue';
	ctx.arc(200, 20, 5, 0, 2 * Math.PI);
	ctx.fill();

	// Control points
	ctx.beginPath();
	ctx.fillStyle = 'red';
	ctx.arc(200, 130, 5, 0, 2 * Math.PI); // Control point one
	ctx.arc(50, 20, 5, 0, 2 * Math.PI);   // Control point two
	ctx.fill();
}

export function arcToAnimation(canvas) {
	const ctx = canvas.getContext('2d');
	ctx.scale(3, 3);
	const mouse = { x: 0, y: 0 };

	let r = 100; // Radius
	const p0 = { x: 0, y: 50 };

	const p1 = { x: 100, y: 100 };
	const p2 = { x: 150, y: 50 };
	const p3 = { x: 200, y: 100 };

	const labelPoint = function (p, offset, i = 0) {
		const { x, y } = offset;
		ctx.beginPath();
		ctx.arc(p.x, p.y, 2, 0, Math.PI * 2);
		ctx.fill();
		ctx.fillText(`${i}:(${p.x}, ${p.y})`, p.x + x, p.y + y);
	};

	const drawPoints = function (points) {
		for (let i = 0; i < points.length; i++) {
			var p = points[i];
			labelPoint(p, { x: 0, y: -20 }, i);
		}
	};

	// Draw arc
	const drawArc = function ([p0, p1, p2], r) {
		ctx.beginPath();
		ctx.moveTo(p0.x, p0.y);
		ctx.arcTo(p1.x, p1.y, p2.x, p2.y, r);
		ctx.lineTo(p2.x, p2.y);
		ctx.stroke();
	};


	let t0 = 0;
	let rr = 0; // the radius that changes over time
	let a = 0; // angle
	let PI2 = Math.PI * 2;
	const loop = function (t) {
		t0 = t / 1000;
		a = t0 % PI2;
		rr = Math.abs(Math.cos(a) * r);

		ctx.clearRect(0, 0, canvas.width, canvas.height);

		drawArc([p1, p2, p3], rr);
		drawPoints([p1, p2, p3]);
		requestAnimationFrame(loop);
	};

	loop(0);
}


export function ellipse(canvas) {
	const ctx = canvas.getContext('2d');
	// Draw the ellipse
	ctx.beginPath();
	ctx.ellipse(100, 100, 50, 75, Math.PI / 4, 0, 2 * Math.PI);
	ctx.stroke();

	// Draw the ellipse's line of reflection
	ctx.beginPath();
	ctx.setLineDash([5, 5]);
	ctx.moveTo(0, 200);
	ctx.lineTo(200, 0);
	ctx.stroke();
}

export function fill(ctx) {

}

export function fillPath(canvas) {
	const context = canvas.getContext('2d');
	const ctx = global.__getCanvasRenderingContext2DImpl(String(context.canvas.nativeView.getNativeContext()));


	// Create path
	let region = new Path2D();
	region.moveTo(30, 90);
	region.lineTo(110, 20);
	region.lineTo(240, 130);
	region.lineTo(60, 130);
	region.lineTo(190, 20);
	region.lineTo(270, 90);
	region.closePath();

	// Fill path
	ctx.fillStyle = 'green';
	ctx.fill(region, 'evenodd');
}

export function createLinearGradient(canvas) {
	const ctx = canvas.getContext('2d');
	// Create a linear gradient
	// The start gradient point is at x=20, y=0
	// The end gradient point is at x=220, y=0
	var gradient = ctx.createLinearGradient(20, 0, 220, 0);

	// Add three color stops
	gradient.addColorStop(0, 'green');
	gradient.addColorStop(.5, 'cyan');
	gradient.addColorStop(1, 'green');

	// Set the fill style and draw a rectangle
	ctx.fillStyle = gradient;
	ctx.fillRect(20, 20, 200, 100);
}

export function createRadialGradient(canvas) {
	const ctx = canvas.getContext('2d');
	// Create a radial gradient
	// The inner circle is at x=110, y=90, with radius=30
	// The outer circle is at x=100, y=100, with radius=70
	var gradient = ctx.createRadialGradient(110, 90, 30, 100, 100, 70);

	// Add three color stops
	gradient.addColorStop(0, 'pink');
	gradient.addColorStop(.9, 'white');
	gradient.addColorStop(1, 'green');

	// Set the fill style and draw a rectangle
	ctx.fillStyle = gradient;
	ctx.fillRect(20, 20, 160, 160);
}

export function fillRule(canvas) {
	const ctx = canvas.getContext('2d');

	// Create path
	let region = new Path2D();
	region.moveTo(30, 90);
	region.lineTo(110, 20);
	region.lineTo(240, 130);
	region.lineTo(60, 130);
	region.lineTo(190, 20);
	region.lineTo(270, 90);
	region.closePath();

	// Fill path
	ctx.fillStyle = 'green';
	ctx.fill(region, 'evenodd');
}

export function scale(canvas) {
	const ctx = canvas.getContext('2d');
	// Scaled rectangle
	ctx.scale(9, 3);
	ctx.fillStyle = 'red';
	ctx.fillRect(10, 10, 8, 20);

	// Reset current transformation matrix to the identity matrix
	ctx.setTransform(1, 0, 0, 1, 0, 0);

	// Non-scaled rectangle
	ctx.fillStyle = 'gray';
	ctx.fillRect(10, 10, 8, 20);
}

export function pattern(canvas) {
	const ctx = canvas.getContext('2d');
	ImageSource.fromUrl('https://mdn.mozillademos.org/files/222/Canvas_createpattern.png')
		.then(function (img) {
			ctx.fillStyle = ctx.createPattern(img, 'repeat');
			ctx.fillRect(0, 0, 300, 300);
		});
}

export function patternWithCanvas(canvas) {
	const patternCanvas = Canvas.createCustomView();
	const patternContext = patternCanvas.getContext('2d') as any;

	// Give the pattern a width and height of 50
	patternCanvas.width = 50;
	patternCanvas.height = 50;

	// Give the pattern a background color and draw an arc
	patternContext.fillStyle = '#fec';
	patternContext.fillRect(0, 0, patternCanvas.width, patternCanvas.height);
	patternContext.arc(0, 0, 50, 0, .5 * Math.PI);
	patternContext.stroke();

	const ctx = canvas.getContext('2d');

	// Create our primary canvas and fill it with the pattern
	ctx.fillStyle = ctx.createPattern(patternCanvas, 'repeat');
	ctx.fillRect(0, 0, ctx.canvas.width, ctx.canvas.height);
}

export function clip(canvas) {
	const ctx = canvas.getContext('2d');
	// Create circular clipping region
	ctx.beginPath();
	ctx.arc(100, 75, 50, 0, Math.PI * 2);
	ctx.clip();

	// Draw stuff that gets clipped
	ctx.fillStyle = 'blue';
	ctx.fillRect(0, 0, canvas.width, canvas.height);
	ctx.fillRect(0, 0, canvas.width, canvas.height);
	ctx.fillStyle = 'orange';
	ctx.fillRect(0, 0, 100, 100);
}

export function isPointInStrokeTouch(canvas) {
	const ctx = canvas.getContext('2d');
	// Create ellipse
	const ellipse = new Path2D();
	ellipse.ellipse(150, 75, 40, 60, Math.PI * .25, 0, 2 * Math.PI);
	ctx.lineWidth = 25;
	ctx.strokeStyle = 'red';
	ctx.fill(ellipse);
	ctx.stroke(ellipse);


	// Listen for mouse moves
	canvas.addEventListener('touchmove', function (args) {
		// Check whether point is inside ellipse's stroke
		const event = args.changedTouches[0];
		//console.log(event.clientX, event.clientY);
		// event.offsetX, event.offsetY
		if (ctx.isPointInStroke(ellipse, event.offsetX, event.offsetY)) {
			ctx.strokeStyle = 'green';
		} else {
			ctx.strokeStyle = 'red';
		}

		// Draw ellipse
		ctx.clearRect(0, 0, canvas.width, canvas.height);
		ctx.fill(ellipse);
		ctx.stroke(ellipse);
	});
}

export function march(canvas) {
	const ctx = canvas.getContext('2d');
	var offset = 0;
	ctx.scale(3, 3);

	function draw() {
		ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
		ctx.setLineDash([4, 2]);
		ctx.lineDashOffset = -offset;
		ctx.strokeRect(10, 10, 100, 100);
	}

	function _march() {
		offset++;
		if (offset > 16) {
			offset = 0;
		}
		draw();
		setTimeout(_march, 20);
	}

	_march();
}
