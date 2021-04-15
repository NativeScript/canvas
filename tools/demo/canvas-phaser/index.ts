import {DemoSharedBase} from '../utils';
import {setupGame as flappySetup} from './games/flappybird/game';
import {setupGame as breakoutSetup} from './games/breakout/breakout-game';
import {setupGame as muybridgeSetup} from './animation/muybridge/muybridge';
import {GestureStateTypes} from "@nativescript/core";

class TouchList extends Array {
	item(index: number) {
		return this[index];
	}
}

export class DemoSharedCanvasPhaser extends DemoSharedBase {
	canvas;
	state = 'idle';

	canvasLoaded(args) {
		this.canvas = args.object;
		 //this.initFlappyBird();
		this.initBreakOut();
		//this.initMuyBridge();
	}

	initFlappyBird() {
		flappySetup(this.canvas);
	}

	initBreakOut() {
		breakoutSetup(this.canvas);
	}

	initMuyBridge() {
		muybridgeSetup(this.canvas);
	}

	onTouch(event: any) {
		// event.getAllPointers()[0]

		/*
		 mousemove
CONSOLE LOG file:///node_modules/@nativescript/core/ui/core/view/view-common.js:193:0: events mousedown
CONSOLE LOG file:///node_modules/@nativescript/core/ui/core/view/view-common.js:193:0: events mouseup
CONSOLE LOG file:///node_modules/@nativescript/core/ui/core/view/view-common.js:193:0: events mouseover
CONSOLE LOG file:///node_modules/@nativescript/core/ui/core/view/view-common.js:193:0: events mouseout
CONSOLE LOG file:///node_modules/@nativescript/core/ui/core/view/view-common.js:193:0: events wheel
CONSOLE LOG file:///node_modules/@nativescript/core/ui/core/view/view-common.js:193:0: events touchstart
CONSOLE LOG file:///node_modules/@nativescript/core/ui/core/view/view-common.js:193:0: events touchmove
CONSOLE LOG file:///node_modules/@nativescript/core/ui/core/view/view-common.js:193:0: events touchend
CONSOLE LOG file:///node_modules/@nativescript/core/ui/core/view/view-common.js:193:0: events touchcancel
CONSOLE LOG file:///node_modules/@nativescript/core/ui/core/view/view-common.js:193:0: events touchover
CONSOLE LOG file:///node_modules/@nativescript/core/ui/core/view/view-common.js:193:0: events touchout
CONSOLE LOG file:///node_modules/nativescript-http-async/xhr/TNSXMLHttpRequest.js:464:0: type image/png
		*/

		if (event.eventName === "touch") {
			switch (event.action) {
				case "down":
					// if(state === 'idle'){
					this.canvas.notify(this.getTouchEvent("touchstart", event, this.canvas));
					this.state = 'touching';
					//   }
					break;
				case "up":
					//   if(state === 'touching'){
					this.canvas.notify(this.getTouchEvent("touchend", event, this.canvas));
					this.state = 'idle';
					//  }
					break;
				case "cancel":
					//  if(state === 'touching'){
					this.canvas.notify(this.getTouchEvent("touchcancel", event, this.canvas));
					this.state = 'idle';
					//  }
					break;
				case 'move':
					//if(state === 'touching'){

					// state = 'moving';
					// }
					break;
				default:
					break;
			}
		} else if (event.eventName === "pan") {
			if (event.state === GestureStateTypes.began || event.state === GestureStateTypes.changed) {
				this.canvas.notify(this.getTouchEvent("touchmove", event, this.canvas));
			}

			//canvas.notify(getTouchEvent("touchmove", event));
			//this.game.updateControls(event.deltaX);
		}
	}

	getTouchEvent(name, event, target) {
		const pointers = new TouchList();
		if (name === "touchmove") {
			pointers.push({
				clientX: event.deltaX * 3,
				clientY: event.deltaY * 3,
				force: 0.0,
				identifier: 0,
				pageX: event.deltaX * 3,
				pageY: event.deltaY * 3,
				radiusX: 0,
				radiusY: 0,
				rotationAngle: 0,
				screenX: event.deltaX * 3,
				screenY: event.deltaY * 3,
				target,
			});
		} else {
			const count = event.getAllPointers().length;
			for (let i = 0; i < count; i++) {
				const point = event.getAllPointers()[i];
				pointers.push({
					clientX: point.getX(),
					clientY: point.getY(),
					force: 0.0,
					identifier: i,
					pageX: point.getX(),
					pageY: point.getY(),
					radiusX: 0,
					radiusY: 0,
					rotationAngle: 0,
					screenX: point.getX(),
					screenY: point.getY(),
					target,
				});
			}
		}


		return {
			eventName: name,
			object: null,
			defaultPrevented: false,
			cancelable: false,
			altKey: false,
			changedTouches: pointers,
			ctrlKey: false,
			metaKey: false,
			shiftKey: false,
			targetTouches: pointers,
			touches: pointers,
			preventDefault: () => {
			},
			target
		};
	}
}
