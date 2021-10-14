<template>
  <Page>
    <ActionBar>
      <Label text="canvas-phaser"/>
    </ActionBar>

    <GridLayout>
      <Button class="info" :text="message"/>
    </GridLayout>
  </Page>
</template>

<script lang="ts">
  import Vue from "nativescript-vue";
  import { DemoSharedCanvasPhaser } from '@demo/shared';
  import { } from '@nativescript/canvas-phaser';

  export default Vue.extend({
    computed: {
      message() {
        return "Test canvas-phaser";
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
