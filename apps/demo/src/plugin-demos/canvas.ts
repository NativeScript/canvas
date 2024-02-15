import { EventData, ImageSource, Page, Screen, View, WebView } from '@nativescript/core';
import { DemoSharedCanvas } from '@demo/shared';
import { ImageAsset } from '@nativescript/canvas';
import { Dom, Group, Rect, drawAsImage } from '@nativescript/canvas/Dom';

export function navigatingTo(args: EventData) {
	const page = <Page>args.object;
	page.bindingContext = new DemoModel();
}

export function loaded(args) {
	const view = <WebView>args.object;
	view.src = `
<script>

document.addEventListener('DOMContentLoaded', ()=>{
  const canvas = document.createElement('canvas');
  canvas.style = "width:100%;height:100%;background-color:red;";
  document.body.appendChild(canvas);
  canvas.addEventListener('touchstart', e =>{
    console.log('touchstart',e);
  });
  canvas.addEventListener('pointerdown', e =>{
    console.log('pointerdown',e);
  });
  canvas.addEventListener('pointermove', e =>{
    console.log('pointermove',e);
  });
  canvas.addEventListener('touchmove', e =>{
    console.log('touchmove',e);
  });
  canvas.addEventListener('pointerup', e =>{
    console.log('pointerup',e);
  });
  canvas.addEventListener('touchend', e =>{
    console.log('touchend',e);
  });
  canvas.addEventListener('pointercancel', e =>{
    console.log('pointercancel',e);
  });
  canvas.addEventListener('touchcancel', e =>{
    console.log('touchcancel',e);
  });
  canvas.addEventListener('wheel', e =>{
    console.log('wheel',e);
  });
}, false);

</script>
<!DOCTYPE html>
<html>
<body>
</body>
</html>
`;
}

declare const android;

export class DemoModel extends DemoSharedCanvas {
	origin = { x: 128, y: 128 };
	points = [
		{ x: 128, y: 0 },
		{ x: 168, y: 80 },
		{ x: 256, y: 93 },
		{ x: 192, y: 155 },
		{ x: 207, y: 244 },
		{ x: 128, y: 202 },
		{ x: 49, y: 244 },
		{ x: 64, y: 155 },
		{ x: 0, y: 93 },
		{ x: 88, y: 80 },
		{ x: 128, y: 0 },
	];

	start = { x: 0, y: 0 };
	end = { x: 256, y: 256 };

	start1 = { x: 128, y: 128 };
	end1 = { x: 128, y: 16 };

	colors = ['blue', 'yellow'];

	image = new ImageAsset();

	width = 256;
	height = 256;
	strokeWidth = 10;
	c = { x: this.width / 2, y: this.height / 2 };
	r = (this.width - this.strokeWidth) / 2;
	groupR = 128;
	dom: Dom;
	img;
	transforms = [];
	sprites = [];
	constructor() {
		super();
		this.image.fromFile('~/assets/file-assets/webgl/svh.jpeg');
	}

	domLoaded(event) {
		this.dom = event.object;
		/*const size = { width: 25, height: 11.25 };
		const strokeWidth = 2;
		const imageSize = {
			width: size.width + strokeWidth,
			height: size.height + strokeWidth,
		};
		const group = new Group();
		const rect1 = new Rect();
		rect1.x = strokeWidth / 2;
		rect1.y = strokeWidth / 2;
		rect1.width = size.width;
		rect1.height = size.height;
		rect1.color = 'cyan' as any;

		const rect2 = new Rect();

		rect2.x = strokeWidth / 2;
		rect2.y = strokeWidth / 2;
		rect2.width = size.width;
		rect2.height = size.height;

		rect2.color = 'blue' as any;
		rect2.paintStyle = 'stroke';
		rect2.strokeWidth = strokeWidth;

		group.addChild(rect1);
		group.addChild(rect2);

		const img = drawAsImage([group], imageSize);

		const numberOfBoxes = 150;
		const pos = { x: 128, y: 128 };
		const width = 256;


		
		const sprites = new Array(numberOfBoxes).fill(0).map(() => {
			return { x: 0, y: 0, width: imageSize.width, height: imageSize.height };
		});

		this.set('sprites', sprites);

		const transforms = new Array(numberOfBoxes).fill(0).map((_, i) => {
			const tx = 5 + ((i * size.width) % width);
			const ty = 25 + Math.floor(i / (width / size.width)) * size.width;
			const r = Math.atan2(pos.y - ty, pos.x - tx);
			return { scos: Math.cos(r), ssin: Math.sin(r), tx, ty };
		});

		this.set('transforms', transforms);

		this.set('img', img);

		*/

	}

	rectLoaded(event) {
		event.object.parent.on(View.layoutChangedEvent, (e) => {
			const view = event.object;
			view.x = 50;
			view.y = 50;
			const width = view.width;
			const height = view.height;
			const maxWidth = view.parent.getMeasuredWidth() / Screen.mainScreen.scale;
			const maxHeight = view.parent.getMeasuredHeight() / Screen.mainScreen.scale;
			const maxX = maxWidth - width;
			const maxY = maxHeight - height;
			if (global.isAndroid) {
				const animator = android.animation.ValueAnimator.ofFloat([0, 1] as any);
				animator.setDuration(3000);

				animator.addUpdateListener(
					new android.animation.ValueAnimator.AnimatorUpdateListener({
						onAnimationUpdate(value) {
							const progress = value.getAnimatedValue();
							view.x = maxX * progress;
							view.y = maxY * progress;
						},
					})
				);

				animator.setInterpolator(new android.view.animation.AccelerateDecelerateInterpolator());
				animator.setRepeatMode(android.view.animation.Animation.REVERSE);
				animator.setRepeatCount(android.animation.ValueAnimator.INFINITE);

				animator.start();
			}
		});
	}
}
