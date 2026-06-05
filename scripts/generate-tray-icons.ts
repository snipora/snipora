import fs from "node:fs";
import path from "node:path";
import sharp from "sharp";

const SOURCE_ROOT = "assets/";
const OUTPUT_ROOT = "src-tauri/icons/tray/";
const SIZE = 128;

type Variant = { input: string, name: string };

const variants: Variant[] = [
  { input: "snipora.svg", name: "app" },
  { input: "snipora-wireframe-dark.svg", name: "dark" },
  { input: "snipora-wireframe-light.svg", name: "light" },
];

fs.mkdirSync(OUTPUT_ROOT, { recursive: true })

for (const variant of variants) {
  const input = path.join(SOURCE_ROOT, `${variant.input}`);
  const output = path.join(OUTPUT_ROOT, `logo-${variant.name}.png`);

  const png_buffer = await sharp(input)
      .resize(SIZE, SIZE)
      .ensureAlpha()
      .png({ effort: 10, compressionLevel: 9, palette: false })
      .toBuffer();
  fs.writeFileSync(output, png_buffer);
  console.info(`Created ${output}`);
}
