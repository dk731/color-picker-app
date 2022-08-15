<template>
  <router-view></router-view>
  <!-- <div
    :style="{ height: '20px', background: 'red' }"
    data-tauri-drag-region
  ></div>
  <color-result :active-color="[100, 200, 55]"></color-result> -->
</template>

<script setup lang="ts">
import {
  availableMonitors,
  getCurrent,
  PhysicalPosition,
  PhysicalSize,
} from "@tauri-apps/api/window";
import FlexSpacer from "./components/FlexSpacer.vue";
import ColorResult from "./components/ColorResult.vue";

// const qwe = availableMonitors()

// async function startColorPicker() {
//   console.log("qwe")
// }
// const currentWindow = getCurrent()

// await currentWindow.setFocus()
// await currentWindow.setPosition(new PhysicalPosition(0, 0));
// await currentWindow.setSize(new PhysicalSize(2560 * 2, 1440))

// setTimeout(() => {
//   currentWindow.setSize(new PhysicalSize(100, 100))
// }, 1500)

// console.log(qwe)
</script>
