<template>
  <div class="pick-lens-cursor">1</div>
</template>

<script setup lang="ts">
import { watch } from "vue";

const { ctx, position } = defineProps<{
  ctx: CanvasRenderingContext2D | null;
  position: { x: number; y: number };
}>();

watch(position, (oldPos, newPos) => {
  console.log(newPos);
});
</script>

<style scope>
.pick-lens-cursor {
  position: absolute;
}
</style>
