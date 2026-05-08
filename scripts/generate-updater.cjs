#!/usr/bin/env node
const fs = require("fs");
const path = require("path");

const ref_name = process.env.REF_NAME;
const repo = process.env.REPO;
if (!ref_name || !repo) {
  console.error("Missing REF_NAME or REPO env vars");
  process.exit(1);
}

const baseUrl = `https://github.com/${repo}/releases/download/${ref_name}`;
const repoRootDir = path.join(__dirname, "..");
const artifactsDir = path.join(repoRootDir, "release-artifacts");

function readSignature(...parts) {
  const sigPath = path.join(artifactsDir, ...parts);
  return fs.readFileSync(sigPath, "utf-8").trim();
}

const updaterJson = {
  version: ref_name.replace(/^v/, ""),
  pub_date: new Date().toISOString(),
  platforms: {
    // Microsoft Installer (.msi)
    "windows-x86_64-msi": {
      signature: readSignature(`Windows-x86_64`, `snipora-${ref_name}-x86_64.msi.sig`),
      url: `${baseUrl}/snipora-${ref_name}-x86_64.msi`
    },
    // NSIS (.exe)
    "windows-x86_64-nsis": {
      signature: readSignature(`Windows-x86_64`, `snipora-${ref_name}-x86_64-setup.exe.sig`),
      url: `${baseUrl}/snipora-${ref_name}-x86_64-setup.exe`
    },

    // AppImage (.appimage)
    "linux-x86_64-appimage": {
      signature: readSignature(`Linux-x86_64`, `snipora-${ref_name}-x86_64.AppImage.sig`),
      url: `${baseUrl}/snipora-${ref_name}-x86_64.AppImage`
    },
    // Debian Package (.deb)
    "linux-x86_64-deb": {
      signature: readSignature(`Linux-x86_64`, `snipora-${ref_name}-x86_64.deb.sig`),
      url: `${baseUrl}/snipora-${ref_name}-x86_64.deb`
    },
    // Red Hat Package Manager (.rpm)
    "linux-x86_64-rpm": {
      signature: readSignature(`Linux-x86_64`, `snipora-${ref_name}-x86_64.rpm.sig`),
      url: `${baseUrl}/snipora-${ref_name}-x86_64.rpm`
    },

    // macOS application (.app)
    "macos-x86_64-app": {
      signature: readSignature(`macOS-x86_64`, `snipora-${ref_name}-x86_64.app.tar.gz.sig`),
      url: `${baseUrl}/snipora-${ref_name}-x86_64.app.tar.gz`
    },

    // macOS application (.app)
    "macos-aarch64-app": {
      signature: readSignature(`macOS-arm64`, `snipora-${ref_name}-arm64.app.tar.gz.sig`),
      url: `${baseUrl}/snipora-${ref_name}-arm64.app.tar.gz`
    },
  },
};

const outPath = path.join(artifactsDir, "updater.json");
fs.writeFileSync(outPath, JSON.stringify(updaterJson, null, 2) + "\n");

console.info(`Generated updater.json for version ${ref_name}`);
