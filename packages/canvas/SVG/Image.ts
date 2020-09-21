import {SVGItem} from "./SVGItem";

const b64Extensions = {
	"/": "jpg",
	i: "png",
	R: "gif",
	U: "webp",
};

function b64WithoutPrefix(b64) {
	return b64.split(",")[1];
}

function getMIMEforBase64String(b64) {
	let input = b64;
	if (b64.includes(",")) {
		input = b64WithoutPrefix(b64);
	}
	const first = input.charAt(0);
	const mime = b64Extensions[first];
	if (!mime) {
		throw new Error("Unknown Base64 MIME type: " + b64);
	}
	return mime;
}

export class Image extends SVGItem {
	xlink: { href?: string } = {};
	href: string;

	handleValues(canvas?) {
		let ctx: any;
		if (canvas) {
			ctx = canvas.getContext('2d');
		} else {
			ctx = this._canvas.getContext('2d') as any;
		}
		let src;
		if (this.xlink && this.xlink.href) {
			src = this.xlink.href;
		}
		if (this.href) {
			src = this.href;
		}
		ctx.save();
		if (typeof src === 'string') {
			if (this.isBase64(src)) {
				const nativeImage = this.loadSrc(src);
				if (nativeImage) {
					ctx.drawImage(nativeImage, 0, 0, this.width as any, this.height as any);
				}
			}
		}
		ctx.restore();
	}

	isBase64(src) {
		return typeof src === "string" &&
			src.startsWith &&
			src.startsWith("data:");
	}

	loadSrc(src) {
		if (
			typeof src === "string" &&
			src.startsWith &&
			src.startsWith("data:")
		) {
			const base64result = src.split(",")[1];
			if (!base64result) {
				return null;
			}
			try {
				if (global.isIOS) {
					return UIImage.imageWithData(NSData.alloc().initWithBase64EncodedStringOptions(
						base64result,
						0
					));
				} else {
					const bytes = android.util.Base64.decode(
						base64result,
						android.util.Base64.DEFAULT
					);
					return android.graphics.BitmapFactory.decodeByteArray(bytes, 0, bytes.length);
				}
			} catch (error) {
				return null;
			}
		}
		return null;
	}
}
