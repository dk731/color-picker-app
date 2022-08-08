import {
  PhysicalPosition,
  PhysicalSize,
  getCurrent,
} from "@tauri-apps/api/window";
import { defineStore } from "pinia";

type AppWindowMeta = {
  position: PhysicalPosition;
  size: PhysicalSize;
  name?: string;
};

type AppWindowMetaState = {
  [windowPath: string]: AppWindowMeta;
};

const useAppWindowMeta = defineStore("AppWindowMeta", {
  state: (): AppWindowMetaState => {
    return {};
  },
});

function updateWindow(newWindow: AppWindowMeta) {
  getCurrent().setSize(newWindow.size);
  getCurrent().setPosition(newWindow.position);
}

export { updateWindow };
export default useAppWindowMeta;
