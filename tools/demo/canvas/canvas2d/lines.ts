import { Canvas } from '@nativescript/canvas';
import { Screen } from '@nativescript/core';

export function lines(c: Canvas) {
	var w = c.getMeasuredWidth(),
		h = c.getMeasuredHeight(),
		ctx = c.getContext('2d') as never as CanvasRenderingContext2D,
		spawnProb = 1,
		numberOfMoves = [8, 16], //[min, max]
		distance = [50, 400],
		attenuator = 30,
		timeBetweenMoves = [4, 10],
		lines = [] as any[],
		frame = (Math.random() * 360) | 0;
	c.width = w;
	c.height = h;
	ctx.lineWidth = 1;

	//  console.log(c.width, c.height, c.getMeasuredWidth(), c.getMeasuredHeight());

	function rand(ar: any) {
		return Math.random() * (ar[1] - ar[0]) + ar[0];
	}
	function Line() {
		this.x = Math.random() * w;
		this.y = Math.random() * h;
		this.last = { x: 0, y: 0 };
		this.target = { x: 0, y: 0 };
		this.totalMoves = rand(numberOfMoves);
		this.move = 0;
		this.timeBetweenMoves = rand(timeBetweenMoves);
		this.timeSpentThisMove = this.timeBetweenMoves;
		this.distance = rand(distance);

		this.color = `hsl(${frame % 360}, 80%, 50%)`;
	}
	Line.prototype.use = function () {
		++this.timeSpentThisMove;
		if (this.timeSpentThisMove >= this.timeBetweenMoves) {
			++this.move;
			this.timeSpentThisMove = 0;

			var rad = Math.random() * 2 * Math.PI;
			this.target.x = this.x + Math.cos(rad) * this.distance;
			this.target.y = this.y + Math.sin(rad) * this.distance;
		}

		this.last.x = this.x;
		this.last.y = this.y;

		this.x += (this.x - this.target.x) / attenuator;
		this.y += (this.y - this.target.y) / attenuator;

		ctx.strokeStyle = this.color;
		ctx.shadowColor = this.color;

		ctx.beginPath();
		ctx.moveTo(this.last.x, this.last.y);
		ctx.lineTo(this.x, this.y);
		ctx.stroke();
	};

	let start = 0;
	function anim(now) {
		if (start === 0) {
			start = now;
		}
		frame = now - start + 1.5;
		ctx.shadowBlur = 0;
		ctx.fillStyle = 'rgba(0, 0, 0, .04)';
		ctx.fillRect(0, 0, w, h);
		ctx.shadowBlur = 10;

		if (Math.random() < spawnProb) lines.push(new Line());

		for (var i = 0; i < lines.length; ++i) {
			const line = lines[i];
			line.use();

			if (line.move >= line.totalMoves) {
				lines.splice(i, 1);
				--i;
			}
		}

		requestAnimationFrame(anim);
	}

	anim(0);
}
