import { EventData, ImageSource, Page, WebView } from '@nativescript/core';
import { DemoSharedCanvas } from '@demo/shared';
import { ImageAsset } from '@nativescript/canvas';

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
	constructor() {
		super();
		this.image.fromFileSync('~/assets/file-assets/webgl/svh.jpeg');
		console.log(this.image.width);
	}
}
