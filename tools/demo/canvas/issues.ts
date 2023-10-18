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

export function issue93(canvas) {
	const ctx = canvas.getContext('2d');

	ctx.rect(50, 20, 200, 120);
	ctx.stroke();
	ctx.fillStyle = 'red';
	setTimeout(() => ctx.fillRect(0, 0, 150, 100), 2000);
}
