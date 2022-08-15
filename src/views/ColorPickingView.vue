<template>
  <canvas
    class="color-pick-canvas"
    ref="baseCanvasRef"
    v-on:mousemove="onMouseMove"
    v-on:mouseenter="onMouseMove"
    v-on:mousewheel="onMouseZoom"
    v-on:contextmenu="onContextClick"
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
            left: `${Math.floor(
              Math.floor(magnifierHighlighPosition.x / magnifierGridSize) *
                magnifierGridSize
            )}px`,
            top: `${
              Math.floor(magnifierHighlighPosition.y / magnifierGridSize) *
              magnifierGridSize
            }px`,
            width: `${magnifierGridSize}px`,
            height: `${magnifierGridSize}px`,
            borderColor: `rgb(${magnifierHighlighColor.join(', ')})`,
          }"
        ></div>
      </div>
      <color-result :active-color="activeColor"></color-result>
      <flex-spacer />
    </div>
  </div>
  <div
    v-if="isPreciesActive"
    class="picker-events-overlay"
    v-on:mousemove="onPreciseMouseMove"
    v-on:contextmenu="onContextClick"
  ></div>
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
import { invoke } from "@tauri-apps/api/tauri";
import converter from "color-convert";

unregisterAll().then(() =>
  register("Esc", (short) => router.replace({ name: "MainApp" }))
);

// Define base magnifier sizes
const magnifierViewSize = ref<number>(11);

const baseCanvasRef = ref<HTMLCanvasElement>(null as any);

// On base canvas mouse move (as canvas is fullscreen -> global mouse listener)
const onMouseMove = ref<(e: MouseEvent) => void>();

const magnifierLensCanvasRef = ref<HTMLCanvasElement>(null as any);
const magnifierGridSize = ref<number>(0);
const magnifierPosition = ref<Coord2D>({ x: 0, y: 0 });

const magnifierHighlighPosition = ref<Coord2D>({ x: 0, y: 0 });
const magnifierHighlighColor = ref<RGB>([255, 255, 255]);

const activeColor = ref<RGB>([0, 0, 0]);

const preciseSensivity = ref<number>(0.3);
const isPreciesActive = ref<boolean>(false);

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

var baseCanvasCache: Uint8ClampedArray = null as any;

function updateActiveColor(currentPosition: Coord2D) {
  // Get current color
  const globalStartByte =
    (currentPosition.x + (currentPosition.y - 1) * baseCanvasSize.width - 1) *
    4; // Each pixel is 4 bytes (rgba)
  const currentActiveColor: RGB = [
    baseCanvasCache[globalStartByte + 0],
    baseCanvasCache[globalStartByte + 1],
    baseCanvasCache[globalStartByte + 2],
  ];
  activeColor.value = currentActiveColor;

  const hsvActive = converter.rgb.hsv(currentActiveColor);
  const activeLightness = hsvActive[2] > 50 ? 0 : 255;

  magnifierHighlighColor.value = [
    activeLightness,
    activeLightness,
    activeLightness,
  ];
}

onMounted(() => {
  // Setup base canvas for displaying all captured screenshots
  const baseCtx = baseCanvasRef.value.getContext("2d")!;

  baseCtx.canvas.width = baseCanvasSize.width;
  baseCtx.canvas.height = baseCanvasSize.height;

  // Setup magnifier canvas
  const magnifierEl = magnifierLensCanvasRef.value;
  const magnifierCtx = magnifierEl.getContext("2d")!;

  magnifierCtx.canvas.width = magnifierEl.offsetWidth;
  magnifierCtx.canvas.height = magnifierEl.offsetHeight;
  magnifierCtx.imageSmoothingEnabled = false; // Enable pixelated view

  magnifierHighlighPosition.value = {
    x: magnifierEl.offsetWidth / 2,
    y: magnifierEl.offsetHeight / 2,
  };
  magnifierGridSize.value =
    magnifierLensCanvasRef.value.offsetWidth / magnifierViewSize.value;

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
    baseCanvasCache = baseCtx.getImageData(
      0,
      0,
      baseCanvasSize.width,
      baseCanvasSize.height,
      {
        colorSpace: "srgb",
      }
    ).data;

    // Register base canvas mouse move events
    onMouseMove.value = (e: MouseEvent) => {
      // Do not move magnifier if precies mode is ON
      if (isPreciesActive.value) return;

      // Update magnifier position to move html elements
      magnifierPosition.value = { x: e.x, y: e.y };
      const magnifierRounded = Math.round(magnifierViewSize.value / 2);

      // Copy interested base rect to magnifier canvas
      magnifierCtx.drawImage(
        baseCtx.canvas,
        e.x - magnifierRounded,
        e.y - magnifierRounded,
        Math.round(magnifierViewSize.value),
        Math.round(magnifierViewSize.value),
        0,
        0,
        magnifierEl.offsetWidth,
        magnifierEl.offsetHeight
      );

      updateActiveColor(e as any);
    };
    console.log("Finished copying all screenshots");
  });
});

function onMouseZoom(e: WheelEvent) {
  var newValue = magnifierViewSize.value + Math.sign(e.deltaY) * 2;

  if (newValue < 1) newValue = 1;
  else if (newValue > 51) newValue = 51;

  magnifierViewSize.value = newValue;
  magnifierGridSize.value = magnifierLensCanvasRef.value.offsetWidth / newValue;
  onMouseMove.value!({ x: e.clientX, y: e.clientY } as any);
}

var basePosition: Coord2D = { x: 0, y: 0 };
function onContextClick(e: MouseEvent) {
  e.preventDefault();
  if (isPreciesActive.value) {
    invoke("unfreeze_mouse");
    magnifierHighlighPosition.value = {
      x: magnifierLensCanvasRef.value.offsetWidth / 2,
      y: magnifierLensCanvasRef.value.offsetHeight / 2,
    };
  } else {
    basePosition = { x: e.x, y: e.y };
    invoke("freeze_mouse", { currentPos: basePosition });
  }

  isPreciesActive.value = !isPreciesActive.value;
}

function onPreciseMouseMove(e: MouseEvent) {
  invoke("freeze_mouse_update");
  const currentMovement = { x: e.x - 300, y: e.y - 300 };
  const newPosition = {
    x:
      magnifierHighlighPosition.value.x +
      currentMovement.x * preciseSensivity.value,
    y:
      magnifierHighlighPosition.value.y +
      currentMovement.y * preciseSensivity.value,
  };

  if (newPosition.x < 0) newPosition.x = 0;
  if (newPosition.x >= 150) newPosition.x = 149;
  if (newPosition.y < 0) newPosition.y = 0;
  if (newPosition.y >= 150) newPosition.y = 149;

  magnifierHighlighPosition.value = newPosition;
  const gridSize = magnifierGridSize.value;
  const offsetDistance = {
    x:
      Math.floor(
        (2 * newPosition.x -
          magnifierLensCanvasRef.value.offsetWidth -
          gridSize) /
          (gridSize * 2)
      ) + 1,
    y:
      Math.floor(
        (2 * newPosition.y -
          magnifierLensCanvasRef.value.offsetHeight -
          gridSize) /
          (gridSize * 2)
      ) + 1,
  };

  updateActiveColor({
    x: basePosition.x + offsetDistance.x,
    y: basePosition.y + offsetDistance.y,
  });
}
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
  z-index: 100;
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

  width: 170px;
  height: 400px;

  background-color: rgb(130, 130, 130);

  display: flex;
  flex-direction: column;
  align-items: center;
}

.magnifier-panel-holder {
  width: 170px;
  height: 170px;
  min-width: 170px;
  min-height: 170px;

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

  border: 2px solid rgba(255, 255, 255, 1);
  box-sizing: border-box;

  z-index: 12;
}

.picker-events-overlay {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;

  z-index: 200;
}
</style>
