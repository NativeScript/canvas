<template>
  <Page>
    <ActionBar>
      <Label text="canvas-phaser-ce"/>
    </ActionBar>

    <GridLayout>
      <Button class="info" :text="message"/>
    </GridLayout>
  </Page>
</template>

<script lang="ts">
  import Vue from "nativescript-vue";
  import { DemoSharedCanvasPhaserCe } from '@demo/shared';
  import { } from '@nativescript/canvas-phaser-ce';

  export default Vue.extend({
    computed: {
      message() {
        return "Test canvas-phaser-ce";
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
