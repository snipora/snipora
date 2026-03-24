import fs from "node:fs";
import path from "node:path";
import sharp from "sharp";
import pngToIco from "png-to-ico";

const SOURCE_ROOT = "assets/";
const OUTPUT_ROOT = "src-tauri/resources/tray-icons/";
const SIZE = 128;

type Variant = { input: string, name: string };

const variants: Variant[] = [
  { input: "snipora.svg", name: "app" },
  { input: "snipora-icon--dark.svg", name: "dark" },
  { input: "snipora-icon--light.svg", name: "light" },
];

fs.mkdirSync(OUTPUT_ROOT, { recursive: true })

for (const variant of variants) {
  const input = path.join(SOURCE_ROOT, `${variant.input}`);
  const output = path.join(OUTPUT_ROOT, `logo-${variant.name}`);

  const png_buffer = await sharp(input)
      .resize(SIZE, SIZE)
      .png({ effort: 10, compressionLevel: 9 })
      .toBuffer();
  const ico_buffer = await pngToIco(png_buffer);
  fs.writeFileSync(`${output}.png`, png_buffer);
  fs.writeFileSync(`${output}.ico`, ico_buffer);
}
