import { Canvas, GPUDevice, GPUAdapter } from '@nativescript/canvas';

export async function run(canvas: Canvas) {
	const adapter: GPUAdapter = await (navigator as any).gpu?.requestAdapter();

	const features = adapter.features.values();
	Promise.all([
		adapter.requestDevice(),
		adapter.requestDevice().then((device) => {
			return adapter.requestDevice({
				requiredFeatures: Array.from(features),
			});
		}),
		adapter.requestDevice(),
		adapter.requestDevice(),
		adapter.requestDevice(),
	]).then((devices) => {
		console.log('first', Array.from(devices[0].features.values()), Array.from(devices[1].features.values()), Array.from(devices[2].features.values()), Array.from(devices[3].features.values()), Array.from(devices[4].features.values()));
	});

	adapter
		.requestDevice()
		.then((device) => {
			return adapter.requestDevice({
				requiredFeatures: Array.from(features),
			});
		})
		.then((device) => {
			console.log('other device', Array.from(device.features.values()));
		});

	// const devicePixelRatio = window.devicePixelRatio;

	// canvas.width = canvas.clientWidth * devicePixelRatio;
	// canvas.height = canvas.clientHeight * devicePixelRatio;

	// const context = canvas.getContext('webgpu') as never as GPUCanvasContext;

	// const presentationFormat = navigator.gpu.getPreferredCanvasFormat();

	// context.configure({
	// 	device,
	// 	format: presentationFormat,
	// });
}
