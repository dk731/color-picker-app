import convert from "color-convert";
import { RGB } from "color-convert/conversions";

export type ConversionMeta = {
  displayName: string;
  converter: (color: RGB) => any;
  altConverter: (color: RGB) => any;
};

const colorConversionMap: ConversionMeta[] = [
  {
    displayName: "RGB",
    converter: (c: RGB) => c,
    altConverter: (c: RGB) => c.map((l) => parseFloat((l / 255).toFixed(2))),
  },
  {
    displayName: "HEX",
    converter: (c: RGB) => "#" + convert.rgb.hex(c),
    altConverter: (c: RGB) => "#" + convert.rgb.hex(c).toLocaleLowerCase(),
  },
  {
    displayName: "HSL",
    converter: convert.rgb.hsl,
    altConverter: (c: RGB) =>
      convert.rgb.hsl(c).map((l) => parseFloat((l / 255).toFixed(2))),
  },
  {
    displayName: "HSV",
    converter: convert.rgb.hsv,
    altConverter: (c: RGB) =>
      convert.rgb.hsv(c).map((l) => parseFloat((l / 255).toFixed(2))),
  },
  {
    displayName: "HWB",
    converter: convert.rgb.hwb,
    altConverter: (c: RGB) =>
      convert.rgb.hwb(c).map((l) => parseFloat((l / 255).toFixed(2))),
  },
  {
    displayName: "CMYK",
    converter: convert.rgb.cmyk,
    altConverter: (c: RGB) =>
      convert.rgb.cmyk(c).map((l) => parseFloat((l / 255).toFixed(2))),
  },
  {
    displayName: "ANSI16",
    converter: convert.rgb.ansi16,
    altConverter: (c: RGB) =>
      "#" + ("0000" + convert.rgb.ansi16(c).toString(16)).slice(-4),
  },
  {
    displayName: "ANSI256",
    converter: convert.rgb.ansi256,
    altConverter: (c: RGB) =>
      "#" + ("00000000" + convert.rgb.ansi16(c).toString(16)).slice(-8),
  },
  {
    displayName: "APPLE",
    converter: convert.rgb.apple,
    altConverter: convert.rgb.apple,
  },
  {
    displayName: "GRAY",
    converter: convert.rgb.gray,
    altConverter: (c: RGB) =>
      convert.rgb.gray(c).map((l) => parseFloat((l / 255).toFixed(2))),
  },
  {
    displayName: "HCG",
    converter: convert.rgb.hcg,
    altConverter: (c: RGB) =>
      convert.rgb.gray(c).map((l) => parseFloat((l / 255).toFixed(2))),
  },
  {
    displayName: "LAB",
    converter: convert.rgb.lab,
    altConverter: (c: RGB) =>
      convert.rgb.lab(c).map((l) => parseFloat((l / 255).toFixed(2))),
  },
  {
    displayName: "LCH",
    converter: convert.rgb.lch,
    altConverter: (c: RGB) =>
      convert.rgb.lch(c).map((l) => parseFloat((l / 255).toFixed(2))),
  },
  {
    displayName: "XYZ",
    converter: convert.rgb.xyz,
    altConverter: (c: RGB) =>
      convert.rgb.lch(c).map((l) => parseFloat((l / 255).toFixed(2))),
  },
  {
    displayName: "HEX(Short)",
    converter: (c: RGB) =>
      "#" +
      [...convert.rgb.hex(c.map((l) => Math.round((l / 255) * 5) * 51) as any)]
        .filter((_, i) => i % 2 == 0)
        .join(""),
    altConverter: (c: RGB) =>
      "#" +
      [...convert.rgb.hex(c.map((l) => Math.round((l / 255) * 5) * 51) as any)]
        .filter((_, i) => i % 2 == 0)
        .join("")
        .toLocaleLowerCase(),
  },
];

export default colorConversionMap;
