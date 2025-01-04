export function drawImage(canvas) {
	canvas.width = canvas.clientWidth * window.devicePixelRatio;
	canvas.height = canvas.clientHeight * window.devicePixelRatio;
	const ctx = canvas.getContext('2d');
	ctx.scale(window.devicePixelRatio, window.devicePixelRatio);

	ctx.translate(0, 100);
	const image = document.createElement('img');

	image.addEventListener('load', (e) => {
		ctx.drawImage(image, 33, 71, 104, 124, 21, 20, 87, 104);
	});

	image.src = 'https://mdn.github.io/shared-assets/images/examples/rhino.jpg';
}
