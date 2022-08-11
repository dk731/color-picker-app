<template>
  <canvas
    v-on:mousemove="onMouseMove"
    v-on:mouseenter="onMouseMove"
    class="color-pick-canvas"
    id="pick-canvas"
    ref="canvasRef"
  ></canvas>
  <div
    class="picker-lens"
    :style="{ left: `${cursorPos.x}px`, top: `${cursorPos.y}px` }"
  >
    <icon class="picker-lens-cursor" icon="radix-icons:crosshair-2"></icon>
    <div class="picker-panel">
      <flex-spacer />
      <div class="picker-mgnifier-lens" ref="magnifierRef"></div>
      <flex-spacer />
    </div>
  </div>
  <!-- <picker-lens
    :ctx="canvasRef?.getContext('2d')"
    :position="cursorPos"
  ></picker-lens> -->
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

unregisterAll().then(() =>
  register("Esc", (short) => router.replace({ name: "MainApp" }))
);

const canvasRef = ref<HTMLCanvasElement>(null as any);
const magnifierRef = ref<HTMLDivElement>(null as any);
const cursorPos = ref<{ x: number; y: number }>({ x: 0, y: 0 });
const magnifierKoef = ref<number>(1);

// const colorPickData = useColorPickData();
const colorPickData = {
  displays: [
    {
      id: "test1",
      position: { x: 0, y: 0 },
      size: { width: 2560, height: 1440 },
      buffer: "src/assets/disp_1.png",
    },
    {
      id: "test2",
      position: { x: 2560, y: 0 },
      size: { width: 2560, height: 1440 },
      buffer: "src/assets/disp_2.png",
    },
  ],
};

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

const onMouseMove = ref<(e: MouseEvent) => void>();

onMounted(() => {
  const ctx = canvasRef.value.getContext("2d")!;
  const magEl = magnifierRef.value;

  const canvasWidth = rightBottom.x - leftUpper.x;
  const canvasHeight = rightBottom.y - leftUpper.y;

  ctx.canvas.width = canvasWidth;
  ctx.canvas.height = canvasHeight;

  onMouseMove.value = (e: MouseEvent) => {
    cursorPos.value = { x: e.x, y: e.y };

    const pos = `${-e.x * magnifierKoef.value + magEl.offsetWidth / 2}px ${
      -e.y * magnifierKoef.value + magEl.offsetHeight / 2
    }px`;

    magEl.style.backgroundPosition = pos;
    console.log(pos);
  };

  Promise.all(
    colorPickData.displays.map(async (item, i) => {
      const tmpImage = new Image();
      const syncPromise = new Promise<void>((resolve) => {
        tmpImage.onload = () => {
          ctx.drawImage(tmpImage, item.position.x, item.position.y);
          resolve();
        };
        tmpImage.src = item.buffer;
      });

      await syncPromise;
    })
  ).then(() => {
    magEl.style.backgroundImage = `url(${ctx.canvas.toDataURL()})`;
    magEl.style.backgroundRepeat = "no-repeat";
    magEl.style.backgroundSize = `${ctx.canvas.width * magnifierKoef.value}px ${
      ctx.canvas.height * magnifierKoef.value
    }px;`;
  });
});
</script>

<style>
.color-pick-canvas {
  width: 100%;
  height: 100%;

  padding: 0;
  margin: 0;
  cursor: none;
}

.picker-lens,
.picker-lens * {
  cursor: none;
  pointer-events: none;
}

.picker-lens {
  --crosshair-size: 30px;

  position: absolute;
  z-index: 999;
  cursor: none;
}

.picker-lens-cursor {
  position: relative;
  top: 0;
  left: 0;
  transform: translate(-50%, -50%);
  width: var(--crosshair-size);
  height: var(--crosshair-size);
}

.picker-panel {
  position: absolute;
  padding: 10px;

  top: 0;
  left: calc(var(--crosshair-size) / 2 + 5px);

  width: 150px;
  height: 200px;

  background-color: red;

  display: flex;
  flex-direction: column;
  align-items: center;
}

.picker-mgnifier-lens {
  width: 80%;
  aspect-ratio: 1;
  /* border-radius: 50%; */
}
</style>
