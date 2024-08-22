import { Screen } from '@nativescript/core';
import { DemoSharedBase } from '../utils';
import { Svg } from '@nativescript/canvas-svg';
require('@nativescript/canvas-polyfill');
export class DemoSharedCanvasSvg extends DemoSharedBase {
	private svg: Svg;
	private svg2: Svg;
	private svg3: Svg;
	private svg4: Svg;

	viewLoaded(args) {
		const view = args.object;
		const svg = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
		svg.setAttribute('width', '500');
		svg.setAttribute('height', '500');

		/*const text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
		// text.setAttribute('x', '0');
		// text.setAttribute('y', '0');
		text.setAttribute('font-size', '10');
		text.setAttribute('fill', 'red');
		text.setAttribute('stroke', 'blue');

		const textContent = document.createTextNode('I love SVG!');
		text.appendChild(textContent);

		svg.appendChild(text);

		*/

		(<any>svg).nativeElement.src = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100" height = "256" width = "256">
		<path d="M30,1h40l29,29v40l-29,29h-40l-29-29v-40z" stroke="#000" fill="none"/>
		<path d="M31,3h38l28,28v38l-28,28h-38l-28-28v-38z" fill="#a23"/>
		<text x="50" y="68" font-size="48" fill="#FFF" text-anchor="middle">Stop</text>
		</svg>`;

		/*

		const circle = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
		circle.setAttribute('cx', '100');
		circle.setAttribute('cy', '100');
		circle.setAttribute('r', '50');
		circle.setAttribute('fill', 'gold');
		circle.id = 'circle';
		svg.appendChild(circle);

		const rect = document.createElementNS('http://www.w3.org/2000/svg', 'rect');
		rect.setAttribute('x', '0');
		rect.setAttribute('y', '200');
		rect.setAttribute('width', '150');
		rect.setAttribute('height', '150');
		rect.setAttribute('stroke', 'green');
		rect.setAttribute('fill', 'none');
		rect.id = 'rect';
		svg.appendChild(rect);

		const image = document.createElementNS('http://www.w3.org/2000/svg', 'image');
		image.setAttribute('xlink:href', 'https://staticg.sportskeeda.com/editor/2023/06/6a942-16869513670522-1920.jpg');
		image.setAttribute('x', '400');
		image.setAttribute('y', '300');
		image.setAttribute('width', '500');
		image.setAttribute('height', '500');
		image.setAttribute('decoding', 'sync');
		svg.appendChild(image);

		const image2 = document.createElementNS('http://www.w3.org/2000/svg', 'image');
		image2.setAttribute('xlink:href', 'https://source.unsplash.com/1600x900/?nature');
		image2.setAttribute('x', '100');
		image2.setAttribute('y', '600');
		image2.setAttribute('width', '500');
		image2.setAttribute('height', '500');
		image2.setAttribute('decoding', 'sync');
		svg.appendChild(image2);

		const path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
		path.setAttribute('d', 'M150 0 L75 200 L225 200 Z');
		svg.appendChild(path);

		const ellipse = document.createElementNS('http://www.w3.org/2000/svg', 'ellipse');
		ellipse.setAttribute('cx', '500');
		ellipse.setAttribute('cy', '80');
		ellipse.setAttribute('rx', '100');
		ellipse.setAttribute('ry', '50');
		ellipse.setAttribute('style', 'fill:yellow;stroke:purple;stroke-width:2');
		svg.appendChild(ellipse);

		const line = document.createElementNS('http://www.w3.org/2000/svg', 'line');
		line.setAttribute('x1', '0');
		line.setAttribute('y1', '0');
		line.setAttribute('x2', '200');
		line.setAttribute('y2', '200');
		line.setAttribute('style', 'stroke:rgb(255,0,0);stroke-width:2');
		svg.appendChild(line);

		const polygon = document.createElementNS('http://www.w3.org/2000/svg', 'polygon');
		polygon.setAttribute('points', '200,10 250,190 160,210');
		polygon.setAttribute('style', 'fill:lime;stroke:purple;stroke-width:1');
		svg.appendChild(polygon);

		const polyline = document.createElementNS('http://www.w3.org/2000/svg', 'polyline');
		polyline.setAttribute('points', '20,20 40,25 60,40 80,120 120,140 200,180');
		polyline.setAttribute('style', 'fill:none;stroke:pink;stroke-width:3');
		svg.appendChild(polyline);

		const text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
		text.setAttribute('x', '0');
		text.setAttribute('y', '0');
		text.setAttribute('font-size', '100px');

		const textContent = document.createTextNode('I love SVG!');
		text.appendChild(textContent);

		svg.appendChild(text);

		const g = document.createElementNS('http://www.w3.org/2000/svg', 'g');

		const path1 = document.createElementNS('http://www.w3.org/2000/svg', 'path');
		path1.setAttribute('d', 'M5 20 l215 0');
		path1.setAttribute('stroke', 'red');

		const path2 = document.createElementNS('http://www.w3.org/2000/svg', 'path');
		path2.setAttribute('d', 'M5 40 l215 0');
		path2.setAttribute('stroke', 'black');

		const path3 = document.createElementNS('http://www.w3.org/2000/svg', 'path');
		path3.setAttribute('d', 'M5 60 l215 0');
		path3.setAttribute('stroke', 'blue');

		g.append(path1, path2, path3);

		svg.appendChild(g);

		const useCircle = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
		useCircle.setAttribute('cx', '5');
		useCircle.setAttribute('cy', '5');
		useCircle.setAttribute('r', '4');
		useCircle.setAttribute('fill', 'blue');
		useCircle.id = 'myCircle';
		svg.appendChild(useCircle);

		const use1 = document.createElementNS('http://www.w3.org/2000/svg', 'use');
		use1.setAttribute('href', '#myCircle');
		use1.setAttribute('x', '10');
		use1.setAttribute('fill', 'blue');

		svg.appendChild(use1);

		const use2 = document.createElementNS('http://www.w3.org/2000/svg', 'use');
		use2.setAttribute('href', '#myCircle');
		use2.setAttribute('x', '20');
		use2.setAttribute('fill', 'white');
		use2.setAttribute('stroke', 'red');

		svg.appendChild(use2);

		*/

		const svgView: Svg = (<any>svg).nativeElement;
		svgView.rowSpan = 2;
		svgView.colSpan = 2;
		view.addChild(svgView);
		// svgView.src = 'https://upload.wikimedia.org/wikipedia/commons/8/85/Australian_Census_2011_demographic_map_-_Australia_by_SLA_-_BCP_field_0001_Total_Persons_Males.svg';
	}

	testIt() {
		console.log('test canvas-svg!');
	}

	svgViewLoaded(args) {
		const view = args.object;
		this.drawSvg(this.svg, view.id);
	}

	svg2ViewLoaded(args) {
		this.svg2 = args.object;
		console.log('svg2 ready');
		this.set('src2', 'http://thenewcode.com/assets/images/thumbnails/homer-simpson.svg');
	}

	drawTransformMatrixSvg() {
		this.set(
			'src',
			`<svg viewBox='0 0 200 200' xmlns='http://www.w3.org/2000/svg'>
		<rect x='10' y='10' width='30' height='20' fill='green' />

		<!--
		In the following example we are applying the matrix:
		[a c e]    [3 -1 30]
		[b d f] => [1  3 40]
		[0 0 1]    [0  0  1]

		which transform the rectangle as such:

		top left corner: oldX=10 oldY=10
		newX = a * oldX + c * oldY + e = 3 * 10 - 1 * 10 + 30 = 50
		newY = b * oldX + d * oldY + f = 1 * 10 + 3 * 10 + 40 = 80

		top right corner: oldX=40 oldY=10
		newX = a * oldX + c * oldY + e = 3 * 40 - 1 * 10 + 30 = 140
		newY = b * oldX + d * oldY + f = 1 * 40 + 3 * 10 + 40 = 110

		bottom left corner: oldX=10 oldY=30
		newX = a * oldX + c * oldY + e = 3 * 10 - 1 * 30 + 30 = 30
		newY = b * oldX + d * oldY + f = 1 * 10 + 3 * 30 + 40 = 140

		bottom right corner: oldX=40 oldY=30
		newX = a * oldX + c * oldY + e = 3 * 40 - 1 * 30 + 30 = 120
		newY = b * oldX + d * oldY + f = 1 * 40 + 3 * 30 + 40 = 170
		-->
		<rect x='10' y='10' width='30' height='20' fill='red'
		transform='matrix(3 1 -1 3 30 40)' />
		</svg>`
		);
	}

	drawTransformTranslateSvg() {
		/// translate transform

		this.set(
			'src',
			`<svg viewBox='0 0 100 100' xmlns='http://www.w3.org/2000/svg'>
  <!-- No translation -->
  <rect x='5' y='5' width='40' height='40' fill='green' />

  <!-- Horizontal translation -->
  <rect x='5' y='5' width='40' height='40' fill='blue'
        transform='translate(50)' />

  <!-- Vertical translation -->
  <rect x='5' y='5' width='40' height='40' fill='red'
        transform='translate(0 50)' />

  <!-- Both horizontal and vertical translation -->
  <rect x='5' y='5' width='40' height='40' fill='yellow'
         transform='translate(50,50)' />
</svg>
			`
		);
	}

	drawTransformScaleSvg() {
		this.set(
			'src',
			`
				<svg viewBox='-50 -50 100 100' xmlns='http://www.w3.org/2000/svg'>
				  <!-- uniform scale -->
				  <circle cx='0' cy='0' r='10' fill='red'
				          transform='scale(4)' />

				  <!-- vertical scale -->
				  <circle cx='0' cy='0' r='10' fill='yellow'
				          transform='scale(1,4)' />

				  <!-- horizontal scale -->
				  <circle cx='0' cy='0' r='10' fill='pink'
				          transform='scale(4,1)' />

				  <!-- No scale -->
				  <circle cx='0' cy='0' r='10' fill='black' />
				</svg>
			`
		);
	}

	drawTransformRotateSvg() {
		this.set(
			'src',
			`
			<svg viewBox='-12 -2 34 14' xmlns='http://www.w3.org/2000/svg'>
			  <rect x='0' y='0' width='10' height='10' />

			  <!-- rotation is done around the point 0,0 -->
			  <rect x='0' y='0' width='10' height='10' fill='red'
			        transform='rotate(100)' />

			  <!-- rotation is done around the point 10,10 -->
			  <rect x='0' y='0' width='10' height='10' fill='green'
			        transform='rotate(100,10,10)' />
			</svg>
		`
		);
	}

	drawTransformSkewX() {
		this.set(
			'src',
			`
			<svg viewBox='-5 -5 10 10' xmlns='http://www.w3.org/2000/svg'>
			  <rect x='-3' y='-3' width='6' height='6' />

			  <rect x='-3' y='-3' width='6' height='6' fill='red'
			        transform='skewX(30)' />
			</svg>
		`
		);
	}

	drawTransformSkewY() {
		this.set(
			'src',
			`
			<svg viewBox='-5 -5 10 10' xmlns='http://www.w3.org/2000/svg'>
			  <rect x='-3' y='-3' width='6' height='6' />

			  <rect x='-3' y='-3' width='6' height='6' fill='red'
			        transform='skewY(30)' />
			</svg>
		`
		);
	}

	drawSvg(args: Svg, id: string) {
		switch (id) {
			case '1':
				this.set('src1', 'https://upload.wikimedia.org/wikipedia/commons/8/85/Australian_Census_2011_demographic_map_-_Australia_by_SLA_-_BCP_field_0001_Total_Persons_Males.svg');
				break;
			case '2':
				this.set('src2', 'https://upload.wikimedia.org/wikipedia/commons/4/4c/The_Hague%2C_Netherlands%2C_the_old_city_center.svg');
				break;
			case '3':
				this.set('src3', 'https://upload.wikimedia.org/wikipedia/commons/7/7c/Map_of_the_world_by_the_US_Gov_as_of_2016_no_legend.svg');
				break;
			case '4':
				this.set('src4', 'https://upload.wikimedia.org/wikipedia/commons/9/9d/The_Rhodopes_on_The_Paths_Of_Orpheus_And_Eurydice_Project_Map.svg');
				break;
		}
		//this.drawTransformSkewY();
		//this.set('src','https://dev.w3.org/SVG/tools/svgweb/samples/svg-files/car.svg');
		//this.set('src','http://thenewcode.com/assets/images/thumbnails/homer-simpson.svg');
		//this.set('src', 'https://upload.wikimedia.org/wikipedia/commons/a/a0/Location_map_San_Francisco_Bay_Area.svg');
		//this.set('src','https://upload.wikimedia.org/wikipedia/commons/4/4c/The_Hague%2C_Netherlands%2C_the_old_city_center.svg');
		//this.set('src', 'https://upload.wikimedia.org/wikipedia/commons/6/6c/Trajans-Column-lower-animated.svg');
		//this.set('src', 'https://upload.wikimedia.org/wikipedia/commons/7/7c/Map_of_the_world_by_the_US_Gov_as_of_2016_no_legend.svg');
		//this.set('src', 'https://upload.wikimedia.org/wikipedia/commons/b/b6/Moldova_%281483%29-en.svg');
		//this.set('src', 'https://upload.wikimedia.org/wikipedia/commons/9/95/Kaiserstandarte_Version1.svg');
		//this.set('src', 'https://upload.wikimedia.org/wikipedia/commons/f/ff/1_42_polytope_7-cube.svg');
		//this.set('src', 'https://upload.wikimedia.org/wikipedia/commons/1/1c/KINTETSU23000_20140424A.svg');
		//this.set('src', 'https://raw.githubusercontent.com/RazrFalcon/resvg/7b26adbcc9698dcca687214c84d216794f60a5be/tests/svg/e-radialGradient-013.svg');
		//this.set('src','https://upload.wikimedia.org/wikipedia/commons/c/c1/Propane_flame_contours-en.svg')
		//this.set('src','https://upload.wikimedia.org/wikipedia/commons/9/9d/The_Rhodopes_on_The_Paths_Of_Orpheus_And_Eurydice_Project_Map.svg')
		//this.set('src', 'https://upload.wikimedia.org/wikipedia/commons/7/7c/Map_of_the_world_by_the_US_Gov_as_of_2016_no_legend.svg');
		//this.set('src','https://upload.wikimedia.org/wikipedia/commons/7/78/61453-Planeta_berria_2006an.svg')
		// https://upload.wikimedia.org/wikipedia/commons/6/61/Figure_in_Manga_style.svg
		// https://upload.wikimedia.org/wikipedia/commons/a/a0/Plan_des_Forts_de_Lyon_premi%C3%A8re_ceinture_-_OSM.svg

		// 		this.set('src1', `
		// 		<svg viewBox="0 0 100 100">
		//   <!-- No translation -->
		//   <rect x="5" y="5" width="40" height="40" fill="green" />

		//   <!-- Horizontal translation -->
		//   <rect x="5" y="5" width="40" height="40" fill="blue"
		// 		transform="translate(50)" />

		//   <!-- Vertical translation -->
		//   <rect x="5" y="5" width="40" height="40" fill="red"
		// 		transform="translate(0 50)" />

		//   <!-- Both horizontal and vertical translation -->
		//   <rect x="5" y="5" width="40" height="40" fill="yellow"
		// 		 transform="translate(50,50)" />
		// </svg>
		// 		`)

		/*
		const circle = new Svg.Circle();
		circle.cx = 100;
		circle.cy = 100;
		circle.r = 50;
		circle.fill = 'gold';
		circle.id = 'circle';
		args.addChild(circle);

		const rect = new Svg.Rect();
		rect.x = 0;
		rect.y = 200;
		rect.width = 300;
		rect.height = 300;
		rect.stroke = 'green';
		rect.fill = 'black';
		rect.id = 'rect';
		args.addChild(rect);

		const image = new Svg.Image();
		image.href = 'https://source.unsplash.com/1600x900/?water';
		image.x = 0;
		image.y = 600;
		image.width = 500;
		image.height = 500;
		args.addChild(image);

		const image2 = new Svg.Image();
		image2.href = 'https://source.unsplash.com/1600x900/?nature';
		image2.x = 600;
		image2.y = 600;
		image2.width = 500;
		image2.height = 500;
		args.addChild(image2);

		const path = new Svg.Path();
		path.d = "M150 0 L75 200 L225 200 Z";
		args.addChild(path);

		const ellipse = new Svg.Ellipse();
		ellipse.cx = 500;
		ellipse.cy = 80;
		ellipse.rx = 100;
		ellipse.ry = 50;
		ellipse.setInlineStyle('fill:yellow;stroke:purple;stroke-width:2');
		args.addChild(ellipse);

		const line = new Svg.Line();
		line.x1 = 0;
		line.y1 = 0;
		line.x2 = 200;
		line.y2 = 200;
		line.setInlineStyle('stroke:rgb(255,0,0);stroke-width:2');
		args.addChild(line);


		const polygon = new Svg.Polygon();
		polygon.points = "200,10 250,190 160,210";
		polygon.setInlineStyle('fill:lime;stroke:purple;stroke-width:1');
		args.addChild(polygon);


		const polyline = new Svg.Polyline();
		polyline.points = "20,20 40,25 60,40 80,120 120,140 200,180";
		polyline.setInlineStyle("fill:none;stroke:black;stroke-width:3");
		args.addChild(polyline);

		const text = new Svg.Text();
		text.text = "I love SVG!";
		text.x = 0;
		text.y = 15;
		args.addChild(text);
		const g = new Svg.G();

		const path1 = new Svg.Path();
		path1.d = "M5 20 l215 0";
		path1.stroke = "red";

		const path2 = new Svg.Path();
		path2.d = "M5 40 l215 0";
		path2.stroke = "black";

		const path3 = new Svg.Path();
		path3.d = "M5 60 l215 0";
		path3.stroke = "blue";
		g.addChildren(path1, path2, path3);
		args.addChild(g);
		*/
	}
}
