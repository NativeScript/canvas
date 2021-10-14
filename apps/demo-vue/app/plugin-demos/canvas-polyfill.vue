<template>
  <Page>
    <ActionBar>
      <Label text="canvas-polyfill"/>
    </ActionBar>

    <GridLayout>
      <Button class="info" :text="message"/>
    </GridLayout>
  </Page>
</template>

<script lang="ts">
  import Vue from "nativescript-vue";
  import { DemoSharedCanvasPolyfill } from '@demo/shared';
  import { } from '@nativescript/canvas-polyfill';

  export default Vue.extend({
    computed: {
      message() {
        return "Test canvas-polyfill";
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
