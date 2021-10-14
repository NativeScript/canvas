<template>
  <Page>
    <ActionBar>
      <Label text="canvas-three"/>
    </ActionBar>

    <GridLayout>
      <Button class="info" :text="message"/>
    </GridLayout>
  </Page>
</template>

<script lang="ts">
  import Vue from "nativescript-vue";
  import { DemoSharedCanvasThree } from '@demo/shared';
  import { } from '@nativescript/canvas-three';

  export default Vue.extend({
    computed: {
      message() {
        return "Test canvas-three";
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
