<template>
  <Page>
    <ActionBar>
      <Label text="canvas-babylon"/>
    </ActionBar>

    <GridLayout>
      <Button class="info" :text="message"/>
    </GridLayout>
  </Page>
</template>

<script lang="ts">
  import Vue from "nativescript-vue";
  import { DemoSharedCanvasBabylon } from '@demo/shared';
  import { } from '@nativescript/canvas-babylon';

  export default Vue.extend({
    computed: {
      message() {
        return "Test canvas-babylon";
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
