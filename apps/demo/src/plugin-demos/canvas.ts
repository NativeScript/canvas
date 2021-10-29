import { EventData, Page, WebView } from '@nativescript/core';
import { DemoSharedCanvas } from '@demo/shared';

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
}
