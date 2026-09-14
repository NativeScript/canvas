let LAF = 0;

export function swarm(canvas, width?, height?, nativeCanvas?) {
	var requestAnimFrame = requestAnimationFrame;

	function init() {
		// Otherwise re-entering the demo leaves the previous rAF loop running.
		if (LAF) {
			cancelAnimationFrame(LAF);
			LAF = 0;
		}

		// Set the canvas width and height to occupy full window
		var W = width || canvas.clientWidth * window.devicePixelRatio,
			H = height || canvas.clientHeight * window.devicePixelRatio;

		canvas.width = W;
		canvas.height = H;

		var ctx = canvas.getContext ? canvas.getContext('2d') : canvas;

		ctx.scale(window.devicePixelRatio, window.devicePixelRatio);

		// Some variables for later use
		var particleCount = 1500,
			particles = [],
			minDist = 50;

		// Function to paint the canvas black
		function paintCanvas() {
			// Set the fill color to black
			ctx.fillStyle = 'black';

			// This will create a rectangle of white color from the
			// top left (0,0) to the bottom right corner (W,H)
			ctx.fillRect(0, 0, W, H);
		}

		// Now the idea is to create some particles that will attract
		// each other when they come close. We will set a minimum
		// distance for it and also draw a line when they come
		// close to each other.

		// The attraction can be done by increasing their velocity as
		// they reach closer to each other

		// Let's make a function that will act as a class for
		// our particles.

		function Particle() {
			// Position them randomly on the canvas
			// Math.random() generates a random value between 0
			// and 1 so we will need to multiply that with the
			// canvas width and height.
			this.x = Math.random() * W;
			this.y = Math.random() * H;

			// We would also need some velocity for the particles
			// so that they can move freely across the space
			this.vx = -1 + Math.random() * 2;
			this.vy = -1 + Math.random() * 2;

			// Now the radius of the particles. I want all of
			// them to be equal in size so no Math.random() here..
			this.radius = 4;
		}

		// Time to push the particles into an array
		for (var i = 0; i < particleCount; i++) {
			particles.push(new Particle());
		}

		// Bucket particles into minDist-sized cells so each looks at 9 cells, not all
		// 1500. Only four neighbours are scanned (E, SW, S, SE), plus the own cell from
		// i+1: that visits each unordered pair exactly once, which matters because a hit
		// accelerates *both* particles.
		const cols = Math.max(1, Math.ceil(W / minDist));
		const rows = Math.max(1, Math.ceil(H / minDist));
		const buckets: number[][] = new Array(cols * rows);
		for (let b = 0; b < buckets.length; b++) {
			buckets[b] = [];
		}
		const NEIGHBOUR_OFFSETS = [
			[1, 0],
			[-1, 1],
			[0, 1],
			[1, 1],
		];

		// Lines differ only in alpha, so quantise it into levels and collect each
		// level's segments into one path: one stroke per level instead of ~7500.
		const ALPHA_LEVELS = 16;
		const laneColors: string[] = [];
		const lanes: number[][] = [];
		for (let l = 0; l < ALPHA_LEVELS; l++) {
			// Original alpha was 1.2 - dist/minDist, i.e. (0.2 .. 1.2] over the range.
			const alpha = 0.2 + (1.0 * (l + 0.5)) / ALPHA_LEVELS;
			laneColors.push('rgba(255,255,255,' + alpha + ')');
			lanes.push([]);
		}

		function draw() {
			// Call the paintCanvas function here so that our canvas
			// will get re-painted in each next frame
			paintCanvas();

			// One path and one fill for all particles. The moveTo before each arc keeps
			// them separate subpaths -- without it the circles chain together.
			ctx.fillStyle = 'white';
			ctx.beginPath();
			for (let i = 0; i < particles.length; i++) {
				const p = particles[i];
				ctx.moveTo(p.x + p.radius, p.y);
				ctx.arc(p.x, p.y, p.radius, 0, Math.PI * 2, false);
			}
			ctx.fill();

			//Finally call the update function
			update();
		}

		// Give every particle some life
		function update() {
			for (let l = 0; l < ALPHA_LEVELS; l++) {
				lanes[l].length = 0;
			}
			for (let b = 0; b < buckets.length; b++) {
				buckets[b].length = 0;
			}

			// In this function, we are first going to update every
			// particle's position according to their velocities
			for (let i = 0; i < particles.length; i++) {
				const p = particles[i];

				// Change the velocities
				p.x += p.vx;
				p.y += p.vy;

				// We don't want to make the particles leave the
				// area, so just change their position when they
				// touch the walls of the window
				if (p.x + p.radius > W) p.x = p.radius;
				else if (p.x - p.radius < 0) {
					p.x = W - p.radius;
				}

				if (p.y + p.radius > H) p.y = p.radius;
				else if (p.y - p.radius < 0) {
					p.y = H - p.radius;
				}

				const cx = Math.min(cols - 1, Math.max(0, (p.x / minDist) | 0));
				const cy = Math.min(rows - 1, Math.max(0, (p.y / minDist) | 0));
				buckets[cy * cols + cx].push(i);
			}

			// Now we need to make them attract each other, so check the distance
			// between nearby pairs and compare it to the minDist we have set.
			for (let cy = 0; cy < rows; cy++) {
				for (let cx = 0; cx < cols; cx++) {
					const cell = buckets[cy * cols + cx];
					if (cell.length === 0) {
						continue;
					}

					for (let a = 0; a < cell.length; a++) {
						const p1 = particles[cell[a]];

						// Same cell: only forward, so each pair is seen once.
						for (let b = a + 1; b < cell.length; b++) {
							distance(p1, particles[cell[b]]);
						}

						for (let n = 0; n < NEIGHBOUR_OFFSETS.length; n++) {
							const nx = cx + NEIGHBOUR_OFFSETS[n][0];
							const ny = cy + NEIGHBOUR_OFFSETS[n][1];
							if (nx < 0 || nx >= cols || ny >= rows) {
								continue;
							}
							const other = buckets[ny * cols + nx];
							for (let b = 0; b < other.length; b++) {
								distance(p1, particles[other[b]]);
							}
						}
					}
				}
			}

			for (let l = 0; l < ALPHA_LEVELS; l++) {
				const lane = lanes[l];
				if (lane.length === 0) {
					continue;
				}
				ctx.strokeStyle = laneColors[l];
				ctx.beginPath();
				for (let s = 0; s < lane.length; s += 4) {
					ctx.moveTo(lane[s], lane[s + 1]);
					ctx.lineTo(lane[s + 2], lane[s + 3]);
				}
				ctx.stroke();
			}
		}

		// Distance calculator between two particles
		function distance(p1, p2) {
			const dx = p1.x - p2.x,
				dy = p1.y - p2.y;
			const distSq = dx * dx + dy * dy;

			// Squared compare: only in-range pairs pay the square root.
			if (distSq > minDist * minDist) {
				return;
			}

			const dist = Math.sqrt(distSq);

			let level = ((1 - dist / minDist) * ALPHA_LEVELS) | 0;
			if (level >= ALPHA_LEVELS) level = ALPHA_LEVELS - 1;
			else if (level < 0) level = 0;
			const lane = lanes[level];
			lane.push(p1.x, p1.y, p2.x, p2.y);

			// Some acceleration for the partcles
			// depending upon their distance
			const ax = dx / 2000,
				ay = dy / 2000;

			// Apply the acceleration on the particles
			p1.vx -= ax;
			p1.vy -= ay;

			p2.vx += ax;
			p2.vy += ay;
		}

		// Start the main animation loop using requestAnimFrame
		function animloop() {
			LAF = requestAnimFrame(animloop);
			draw();
		}

		animloop();
	}

	init();
}

export function cancelSwarm() {
	cancelAnimationFrame(LAF);
	LAF = 0;
}
