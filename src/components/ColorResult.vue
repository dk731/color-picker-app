<template>
  <div
    class="color-holder"
    ref="colorHolderRef"
    :onkeydown="onKeyDown"
    :onkeyup="onKeyUp"
    tabindex="0"
  >
    <div v-for="converter in colorConversionMap" class="color-result">
      <div>{{ converter.displayName }}</div>
      <flex-spacer></flex-spacer>
      <div>
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
import { ref, defineProps, onMounted } from "vue";
import { register, unregisterAll } from "@tauri-apps/api/globalShortcut";

import { RGB } from "color-convert/conversions";

import colorConversionMap from "../additional/ColorCoversionMap";
import FlexSpacer from "./FlexSpacer.vue";

const isSpecialActive = ref<boolean>(false);
const colorHolderRef = ref<HTMLDivElement>(null as any);

const { activeColor } = defineProps<{ activeColor: RGB }>();

function onKeyDown(e: KeyboardEvent) {
  console.log(e);
  if (e.key == "Shift") isSpecialActive.value = true;
}
function onKeyUp(e: KeyboardEvent) {
  if (e.key == "Shift") isSpecialActive.value = false;
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

  margin-bottom: 5px;
}

.result-color-display {
  width: 25px;
  height: 25px;
  border: none;

  background-color: red;
}
</style>
