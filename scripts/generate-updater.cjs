#!/usr/bin/env node
const fs = require("node:fs");
const path = require("node:path");

const TAG = process.env.TAG;
const REPOSITORY = process.env.REPOSITORY;
const PUBLISHED_AT = process.env.PUBLISHED_AT;
if (!TAG || !REPOSITORY || !PUBLISHED_AT) {
  console.error("Missing TAG or REPOSITORY or PUBLISHED_AT env vars");
  process.exit(1);
}

const baseUrl = `https://github.com/${REPOSITORY}/releases/download/${TAG}`;
const repoRootDir = path.join(__dirname, "..");
const artifactsDir = path.join(repoRootDir, "release-artifacts");

function readSignature(...parts) {
  const sigPath = path.join(artifactsDir, ...parts);
  return fs.readFileSync(sigPath, "utf-8").trim();
}

const updaterJson = {
  version: TAG.replace(/^v/, ""),
  pub_date: PUBLISHED_AT,
  platforms: {
    // Microsoft Installer (.msi)
    "windows-x86_64-msi": {
      signature: readSignature(`Windows-x86_64`, `snipora-${TAG}-x86_64.msi.sig`),
      url: `${baseUrl}/snipora-${TAG}-x86_64.msi`
    },
    // NSIS (.exe)
    "windows-x86_64-nsis": {
      signature: readSignature(`Windows-x86_64`, `snipora-${TAG}-x86_64-setup.exe.sig`),
      url: `${baseUrl}/snipora-${TAG}-x86_64-setup.exe`
    },

    // AppImage (.appimage)
    "linux-x86_64-appimage": {
      signature: readSignature(`Linux-x86_64`, `snipora-${TAG}-x86_64.AppImage.sig`),
      url: `${baseUrl}/snipora-${TAG}-x86_64.AppImage`
    },
    // Debian Package (.deb)
    "linux-x86_64-deb": {
      signature: readSignature(`Linux-x86_64`, `snipora-${TAG}-x86_64.deb.sig`),
      url: `${baseUrl}/snipora-${TAG}-x86_64.deb`
    },
    // Red Hat Package Manager (.rpm)
    "linux-x86_64-rpm": {
      signature: readSignature(`Linux-x86_64`, `snipora-${TAG}-x86_64.rpm.sig`),
      url: `${baseUrl}/snipora-${TAG}-x86_64.rpm`
    },

    // macOS application (.app)
    "macos-x86_64-app": {
      signature: readSignature(`macOS-x86_64`, `snipora-${TAG}-x86_64.app.tar.gz.sig`),
      url: `${baseUrl}/snipora-${TAG}-x86_64.app.tar.gz`
    },

    // macOS application (.app)
    "macos-aarch64-app": {
      signature: readSignature(`macOS-arm64`, `snipora-${TAG}-arm64.app.tar.gz.sig`),
      url: `${baseUrl}/snipora-${TAG}-arm64.app.tar.gz`
    },
  },
};

const outPath = path.join(artifactsDir, "updater.json");
fs.writeFileSync(outPath, JSON.stringify(updaterJson, null, 2) + "\n");

console.info(`Generated updater.json for tag ${TAG}`);
