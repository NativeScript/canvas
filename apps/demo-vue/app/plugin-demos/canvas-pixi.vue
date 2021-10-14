<template>
  <Page>
    <ActionBar>
      <Label text="canvas-pixi"/>
    </ActionBar>

    <GridLayout>
      <Button class="info" :text="message"/>
    </GridLayout>
  </Page>
</template>

<script lang="ts">
  import Vue from "nativescript-vue";
  import { DemoSharedCanvasPixi } from '@demo/shared';
  import { } from '@nativescript/canvas-pixi';

  export default Vue.extend({
    computed: {
      message() {
        return "Test canvas-pixi";
      }
    }
  });
</script>

<style scoped lang="scss">

  .info {
    font-size: 20;
    horizontal-align: center;
    vertical-align: center;
  }
</style>
