import { Canvas } from '@nativescript/canvas';

export interface Input {
	// Digital input (e.g keyboard state)
	readonly digital: {
		readonly forward: boolean;
		readonly backward: boolean;
		readonly left: boolean;
		readonly right: boolean;
		readonly up: boolean;
		readonly down: boolean;
	};
	// Analog input (e.g mouse, touchscreen)
	readonly analog: {
		readonly x: number;
		readonly y: number;
		readonly zoom: number;
		readonly touching: boolean;
	};
}

// InputHandler is a function that when called, returns the current Input state.
export type InputHandler = Input;

export type InputHandlers = {
	onTouchStart?: (event) => void;
	onTouchMove?: (event) => void;
	onTouchEnd?: (event) => void;
};

// createInputHandler returns an InputHandler by attaching event handlers to the window and canvas.
export function createInputHandler(canvas: Canvas): InputHandler {
	const digital = {
		forward: false,
		backward: false,
		left: false,
		right: false,
		up: false,
		down: false,
	};
	const analog = {
		x: 0,
		y: 0,
		zoom: 0,
		touching: false,
	};

	// const setDigital = (e: KeyboardEvent, value: boolean) => {
	//   switch (e.code) {
	//     case 'KeyW':
	//       digital.forward = value;
	//       e.preventDefault();
	//       e.stopPropagation();
	//       break;
	//     case 'KeyS':
	//       digital.backward = value;
	//       e.preventDefault();
	//       e.stopPropagation();
	//       break;
	//     case 'KeyA':
	//       digital.left = value;
	//       e.preventDefault();
	//       e.stopPropagation();
	//       break;
	//     case 'KeyD':
	//       digital.right = value;
	//       e.preventDefault();
	//       e.stopPropagation();
	//       break;
	//     case 'Space':
	//       digital.up = value;
	//       e.preventDefault();
	//       e.stopPropagation();
	//       break;
	//     case 'ShiftLeft':
	//     case 'ControlLeft':
	//     case 'KeyC':
	//       digital.down = value;
	//       e.preventDefault();
	//       e.stopPropagation();
	//       break;
	//   }
	// };

	// window.addEventListener('keydown', (e) => setDigital(e, true));
	// window.addEventListener('keyup', (e) => setDigital(e, false));

	// canvas.style.touchAction = 'pinch-zoom';
	let lastX = 0;
	let lastY = 0;
	canvas.addEventListener('touchstart', (e: TouchEvent) => {
		const touch = e.touches[0];
		if (!touch) {
			return;
		}
		lastX = touch.pageX;
		lastY = touch.pageY;
		analog.touching = true;
	});

	canvas.addEventListener('touchend', (e) => {
		analog.touching = false;
	});

	canvas.addEventListener('touchmove', (e) => {
		if (analog.touching) {
			const touch = e.changedTouches[0];
			if (!touch) {
				return;
			}

			analog.x += touch.pageX - lastX;
			analog.y += touch.pageY - lastY;

			lastX = touch.pageX;
			lastY = touch.pageY;
		}
	});
	// canvas.addEventListener(
	//   'wheel',
	//   (e) => {
	//     mouseDown = (e.buttons & 1) !== 0;
	//     if (mouseDown) {
	//       // The scroll value varies substantially between user agents / browsers.
	//       // Just use the sign.
	//       analog.zoom += Math.sign(e.deltaY);
	//       e.preventDefault();
	//       e.stopPropagation();
	//     }
	//   },
	//   { passive: false }
	// );

	return {
		digital,
		analog,
	};
}
