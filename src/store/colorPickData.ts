import { defineStore } from "pinia";

type ColorPickDisplayData = {
  id: string;
  buffer: string;
  position: { x: number; y: number };
  size: { width: number; height: number };
};

type ColorPickState = {
  displays: ColorPickDisplayData[] | null;
};

const useColorPickData = defineStore("ColorPickData", {
  state: (): ColorPickState => ({
    displays: null,
  }),
});

export default useColorPickData;
