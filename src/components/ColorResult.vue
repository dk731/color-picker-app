<template>
  <div
    class="color-holder"
    ref="colorHolderRef"
    :onkeydown="onKeyDown"
    :onkeyup="onKeyUp"
    tabindex="0"
  >
    <div
      v-for="converter in colorConversionMap"
      class="color-result"
      v-on:click="(e) => onResultClick(e, converter)"
    >
      <div>{{ converter.displayName }}</div>
      <flex-spacer></flex-spacer>
      <div class="color-result-value">
        {{
          isSpecialActive
            ? converter.altConverter(activeColor)
            : converter.converter(activeColor)
        }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, defineProps } from "vue";

import { RGB } from "color-convert/conversions";

import colorConversionMap, {
  ConversionMeta,
} from "../additional/ColorCoversionMap";
import FlexSpacer from "./FlexSpacer.vue";

import {
  getCurrent,
  PhysicalSize,
  PhysicalPosition,
} from "@tauri-apps/api/window";

getCurrent().setSize(new PhysicalSize(220, 400));

const isSpecialActive = ref<boolean>(false);
const colorHolderRef = ref<HTMLDivElement>(null as any);

const { activeColor, onColorCopyCallback } = defineProps<{
  activeColor: RGB;
  onColorCopyCallback?: (copyText: string) => void;
}>();

function onKeyDown(e: KeyboardEvent) {
  if (e.key == "Shift") isSpecialActive.value = true;
}
function onKeyUp(e: KeyboardEvent) {
  if (e.key == "Shift") isSpecialActive.value = false;
}

function onResultClick(e: MouseEvent, conv: ConversionMeta) {
  const copyText = "Test";
  const clickDiv = e.target as HTMLDivElement;

  // Ignore click if div is in animation
  if (clickDiv.classList.contains("dont-hover")) return;

  clickDiv.classList.add("result-clicked", "slow-transition", "dont-hover");
  setTimeout(() => {
    clickDiv.classList.remove("result-clicked");
    setTimeout(
      () => clickDiv.classList.remove("slow-transition", "dont-hover"),
      350
    );
  }, 350);

  navigator.clipboard.writeText(copyText); // Copy selected color to clipboard
  if (onColorCopyCallback) onColorCopyCallback(copyText);
}
</script>

<style scoped>
.color-holder {
  display: flex;
  flex-direction: column;

  width: 100%;

  outline: none;
}

.color-result {
  display: flex;
  flex-direction: row;

  align-items: center;
  justify-content: center;

  background-color: rgb(13, 129, 231);
  padding-left: 3px;
  padding-right: 3px;

  border: none;
  border-radius: 3px;

  margin-bottom: 5px;

  transition: all 0.1s;
}

.color-result * {
  pointer-events: none;
}

.color-result.slow-transition {
  transition: all 0.35s;
}

.result-clicked {
  transform: scale(0.9);
  background-color: red;
}

.color-result:not(.dont-hover):hover {
  transform: scale(1.1);
  background-color: aqua;
}

.color-result:not(.dont-hover):hover .color-result-value {
  font-weight: bold;
}

.result-color-display {
  width: 25px;
  height: 25px;
  border: none;

  background-color: red;
}
</style>
