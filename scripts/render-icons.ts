import fs from "node:fs";
import path from "node:path";
import sharp from "sharp";

const SOURCE_ROOT = "assets/";
const OUTPUT_ROOT = "src-tauri/icons/";
const SIZE = 128;

type RenderItem = { input: string, output: string };

const renderItems: RenderItem[] = [
  { input: "tray/logo/app.svg", output: "tray/logo/app.png" },
  { input: "tray/logo/dark.svg", output: "tray/logo/dark.png" },
  { input: "tray/logo/light.svg", output: "tray/logo/light.png" },
];

for (const item of renderItems) {
  const input = path.join(SOURCE_ROOT, item.input);
  const output = path.join(OUTPUT_ROOT, item.output);

  fs.mkdirSync(path.dirname(output), { recursive: true })

  const png_buffer = await sharp(input)
      .resize(SIZE, SIZE)
      .ensureAlpha()
      .png({ effort: 10, compressionLevel: 9, palette: false })
      .toBuffer();
  fs.writeFileSync(output, png_buffer);
  console.info(`Rendered ${input} -> ${output}`);
}
