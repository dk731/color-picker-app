<template>
    <div class="main-view-holder">
        <div class="app-toolbar" data-tauri-drag-region>
            <div class="app-title">Color Picker</div>
            <flex-spacer></flex-spacer>
            <icon class="close-btn" icon="ep:close-bold"></icon>
        </div>
        <flex-spacer></flex-spacer>
        <icon class="color-picker-btn" icon="pepicons:color-picker"></icon>
        <flex-spacer></flex-spacer>
    </div>
</template>

<script setup lang="ts">
import FlexSpacer from '../components/FlexSpacer.vue';
import { Icon } from '@iconify/vue';

function startColorPicker() {

}
</script>
<style scoped>
.main-view-holder {
    display: flex;
    flex-direction: column;
    align-items: center;

    width: 100%;
    height: 100%;

    border-radius: 10px;
    background: var(--main-color);
}

/* ToolBar //////////////////////////////////////////////////////////////////////////*/

.app-toolbar {
    --toolbar-height: 26px;
    --side-padding: 10px;

    display: flex;
    flex-direction: row;
    align-items: center;

    box-sizing: border-box;
    padding-left: var(--side-padding);
    padding-right: var(--side-padding);

    height: var(--toolbar-height);
    min-height: var(--toolbar-height);
    max-height: var(--toolbar-height);

    width: 100%;
}

.app-title {
    pointer-events: none;
}

.close-btn {
    font-size: 18px;
    color: var(--main-color-strong);
    transition: all 0.1s;
}

.close-btn:hover {
    transform: scale(1.3);
    color: var(--main-color-strong-strong);
}

/* MainBody /////////////////////////////////////////////////////////////////////////*/
.color-picker-btn {
    padding: 10px;
    background-color: var(--main-color-light);
    border-radius: 50%;
    font-size: 80px;
    transition: all 0.1s;
}

.color-picker-btn:hover {
    transform: scale(1.05);
    background-color: var(--main-color-strong);
}
</style>