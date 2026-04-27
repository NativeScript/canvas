<template>
  <Page>
    <ActionBar>
      <Label text="audio-context"/>
    </ActionBar>

    <GridLayout>
      <Button class="info" :text="message"/>
    </GridLayout>
  </Page>
</template>

<script lang="ts">
  import Vue from "nativescript-vue";
  import { DemoSharedAudioContext } from '@demo/shared';
  import { } from '@nativescript/audio-context';

  export default Vue.extend({
    computed: {
      message() {
        return "Test audio-context";
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
