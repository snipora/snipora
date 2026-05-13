#!/usr/bin/env node
const fs = require("node:fs");
const path = require("node:path");

const TAG = process.env.TAG;
const REPOSITORY = process.env.REPOSITORY;
const OUTPUT_DIR = process.env.OUTPUT_DIR;
const PUBLISHED_AT = process.env.PUBLISHED_AT;
if (!TAG || !REPOSITORY || !OUTPUT_DIR || !PUBLISHED_AT) {
  console.error("Missing TAG or REPOSITORY or OUTPUT_DIR or PUBLISHED_AT env vars");
  process.exit(1);
}

const version = TAG.replace(/^v/, "");

const baseReleaseUrl = `https://github.com/${REPOSITORY}/releases/download/${TAG}`;

function asset(file) {
  return {
    file,
    url: `${baseReleaseUrl}/${file}`,
  };
}

const manifest = {
  version: version,
  tag: TAG,
  publishedAt: PUBLISHED_AT,
  notesUrl: `https://github.com/${REPOSITORY}/releases/tag/${TAG}`,
  downloads: {
    windows: {
      x86_64: {
        nsis: asset(`snipora-${TAG}-x86_64-setup.exe`),
        msi: asset(`snipora-${TAG}-x86_64.msi`),
      },
    },
    linux: {
      x86_64: {
        appimage: asset(`snipora-${TAG}-x86_64.AppImage`),
        deb: asset(`snipora-${TAG}-x86_64.deb`),
        rpm: asset(`snipora-${TAG}-x86_64.rpm`),
      },
    },
    macos: {
      x86_64: {
        dmg: asset(`snipora-${TAG}-x86_64.dmg`),
        app: asset(`snipora-${TAG}-x86_64.app.tar.gz`),
      },
      arm64: {
        dmg: asset(`snipora-${TAG}-arm64.dmg`),
        app: asset(`snipora-${TAG}-arm64.app.tar.gz`),
      },
    },
  },
};

fs.mkdirSync(OUTPUT_DIR, { recursive: true });

const versionedManifestPath = path.join(OUTPUT_DIR, `${TAG}.json`);

const latestManifestPath = path.join(OUTPUT_DIR, `latest.json`);

const serialized = JSON.stringify(manifest, null, 2) + "\n";

fs.writeFileSync(versionedManifestPath, serialized);
fs.writeFileSync(latestManifestPath, serialized);

console.log(`Generated download manifest for ${TAG}`);
