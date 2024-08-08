import { func } from 'canvas-phaser-ce/games/utils';

let LAF;

export function colorRain(canvas) {
	//initial
	var w = canvas.width,
		h = canvas.height,
		ctx = canvas.getContext('2d'),
		//parameters
		total = w / 2,
		accelleration = 0.05,
		//afterinitial calculations
		size = w / total,
		occupation = w / total,
		repaintColor = 'black';
	var colors: number[] = [],
		dots = new Array(total),
		dotsVel = new Array(total),
		parsedColors = [];

	//setting the colors' hue
	//and y level for all dots
	var portion = 360 / total;
	for (var i = 0; i < total; ++i) {
		colors[i] = portion * i;

		dots[i] = h;
		dotsVel[i] = 10;
	}

	var buf = {};
	function anim() {
		LAF = requestAnimationFrame(anim);

		ctx.fillStyle = repaintColor;
		ctx.fillRect(0, 0, w, h);

		for (var i = 0; i < total; ++i) {
			var currentY = dots[i] - 1;
			dots[i] += dotsVel[i] += accelleration;
			//const randomColor = Math.floor(Math.random()*16777215).toString(16);
			// let parsedColor = parsedColors[i];
			// if(!parsedColor){
			// 	parsedColor = `hsl(${colors[i]}, 80%, 50%)`;
			// }
			ctx.fillStyle = `hsl(${colors[i]}, 80%, 50%)`; //`hsl(${color}, 80%, 50%, 1)`;
			ctx.fillRect(occupation * i, currentY, size, dotsVel[i] + 1);

			if (dots[i] > h && Math.random() < 0.01) {
				dots[i] = dotsVel[i] = 0;
			}

			//	parsedColors[i] = parsedColor;
		}
	}

	anim();
}

export function cancelRain() {
	cancelAnimationFrame(LAF);
	LAF = 0;
}
