import { Canvas } from '@nativescript/canvas';
import { Screen } from '@nativescript/core';
interface vec2 {
	x: number;
	y: number;
}
interface pp {
	theta: number;
	radius: number;
}
const cartesian2Canvas = (v: vec2, center: vec2) => {
	return { x: v.x + center.x, y: -1 * v.y + center.y };
};

const cartesian2Polar = (v: vec2) => {
	return { x: Math.atan2(v.y, v.x), y: Math.sqrt(v.x * v.x + v.y * v.y) };
};

const polar2Cartesian = (p: pp) => {
	return {
		x: p.radius * Math.cos(p.theta),
		y: p.radius * Math.sin(p.theta),
	};
};

const polar2Canvas = (p: pp, center: vec2) => {
	return cartesian2Canvas(polar2Cartesian(p), center);
};

export function breathe(canvas: Canvas) {
	const context = canvas.getContext('2d');
	const { width, height } = canvas as any;

	const c1 = '#61bea2';
	const c2 = '#529ca0';

	interface RingProps {
		index: number;
		progress: number;
		context: CanvasRenderingContext2D;
	}

	const mix = (value: number, x: number, y: number) => {
		return x * (1 - value) + y * value;
	};

	const Ring = ({ context, index, progress }: RingProps) => {
		context.save();
		context.beginPath();
		const R = width / 4;
		const center = { x: width / 2, y: height / 2 - 64 };
		const theta = (index * (2 * Math.PI)) / 6;

		const { x, y } = polar2Canvas({ theta, radius: progress * R }, { x: 0, y: 0 });
		const scale = mix(progress, 0.3, 1);

		//context.translate(x, y);
		context.scale(scale / Screen.mainScreen.scale, scale / Screen.mainScreen.scale);

		context.arc(x, y, R, 0, 2 * Math.PI);

		context.fillStyle = index % 2 ? c1 : c2;
		context.fill();
		context.restore();
	};

	const center = { x: width / 2, y: height / 2 - 64 };

	let progress = 0;

	const animator = android.animation.ValueAnimator.ofFloat([0, 1] as any);
	animator.setDuration(3000);

	animator.addUpdateListener(
		new android.animation.ValueAnimator.AnimatorUpdateListener({
			onAnimationUpdate(value) {
				progress = value.getAnimatedValue();
				context.save();
				context.clearRect(0, 0, width, height);
				context.fillStyle = 'rgb(36,43,56)';
				context.fillRect(0, 0, width, height);
				context.filter = 'blur(1px);';

				context.globalCompositeOperation = 'screen';

				context.translate(center.x, center.y);

				context.rotate(mix(progress, -Math.PI, 0));

				for (let i = 0; i < 6; i++) {
					Ring({ context: context as any, index: i, progress });
				}
				context.restore();
			},
		})
	);

	animator.setInterpolator(new android.view.animation.AccelerateDecelerateInterpolator());
	animator.setRepeatMode(android.view.animation.Animation.REVERSE);
	animator.setRepeatCount(android.animation.ValueAnimator.INFINITE);

	animator.start();
}
