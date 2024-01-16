import { ImageSource } from '@nativescript/core';

export function flappyBird(canvas) {
	var ctx,
		width,
		height,
		fgpos = 0,
		frames = 0,
		score = 0,
		best = 0,
		currentstate,
		states = {
			Splash: 0,
			Game: 1,
			Score: 2,
		},
		okbtn,
		bird = {
			x: 60,
			y: 0,
			frame: 0,
			animation: [0, 1, 2, 1],
			rotation: 0,
			velocity: 0,
			radius: 12,
			gravity: 0.25,
			_jump: 4.6,

			jump: function () {
				this.velocity = -this._jump;
			},

			update: function () {
				var n = currentstate === states.Splash ? 10 : 5;
				this.frame += frames % n === 0 ? 1 : 0;
				this.frame %= this.animation.length;

				if (currentstate === states.Splash) {
					this.y = height - 280 + 5 * Math.cos(frames / 10);
					this.rotation = 0;
				} else {
					this.velocity += this.gravity;
					this.y += this.velocity;

					if (this.y >= height - s_fg.height - 10) {
						this.y = height - s_fg.height - 10;
						if (currentstate === states.Game) {
							currentstate = states.Score;
						}
						this.velocity = this._jump;
					}

					if (this.velocity >= this._jump) {
						this.frame = 1;
						this.rotation = Math.min(Math.PI / 2, this.rotation + 0.3);
					} else {
						this.rotation = -0.3;
					}
				}
			},

			draw: function (ctx) {
				ctx.save();
				ctx.translate(this.x, this.y);
				ctx.rotate(this.rotation);

				var n = this.animation[this.frame];
				s_bird[n].draw(ctx, -s_bird[n].width / 2, -s_bird[n].height / 2);
				ctx.restore();
			},
		},
		pipes = {
			_pipes: [],

			reset: function () {
				this._pipes = [];
			},

			update: function () {
				if (frames % 100 === 0) {
					var _y = height - (s_pipeSouth.height + s_fg.height + 120 + 200 * Math.random());
					this._pipes.push({
						x: 500,
						y: _y,
						width: s_pipeSouth.width,
						height: s_pipeSouth.height,
					});
				}
				for (var i = 0, len = this._pipes.length; i < len; i++) {
					var p = this._pipes[i];

					if (i === 0) {
						score += p.x === bird.x ? 1 : 0;

						var cx = Math.min(Math.max(bird.x, p.x), p.x + p.width);
						var cy1 = Math.min(Math.max(bird.y, p.y), p.y + p.height);
						var cy2 = Math.min(Math.max(bird.y, p.y + p.height + 80), p.y + 2 * p.height + 80);

						var dx = bird.x - cx;
						var dy1 = bird.y - cy1;
						var dy2 = bird.y - cy2;

						var d1 = dx * dx + dy1 * dy1;
						var d2 = dx * dx + dy2 * dy2;

						var r = bird.radius * bird.radius;

						if (r > d1 || r > d2) {
							currentstate = states.Score;
						}
					}

					p.x -= 2;
					if (p.x < -50) {
						this._pipes.splice(i, 1);
						i--;
						len--;
					}
				}
			},

			draw: function (...args) {
				for (var i = 0, len = this._pipes.length; i < len; i++) {
					var p = this._pipes[i];
					s_pipeSouth.draw(ctx, p.x, p.y);
					s_pipeNorth.draw(ctx, p.x, p.y + 80 + p.height);
				}
			},
		},
		img;

	function onpress(evt) {
		switch (currentstate) {
			case states.Splash:
				currentstate = states.Game;
				bird.jump();
				break;
			case states.Game:
				bird.jump();
				break;
			case states.Score:
				var mx = evt.offsetX,
					my = evt.offsetY;

				if (mx == null || my == null) {
					mx = evt.changedTouches[0].clientX;
					my = evt.changedTouches[0].clientY;
				}

				if (okbtn.x < mx && mx < okbtn.x + okbtn.width && okbtn.y < my && my < okbtn.y + okbtn.height) {
					pipes.reset();
					currentstate = states.Splash;
					score = 0;
				}
				break;
		}
	}

	function main() {
		width = canvas.width;
		height = canvas.height;

		canvas.addEventListener('touchstart', onpress);

		var evt = 'touchstart';
		if (width >= 500) {
			width = 320;
			height = 480;
			canvas.style.border = '1px solid #000';
			evt = 'mousedown';
		}

		canvas.width = width;
		canvas.height = height;
		ctx = canvas.getContext('2d');

		currentstate = states.Splash;

		img = document.createElement('img');

		img.onload = (event) => {
			initSprites(img);
			ctx.fillStyle = s_bg.color;

			(okbtn = {
				x: (width - s_buttons.Ok.width) / 2,
				y: height - 200,
				width: s_buttons.Ok.width,
				height: s_buttons.Ok.height,
			}),
				run();
		};
		img.src = 'https://raw.githubusercontent.com/maxwihlborg/youtube-tutorials/master/flappy/starter/res/sheet.png';
	}

	function run() {
		var loop = function () {
			requestAnimationFrame(loop);
			update();
			render();
		};
		requestAnimationFrame(loop);
	}

	function update() {
		frames++;

		if (currentstate !== states.Score) {
			fgpos = (fgpos - 2) % 14;
		} else {
			best = Math.max(best, score);
		}

		if (currentstate === states.Game) {
			pipes.update();
		}

		bird.update();
	}

	function render() {
		ctx.fillRect(0, 0, width, height);

		s_bg.draw(ctx, 0, height - s_bg.height);
		s_bg.draw(ctx, s_bg.width, height - s_bg.height);

		pipes.draw(ctx);
		bird.draw(ctx);

		s_fg.draw(ctx, fgpos, height - s_fg.height);
		s_fg.draw(ctx, fgpos + s_fg.width, height - s_fg.height);

		var width2 = width / 2;

		if (currentstate === states.Splash) {
			s_splash.draw(ctx, width2 - s_splash.width / 2, height - 300);
			s_text.GetReady.draw(ctx, width2 - s_text.GetReady.width / 2, height - 380);
		}

		if (currentstate === states.Score) {
			s_text.GameOver.draw(ctx, width2 - s_text.GameOver.width / 2, height - 400);
			s_score.draw(ctx, width2 - s_score.width / 2, height - 340);
			s_buttons.Ok.draw(ctx, okbtn.x, okbtn.y);

			s_numberS.draw(ctx, width2 - 47, height - 304, score, null, 10);
			s_numberS.draw(ctx, width2 - 47, height - 262, best, null, 10);
		} else {
			s_numberB.draw(ctx, null, 20, score, width2);
		}
	}

	var s_bird, s_bg, s_fg, s_pipeNorth, s_pipeSouth, s_text, s_score, s_splash, s_buttons, s_numberS, s_numberB;

	function Sprite(img, x, y, width, height) {
		this.img = img;
		this.x = x * 2;
		this.y = y * 2;
		this.width = width * 2;
		this.height = height * 2;
	}
	Sprite.prototype.draw = function (ctx, x, y) {
		ctx.drawImage(this.img, this.x, this.y, this.width, this.height, x, y, this.width, this.height);
	};

	function initSprites(img) {
		s_bird = [new Sprite(img, 156, 115, 17, 12), new Sprite(img, 156, 128, 17, 12), new Sprite(img, 156, 141, 17, 12)];

		s_bg = new Sprite(img, 0, 0, 138, 114);
		s_bg.color = '#70C5CF';
		s_fg = new Sprite(img, 138, 0, 112, 56);

		s_pipeNorth = new Sprite(img, 251, 0, 26, 200);
		s_pipeSouth = new Sprite(img, 277, 0, 26, 200);

		s_text = {
			FlappyBird: new Sprite(img, 59, 114, 96, 22),
			GameOver: new Sprite(img, 59, 136, 94, 19),
			GetReady: new Sprite(img, 59, 155, 87, 22),
		};
		s_buttons = {
			Rate: new Sprite(img, 79, 177, 40, 14),
			Menu: new Sprite(img, 119, 177, 40, 14),
			Share: new Sprite(img, 159, 177, 40, 14),
			Score: new Sprite(img, 79, 191, 40, 14),
			Ok: new Sprite(img, 119, 191, 40, 14),
			Start: new Sprite(img, 159, 191, 40, 14),
		};

		s_score = new Sprite(img, 138, 56, 113, 58);
		s_splash = new Sprite(img, 0, 114, 59, 49);

		s_numberS = new Sprite(img, 0, 177, 6, 7);
		s_numberB = new Sprite(img, 0, 188, 7, 10);

		s_numberS.draw = s_numberB.draw = function (ctx, x, y, num, center, offset) {
			num = num.toString();

			var step = this.width + 2;

			if (center) {
				x = center - (num.length * step - 2) / 2;
			}

			if (offset) {
				x += step * (offset - num.length);
			}

			for (var i = 0, len = num.length; i < len; i++) {
				var n = parseInt(num[i]);
				ctx.drawImage(img, step * n, this.y, this.width, this.height, x, y, this.width, this.height);
				x += step;
			}
		};
	}

	main();
}
