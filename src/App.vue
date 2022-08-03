<template>


  <div class="app-toolbar" data-tauri-drag-region>
    <div class="app-title">Color Picker</div>
    <flex-spacer></flex-spacer>
    <div>123</div>
  </div>

  <flex-spacer></flex-spacer>
  <img class="picker-button" src="./assets/picker_icon.svg"
    @click="startColorPicker" />
  <flex-spacer></flex-spacer>
  <div>123</div>
</template>

<script setup lang="ts">
import { availableMonitors, getCurrent, PhysicalPosition, PhysicalSize } from "@tauri-apps/api/window"
import FlexSpacer from "./components/FlexSpacer.vue"

const qwe = availableMonitors()

async function startColorPicker() {
  console.log("qwe")
}
// const currentWindow = getCurrent()

// await currentWindow.setFocus()
// await currentWindow.setPosition(new PhysicalPosition(0, 0));
// await currentWindow.setSize(new PhysicalSize(2560 * 2, 1440))

// setTimeout(() => {
//   currentWindow.setSize(new PhysicalSize(100, 100))
// }, 1500)

// console.log(qwe)
</script>


<style scoped>
.picker-button {
  width: 80%;
}

.app-title {
  pointer-events: none;
  padding-left: 4px;
}

.app-toolbar {
  display: flex;
  flex-direction: row;
  align-items: center;


  --toolbar-height: 26px;

  height: var(--toolbar-height);
  min-height: var(--toolbar-height);
  max-height: var(--toolbar-height);

  width: 100%;
}
</style>
