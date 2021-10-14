import { ImageSource } from '@nativescript/core';

export function issue54(canvas) {
	const ctx = canvas.getContext('2d');
	ImageSource.fromUrl('https://mdn.mozillademos.org/files/222/Canvas_createpattern.png').then(function (img) {
		console.log('issue54');
		const pattern = ctx.createPattern(img, 'repeat');
		ctx.fillStyle = pattern;
		ctx.fillRect(0, 0, 300, 300);
	});
}
