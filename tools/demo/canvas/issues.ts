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

export function drawChart(canvas) {
	const { width, height } = canvas;
	const ctx = canvas.getContext('2d') as CanvasRenderingContext2D;

	let centerX = width / 2;
	let centerY = height / 2;
	ctx.clearRect(0, 0, width, height);
	

	ctx.strokeStyle = 'cyan';
	ctx.lineWidth = 1;
	ctx.beginPath();
	ctx.moveTo(centerX, centerY - 50);
	ctx.lineTo(centerX - 12, centerY);
	ctx.stroke();

	ctx.strokeStyle = 'blue';
	ctx.beginPath();
	ctx.rect(centerX - 120, centerY - 150, 80, 80);
	ctx.rect(centerX + 40, centerY - 150, 80, 80);
	ctx.stroke();

	ctx.strokeStyle = 'red';
	ctx.lineWidth = 1;
	ctx.beginPath();
	ctx.arc(centerX, centerY, 105, 0, Math.PI);
	ctx.stroke();

	ctx.strokeStyle = 'purple';
	ctx.beginPath();
	ctx.arc(centerX, centerY, 120, 0, Math.PI / 2);
	ctx.stroke();
}
