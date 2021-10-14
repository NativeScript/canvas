<template>
  <Page>
    <ActionBar>
      <Label text="canvas-media"/>
    </ActionBar>

    <GridLayout>
      <Button class="info" :text="message"/>
    </GridLayout>
  </Page>
</template>

<script lang="ts">
  import Vue from "nativescript-vue";
  import { DemoSharedCanvasMedia } from '@demo/shared';
  import { } from '@nativescript/canvas-media';

  export default Vue.extend({
    computed: {
      message() {
        return "Test canvas-media";
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
