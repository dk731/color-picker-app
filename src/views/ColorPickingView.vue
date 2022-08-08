<template>
  <div
    :style="{
      background: 'red',
      width: '100%',
      height: '100%',
    }"
  ></div>
  <img
    v-for="(display, i) in colorPickData.displays"
    :style="{
      position: 'absolute',
      top: `${display.position.y}px`,
      left: `${display.position.x}px`,
      width: `${display.size.width}px`,
      height: `${display.size.height}px`,
    }"
    :key="i"
    :src="`data:image/png;base64,${display.buffer}`"
  />
</template>

<script setup lang="ts">
import {
  getCurrent,
  PhysicalSize,
  PhysicalPosition,
} from "@tauri-apps/api/window";
import router from "../router";
import useColorPickData from "../store/colorPickData";
import { register, unregisterAll } from "@tauri-apps/api/globalShortcut";

const colorPickData = useColorPickData();

const leftUpper = new PhysicalPosition(Infinity, Infinity);
const rightBottom = new PhysicalPosition(-Infinity, -Infinity);

colorPickData.displays?.forEach((display) => {
  if (leftUpper.x > display.position.x) leftUpper.x = display.position.x;
  if (leftUpper.y > display.position.y) leftUpper.y = display.position.y;
  if (rightBottom.x < display.position.x + display.size.width)
    rightBottom.x = display.position.x + display.size.width;
  if (rightBottom.y < display.position.y + display.size.height)
    rightBottom.y = display.position.y + display.size.height;
});
getCurrent().setSize(
  new PhysicalSize(rightBottom.x - leftUpper.x, rightBottom.y - leftUpper.y)
);
getCurrent().setPosition(leftUpper);

unregisterAll().then(() =>
  register("Esc", (short) => router.replace({ name: "MainApp" }))
);
// setTimeout(() => router.replace({ name: "MainApp" }), 5000);
</script>
