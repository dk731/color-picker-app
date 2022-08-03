<template>

  <div class="main-app">
    <div class="app-toolbar" data-tauri-drag-region>
      123
    </div>
  </div>
</template>

<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import HelloWorld from './components/HelloWorld.vue'
import { availableMonitors, getCurrent, PhysicalPosition, PhysicalSize } from "@tauri-apps/api/window"


const qwe = availableMonitors()
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
.main-app {
  display: flex;
  flex-direction: column;
}

.app-toolbar {
  height: 36px;
  min-height: 36px;
  max-height: 36px;

  background: green;
}
</style>
