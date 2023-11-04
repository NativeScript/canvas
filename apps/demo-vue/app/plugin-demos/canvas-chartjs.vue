<template>
  <Page>
    <ActionBar>
      <Label text="canvas-chartjs"/>
    </ActionBar>

    <GridLayout>
      <Button class="info" :text="message"/>
    </GridLayout>
  </Page>
</template>

<script lang="ts">
  import Vue from "nativescript-vue";
  import { DemoSharedCanvasChartjs } from '@demo/shared';
  import { } from '@nativescript/canvas-chartjs';

  export default Vue.extend({
    computed: {
      message() {
        return "Test canvas-chartjs";
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
