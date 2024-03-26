<template>
  <Page>
    <ActionBar>
      <Label text="canvas-svg"/>
    </ActionBar>

    <GridLayout>
      <Button class="info" :text="message"/>
    </GridLayout>
  </Page>
</template>

<script lang="ts">
  import Vue from "nativescript-vue";
  import { DemoSharedCanvasSvg } from '@demo/shared';
  import { } from '@nativescript/canvas-svg';

  export default Vue.extend({
    computed: {
      message() {
        return "Test canvas-svg";
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
