<template>
  <canvas
    class="color-pick-canvas"
    ref="baseCanvasRef"
    v-on:mousemove="onMouseMove"
    v-on:mouseenter="onMouseMove"
  ></canvas>
  <div
    class="picker-magnifier"
    :style="{
      left: `${magnifierPosition.x}px`,
      top: `${magnifierPosition.y}px`,
    }"
  >
    <icon class="magnifier-cursor" icon="radix-icons:crosshair-2"></icon>
    <div class="magnifier-panel">
      <div class="magnifier-panel-holder">
        <canvas class="magnifier-lens" ref="magnifierLensCanvasRef"></canvas>
        <div
          class="magnifier-lens-grid"
          :style="{
            backgroundSize: `${magnifierGridSize}px ${magnifierGridSize}px`,
          }"
        ></div>
        <div
          class="magnifier-active-highligh"
          :style="{
            width: `${magnifierGridSize}px`,
            height: `${magnifierGridSize}px`,
          }"
        ></div>
      </div>
      <div class="active-color-result">
        <div>Current Color:</div>
        <flex-spacer />
        <div
          class="magnifier-result-color-display"
          :style="{
            backgroundColor: `rgb(${activeColor[0]}, ${activeColor[1]}, ${activeColor[2]})`,
          }"
        />
      </div>
      <color-result :active-color="activeColor"></color-result>
      <flex-spacer />
    </div>
  </div>
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
import { onMounted, ref } from "vue";
import { Icon } from "@iconify/vue";
import FlexSpacer from "../components/FlexSpacer.vue";
import ColorResult from "../components/ColorResult.vue";
import { RGB } from "color-convert/conversions";

unregisterAll().then(() =>
  register("Esc", (short) => router.replace({ name: "MainApp" }))
);

// Define base magnifier sizes
const BASE_MAGNIFIER_PIXELS = 11;

const baseCanvasRef = ref<HTMLCanvasElement>(null as any);

// On base canvas mouse move (as canvas is fullscreen -> global mouse listener)
const onMouseMove = ref<(e: MouseEvent) => void>();

const magnifierLensCanvasRef = ref<HTMLCanvasElement>(null as any);
const magnifierGridSize = ref<number>(0);
const magnifierPosition = ref<{ x: number; y: number }>({ x: 0, y: 0 });

const activeColor = ref<RGB>([0, 0, 0]);

// const colorPickData = useColorPickData();
const colorPickData = {
  displays: [
    {
      id: "test1",
      position: { x: 0, y: 0 },
      size: { width: 2560, height: 1440 },
      buffer: "public/disp_1.png",
    },
    {
      id: "test2",
      position: { x: 2560, y: 0 },
      size: { width: 2560, height: 1440 },
      buffer: "public/disp_2.png",
    },
  ],
};

const leftUpper = new PhysicalPosition(Infinity, Infinity);
const rightBottom = new PhysicalPosition(-Infinity, -Infinity);
const tauriScreen = getCurrent();
colorPickData.displays?.forEach((display) => {
  if (leftUpper.x > display.position.x) leftUpper.x = display.position.x;
  if (leftUpper.y > display.position.y) leftUpper.y = display.position.y;
  if (rightBottom.x < display.position.x + display.size.width)
    rightBottom.x = display.position.x + display.size.width;
  if (rightBottom.y < display.position.y + display.size.height)
    rightBottom.y = display.position.y + display.size.height;
});
const baseCanvasSize = {
  width: rightBottom.x - leftUpper.x,
  height: rightBottom.y - leftUpper.y,
};

// Set screen size to be fullscreen on all displays
tauriScreen.setSize(
  new PhysicalSize(baseCanvasSize.width, baseCanvasSize.height)
);
tauriScreen.setPosition(leftUpper);

onMounted(() => {
  // Setup base canvas for displaying all captured screenshots
  const baseCtx = baseCanvasRef.value.getContext("2d")!;
  const canvasWidth = rightBottom.x - leftUpper.x;
  const canvasHeight = rightBottom.y - leftUpper.y;

  baseCtx.canvas.width = canvasWidth;
  baseCtx.canvas.height = canvasHeight;

  // Setup magnifier canvas
  const magnifierEl = magnifierLensCanvasRef.value;
  const magnifierCtx = magnifierEl.getContext("2d")!;

  const centerPosition = Math.round(magnifierEl.offsetWidth / 2);

  magnifierCtx.canvas.width = magnifierEl.offsetWidth;
  magnifierCtx.canvas.height = magnifierEl.offsetHeight;
  magnifierCtx.imageSmoothingEnabled = false; // Enable pixelated view

  magnifierGridSize.value = magnifierEl.offsetWidth / BASE_MAGNIFIER_PIXELS;

  // Promise to wait for all screens to render to base canvas
  Promise.all(
    colorPickData.displays.map(async (item, i) => {
      const tmpImage = new Image();
      const syncPromise = new Promise<void>((resolve) => {
        tmpImage.onload = () => {
          baseCtx.drawImage(tmpImage, item.position.x, item.position.y);
          resolve();
        };
        tmpImage.src = item.buffer;
      });

      await syncPromise;
    })
  ).then(() => {
    // Get base canvas in bytes array form, to read pixel values
    const baseCanvasCache = baseCtx.getImageData(
      0,
      0,
      canvasWidth,
      canvasHeight,
      { colorSpace: "srgb" }
    ).data;

    // Register base canvas mouse move events
    onMouseMove.value = (e: MouseEvent) => {
      // Update magnifier position to move html elements
      magnifierPosition.value = { x: e.x, y: e.y };

      // Copy interested base rect to magnifier canvas
      magnifierCtx.drawImage(
        baseCtx.canvas,
        Math.ceil(e.x - BASE_MAGNIFIER_PIXELS / 2),
        Math.ceil(e.y - BASE_MAGNIFIER_PIXELS / 2),
        BASE_MAGNIFIER_PIXELS,
        BASE_MAGNIFIER_PIXELS,
        0,
        0,
        magnifierEl.offsetWidth,
        magnifierEl.offsetHeight
      );

      // Get current color
      const globalStartByte = (e.x + e.y * canvasWidth) * 4; // Each pixel is 4 bytes (rgba)
      activeColor.value = [
        baseCanvasCache[globalStartByte + 0],
        baseCanvasCache[globalStartByte + 1],
        baseCanvasCache[globalStartByte + 2],
      ];
    };
    console.log("Finished copying all screenshots");
  });
});
</script>

<style scoped>
.color-pick-canvas {
  width: 100%;
  height: 100%;

  padding: 0;
  margin: 0;
  cursor: none;
}

.picker-magnifier,
.picker-magnifier * {
  cursor: none;
  pointer-events: none;
}

.picker-magnifier {
  --crosshair-size: 30px;

  position: absolute;
  z-index: 999;
  cursor: none;
}

.magnifier-cursor {
  position: relative;
  top: 0;
  left: 0;
  transform: translate(-50%, -50%);
  width: var(--crosshair-size);
  height: var(--crosshair-size);
}

.magnifier-panel {
  position: absolute;
  padding: 10px;

  top: 0;
  left: calc(var(--crosshair-size) / 2 + 5px);

  width: 150px;
  height: 400px;

  background-color: rgb(130, 130, 130);

  display: flex;
  flex-direction: column;
  align-items: center;
}

.magnifier-panel-holder {
  width: 150px;
  height: 150px;
  min-width: 150px;
  min-height: 150px;

  border-radius: 5px;
  position: relative;

  overflow: hidden;

  margin-bottom: 10px;
}

.magnifier-lens {
  position: absolute;
  left: 0;
  top: 0;

  width: 100%;
  height: 100%;

  z-index: 10;
}

.magnifier-lens-grid {
  --magnifier-lens-grid-color: rgba(255, 255, 255, 0.2);

  position: absolute;

  /* Set -1 to hide first grid line */
  left: -1px;
  top: -1px;

  width: 100%;
  height: 100%;

  background-image: repeating-linear-gradient(
      var(--magnifier-lens-grid-color) 0 1px,
      transparent 1px 100%
    ),
    repeating-linear-gradient(
      90deg,
      var(--magnifier-lens-grid-color) 0 1px,
      transparent 1px 100%
    );

  z-index: 11;
}

.magnifier-active-highligh {
  position: absolute;
  left: 50%;
  top: 50%;
  transform: translate(-50%, -50%);

  border: 2px solid rgba(255, 255, 255, 1);
  box-sizing: border-box;

  z-index: 12;
}
</style>
