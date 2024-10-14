let LAF;
export function particlesLarge(canvas) {
	canvas.width = canvas.clientWidth * window.devicePixelRatio;
	canvas.height = canvas.clientHeight * window.devicePixelRatio;
	const ctx = canvas.getContext('2d');
	//ctx.scale(window.devicePixelRatio, window.devicePixelRatio);

	const width = canvas.width;
	const height = canvas.width;

	/**
	 * Generates random particles using canvas
	 *
	 * @class Particles
	 * @constructor
	 */
	function Particles() {
		//particle colors
		this.colors = ['255, 255, 255', '255, 99, 71', '19, 19, 19'];
		//adds gradient to particles on true
		this.blurry = true;
		//adds white border
		this.border = false;
		//particle radius min/max
		this.minRadius = 10;
		this.maxRadius = 35;
		//particle opacity min/max
		this.minOpacity = 0.005;
		this.maxOpacity = 0.5;
		//particle speed min/max
		this.minSpeed = 0.05;
		this.maxSpeed = 0.5;
		//frames per second
		this.fps = 60;
		//number of particles
		this.numParticles = 500;
		//required canvas variables
		this.canvas = canvas;
		this.ctx = ctx;
	}

	/**
	 * Initializes everything
	 * @method init
	 */
	Particles.prototype.init = function () {
		this.render();
		this.createCircle();
	};

	/**
	 * generates random number between min and max values
	 * @param  {number} min value
	 * @param  {number} max malue
	 * @return {number} random number between min and max
	 * @method _rand
	 */
	Particles.prototype._rand = function (min, max) {
		return Math.random() * (max - min) + min;
	};

	/**
	 * Sets canvas size and updates values on resize
	 * @method render
	 */
	Particles.prototype.render = function () {
		var self = this,
			wHeight = height,
			wWidth = width;
	};

	/**
	 * Randomly creates particle attributes
	 * @method createCircle
	 */
	Particles.prototype.createCircle = function () {
		var particle = [];

		for (var i = 0; i < this.numParticles; i++) {
			var self = this,
				color = self.colors[~~self._rand(0, self.colors.length)];

			particle[i] = {
				radius: self._rand(self.minRadius, self.maxRadius),
				xPos: self._rand(0, width),
				yPos: self._rand(0, height),
				xVelocity: self._rand(self.minSpeed, self.maxSpeed),
				yVelocity: self._rand(self.minSpeed, self.maxSpeed),
				color: 'rgba(' + color + ',' + self._rand(self.minOpacity, self.maxOpacity) + ')',
			};

			//once values are determined, draw particle on canvas
			self.draw(particle, i);
		}
		//...and once drawn, animate the particle
		self.animate(particle);
	};

	/**
	 * Draws particles on canvas
	 * @param  {array} Particle array from createCircle method
	 * @param  {number} i value from createCircle method
	 * @method draw
	 */
	Particles.prototype.draw = function (particle, i) {
		var self = this,
			ctx = self.ctx;

		if (self.blurry === true) {
			//creates gradient if blurry === true
			var grd = ctx.createRadialGradient(particle[i].xPos, particle[i].yPos, particle[i].radius, particle[i].xPos, particle[i].yPos, particle[i].radius / 1.25);

			grd.addColorStop(1.0, particle[i].color);
			grd.addColorStop(0.0, 'rgba(34, 34, 34, 0)');
			ctx.fillStyle = grd;
		} else {
			//otherwise sets to solid color w/ opacity value
			ctx.fillStyle = particle[i].color;
		}

		if (self.border === true) {
			ctx.strokeStyle = '#fff';
			ctx.stroke();
		}

		ctx.beginPath();
		ctx.arc(particle[i].xPos, particle[i].yPos, particle[i].radius, 0, 2 * Math.PI, false);
		ctx.fill();
	};

	/**
	 * Animates particles
	 * @param  {array} particle value from createCircle & draw methods
	 * @method animate
	 */
	Particles.prototype.animate = function (particle) {
		var self = this,
			ctx = self.ctx;

		const doDrawing = (LAF = requestAnimationFrame(function () {
			//clears canvas
			self.clearCanvas();
			//then redraws particles in new positions based on velocity
			for (var i = 0; i < self.numParticles; i++) {
				particle[i].xPos += particle[i].xVelocity;
				particle[i].yPos -= particle[i].yVelocity;

				//if particle goes off screen call reset method to place it offscreen to the left/bottom
				if (particle[i].xPos > self.width + particle[i].radius || particle[i].yPos > self.height + particle[i].radius) {
					self.resetParticle(particle, i);
				} else {
					self.draw(particle, i);
				}
			}
		}));
	};

	/**
	 * Resets position of particle when it goes off screen
	 * @param  {array} particle value from createCircle & draw methods
	 * @param  {number} i value from createCircle method
	 * @method resetParticle
	 */
	Particles.prototype.resetParticle = function (particle, i) {
		var self = this;

		var random = self._rand(0, 1);

		if (random > 0.5) {
			// 50% chance particle comes from left side of window...
			particle[i].xPos = -particle[i].radius;
			particle[i].yPos = self._rand(0, height);
		} else {
			//... or bottom of window
			particle[i].xPos = self._rand(0, width);
			particle[i].yPos = canvas.height + particle[i].radius;
		}
		//redraw particle with new values
		self.draw(particle, i);
	};

	/**
	 * Clears canvas between animation frames
	 * @method clearCanvas
	 */
	Particles.prototype.clearCanvas = function () {
		this.ctx.clearRect(0, 0, width, height);
	};

	// go go go!
	var particle = new Particles().init();
}

export function cancelParticlesLarge() {
	cancelAnimationFrame(LAF);
	LAF = 0;
}
