<template>
  <canvas
    class="color-pick-canvas"
    ref="baseCanvasRef"
    v-on:mousemove="onMouseMove"
    v-on:mouseenter="onMouseMove"
    v-on:mousewheel="onMouseZoom"
    v-on:contextmenu="onContextClick"
    v-on:click="onPickFinish"
    :style="{ cursor: isColorFinish ? 'default' : 'none' }"
  ></canvas>
  <div
    class="picker-magnifier"
    :style="{
      left: `${magnifierPosition.x}px`,
      top: `${magnifierPosition.y}px`,
      pointerEvents: isColorFinish ? 'all' : 'none',
      cursor: isColorFinish ? 'default' : 'none',
    }"
  >
    <icon class="magnifier-cursor" icon="radix-icons:crosshair-2"></icon>
    <svg
      style="opacity: 0; pointer-events: none"
      viewBox="0 0 60 50"
      width="60"
      class="movement-path"
    >
      <path
        id="panel-position-path"
        d="M52,0H8C3.6,0,0,3.6,0,8v34c0,4.4,3.6,8,8,8h44c4.4,0,8-3.6,8-8V8C60,3.6,56.4,0,52,0L52,0z"
      />
    </svg>
    <div class="magnifier-panel reverse-container" ref="magnifierPanelRef">
      <div class="magnifier-panel-holder reversable-column">
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
              magnifierHighlighPosition.x * magnifierGridSize
            )}px`,
            top: `${Math.floor(
              magnifierHighlighPosition.y * magnifierGridSize
            )}px`,
            width: `${magnifierGridSize + 1}px`,
            height: `${magnifierGridSize + 1}px`,
            transform: 'translate(-50%, -50%)',
            borderColor: `rgb(${magnifierHighlighColor.join(', ')})`,
          }"
        ></div>
      </div>
      <color-result
        :active-color="activeColor"
        @on-mounted="qwe"
      ></color-result>
      <flex-spacer />
    </div>
  </div>
  <div
    v-if="isPreciesActive"
    class="picker-events-overlay"
    v-on:mousemove="onPreciseMouseMove"
    v-on:contextmenu="onContextClick"
    v-on:click="onPickFinish"
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
import gsap from "gsap";

unregisterAll().then(() =>
  register("Esc", (short) => router.replace({ name: "MainApp" }))
);

function qwe(el: any) {
  console.log(el);
}

// Define base magnifier sizes
const magnifierViewSize = ref<number>(11);

const baseCanvasRef = ref<HTMLCanvasElement>(null as any);

// On base canvas mouse move (as canvas is fullscreen -> global mouse listener)
const CANVAS_SIZE = 170;
const onMouseMove = ref<(e: MouseEvent) => void>();

const magnifierPanelRef = ref<HTMLDivElement>(null as any);

const magnifierLensCanvasRef = ref<HTMLCanvasElement>(null as any);
const magnifierGridSize = ref<number>(0);
const magnifierPosition = ref<Coord2D>({ x: 0, y: 0 });

const magnifierPanelPosition = ref<Coord2D>({ x: 20, y: 20 });

const magnifierHighlighPosition = ref<Coord2D>({ x: 0, y: 0 });
const magnifierHighlighColor = ref<RGB>([255, 255, 255]);

const activeColor = ref<RGB>([0, 0, 0]);

const preciseSensivity = ref<number>(0.3);
const isPreciesActive = ref<boolean>(false);
const isColorFinish = ref<boolean>(false);

// Scoped CSS binds
const styles = {
  canvasSize: `${CANVAS_SIZE}px`,
};

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
    (currentPosition.x + 1 + currentPosition.y * baseCanvasSize.width - 1) * 4; // Each pixel is 4 bytes (rgba)
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

function resetPrecisePosition() {
  magnifierHighlighPosition.value = {
    x: magnifierViewSize.value / 2,
    y: magnifierViewSize.value / 2,
  };
}

var currentSnapPoint: number;
const currentPosition: { start: number; end: number } = { start: 0, end: 0 };
var previousTween: gsap.core.Tween;
const arcRadius = 8;
const sideLength = 60;
const sideHeight = 50;

const halfArc = (2 * Math.PI * arcRadius) / 8;
const widthSize = sideLength - arcRadius * 2 + halfArc * 2;
const heightSize = sideHeight - arcRadius * 2 + halfArc * 2;
const pathLength = widthSize * 2 + heightSize * 2;
const SNAP_POINTS = [
  widthSize,
  widthSize + heightSize,
  2 * widthSize + heightSize,
  2 * widthSize + 2 * heightSize,
].map((el) => (el - halfArc) / pathLength);

const tl = gsap.timeline();
const reverseContainers: any[] = [];
const reverseBoxes: any[] = [];
var currentFlexReversed: boolean;

function animateFlexItem(box: any) {
  var lastX = box.x;
  var lastY = box.y;

  box.x = box.node.offsetLeft;
  box.y = box.node.offsetTop;

  // Continue if box hasn't moved
  if (lastX === box.x && lastY === box.y) return;

  // Reversed delta values taking into account current transforms
  var x = gsap.getProperty(box.node, "x") + lastX - box.x;
  var y = gsap.getProperty(box.node, "y") + lastY - box.y;

  // Tween to 0 to remove the transforms
  gsap.fromTo(
    box.node,
    { x, y },
    { duration: 0.25, x: 0, y: 0, ease: "power2.out" }
  );
}

function updateMagnifierPanelPosition(newPosition: Coord2D) {
  // Determine new snap point, skip if no need to change position
  var newSnapPoint = 0;
  if (
    newPosition.x <
      baseCanvasSize.width -
        (magnifierPanelRef.value.clientWidth + widthSize / 2) &&
    newPosition.y <
      baseCanvasSize.height -
        (magnifierPanelRef.value.clientHeight + heightSize / 2)
  )
    newSnapPoint = 2;
  else {
    if (
      newPosition.x >
      baseCanvasSize.width -
        (magnifierPanelRef.value.clientWidth + widthSize / 2)
    )
      newSnapPoint =
        newPosition.y >
        baseCanvasSize.height -
          (magnifierPanelRef.value.clientHeight + heightSize / 2)
          ? 0
          : 1;
    else {
      newSnapPoint =
        newPosition.x >
        baseCanvasSize.width -
          (magnifierPanelRef.value.clientWidth + widthSize / 2)
          ? 0
          : 3;
    }
  }

  if (newSnapPoint === currentSnapPoint) return;

  var destinationValue = SNAP_POINTS[newSnapPoint];
  var startValue = destinationValue;
  const panelTween = tl.getTweensOf(".magnifier-panel");
  const newFlexReversed = newSnapPoint == 0 || newSnapPoint == 3;

  if (panelTween.length > 0) {
    startValue =
      currentPosition.start +
      (currentPosition.end - currentPosition.start) * panelTween[0].ratio;

    if (
      Math.abs(destinationValue - startValue) >
      Math.abs(destinationValue - 1 - startValue)
    )
      destinationValue -= 1;

    // previousTween.kill();
    tl.clear();

    tl.to(".magnifier-panel", {
      duration: 0.25,
      ease: "power1.inOut",
      xPercent: newSnapPoint < 2 ? -100 : 0,
      yPercent: newSnapPoint == 3 || newSnapPoint == 0 ? -100 : 0,
      motionPath: {
        path: "#panel-position-path",
        align: "#panel-position-path",
        start: startValue,
        end: destinationValue,
      },
    });
  } else {
    currentFlexReversed = false;
    tl.to(".magnifier-panel", {
      duration: 0,
      motionPath: {
        path: "#panel-position-path",
        align: "#panel-position-path",
        end: destinationValue,
      },
    });
  }

  if (newFlexReversed != currentFlexReversed) {
    if (currentFlexReversed)
      reverseContainers.forEach((el) => el.classList.remove("reverse"));
    else reverseContainers.forEach((el) => el.classList.add("reverse"));

    reverseBoxes.forEach((el) => animateFlexItem(el));
  }

  currentFlexReversed = newFlexReversed;
  currentPosition.start = startValue;
  currentPosition.end = destinationValue;
  currentSnapPoint = newSnapPoint;
}

onMounted(() => {
  const nodes = document.querySelectorAll(".reversable-column");
  for (let i = 0; i < nodes.length; i++) {
    const node = nodes[i] as any;

    gsap.set(node, { x: 0, y: 0 });
    reverseBoxes.push({
      x: node.offsetLeft,
      y: node.offsetTop,
      node,
    });
  }

  document
    .querySelectorAll(".reverse-container")
    .forEach((el) => reverseContainers.push(el));

  console.log(reverseBoxes, reverseContainers);

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

  resetPrecisePosition();

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
      if (isPreciesActive.value || isColorFinish.value) return;

      // Update magnifier position to move html elements
      const newPosition = { x: e.x, y: e.y };
      magnifierPosition.value = newPosition;
      const magnifierRounded = Math.round(magnifierViewSize.value / 2);

      updateMagnifierPanelPosition(newPosition);

      magnifierCtx.rect(
        0,
        0,
        magnifierEl.offsetWidth,
        magnifierEl.offsetHeight
      );
      magnifierCtx.fill();
      // Copy interested base rect to magnifier canvas
      magnifierCtx.drawImage(
        baseCtx.canvas,
        e.x - magnifierRounded + 1,
        e.y - magnifierRounded + 1,
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

  if (newValue < 3) newValue = 3;
  else if (newValue > 21) newValue = 21;

  magnifierViewSize.value = newValue;
  magnifierGridSize.value = magnifierLensCanvasRef.value.offsetWidth / newValue;
  onMouseMove.value!({ x: e.clientX, y: e.clientY } as any);

  resetPrecisePosition();
}

var preciseMouseOffset = { x: 0, y: 0 };
var basePosition: Coord2D = { x: 0, y: 0 };
function onContextClick(e: MouseEvent) {
  e.preventDefault();
  if (isPreciesActive.value) {
    invoke("unfreeze_mouse");
    isPreciesActive.value = false;
  } else {
    basePosition = { ...magnifierPosition.value };

    invoke("freeze_mouse", { currentPos: basePosition });
    preciseMouseOffset = { x: 0, y: 0 };
    isColorFinish.value = false;
    isPreciesActive.value = true;
  }

  resetPrecisePosition();
}

function onPreciseMouseMove(e: MouseEvent) {
  invoke("freeze_mouse_update");
  const currentMovement = { x: e.x - 300, y: e.y - 300 };
  const prevPosition = { ...preciseMouseOffset };
  preciseMouseOffset.x += currentMovement.x * preciseSensivity.value;
  preciseMouseOffset.y += currentMovement.y * preciseSensivity.value;

  const pixelsOffset = {
    x: Math.floor(preciseMouseOffset.x / magnifierGridSize.value),
    y: Math.floor(preciseMouseOffset.y / magnifierGridSize.value),
  };

  if (
    basePosition.x + pixelsOffset.x < leftUpper.x ||
    basePosition.x + pixelsOffset.x > rightBottom.x
  )
    pixelsOffset.x = 0;

  if (
    basePosition.y + pixelsOffset.y < leftUpper.y ||
    basePosition.y + pixelsOffset.y > rightBottom.y
  )
    pixelsOffset.y = 0;

  const newHighlightPosition = {
    x: magnifierViewSize.value,
    y: magnifierViewSize.value,
  };

  // if (newHighlightPosition.x < 0.5) {
  //   preciseMouseOffset.x = prevPosition.x;
  //   newHighlightPosition.x = 0.5;
  // } else if (newHighlightPosition.x > magnifierGridSize.value / 2) {
  //   preciseMouseOffset.x = prevPosition.x;
  //   newHighlightPosition.x = magnifierGridSize.value / 2;
  // }
  // if (newHighlightPosition.y < 0.5) {
  //   preciseMouseOffset.y = prevPosition.y;
  //   newHighlightPosition.y = 0.5;
  // } else if (newHighlightPosition.y > magnifierGridSize.value / 2) {
  //   preciseMouseOffset.y = prevPosition.y;
  //   newHighlightPosition.y = magnifierGridSize.value / 2;
  // }

  magnifierHighlighPosition.value = newHighlightPosition;
  // console.log(pixelsOffset);
  // magnifierHighlighPosition.value = {
  //   x: magnifierPanelRef.value.clientWidth / 2 + offsetDistance.x * gridSize,
  //   y: magnifierPanelRef.value.clientHeight / 2 + offsetDistance.y * gridSize,
  // };

  updateActiveColor({
    x: basePosition.x + pixelsOffset.x,
    y: basePosition.y + pixelsOffset.y,
  });
}

function onPickFinish() {
  if (isPreciesActive.value) {
    invoke("unfreeze_mouse");
    isPreciesActive.value = false;
  } else {
    resetPrecisePosition();
  }

  isColorFinish.value = !isColorFinish.value;
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

.picker-magnifier {
  --crosshair-size: 30px;

  position: absolute;
  z-index: 100;
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

  width: 170px;

  background-color: rgb(130, 130, 130);

  display: flex;
  flex-direction: column;
  align-items: center;
}

.magnifier-panel-holder {
  width: v-bind("styles.canvasSize");
  height: v-bind("styles.canvasSize");
  min-width: v-bind("styles.canvasSize");
  min-height: v-bind("styles.canvasSize");

  border-radius: 3px;
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
  --magnifier-lens-grid-color: rgba(255, 255, 255, 1);

  position: absolute;

  left: 0px;
  top: 0px;

  width: 200%;
  height: 200%;

  background-image: repeating-linear-gradient(
      0deg,
      var(--magnifier-lens-grid-color) 0 0%,
      transparent 1px 100%
    ),
    repeating-linear-gradient(
      90deg,
      var(--magnifier-lens-grid-color) 0 0,
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

.movement-path {
  position: absolute;
  left: 0;
  top: 0;

  transform: translate(-50%, -50%);
}

.flex-col-reverse {
  flex-direction: column-reverse;
}
</style>

<style>
.reverse-container.reverse {
  flex-direction: column-reverse;
}
</style>
