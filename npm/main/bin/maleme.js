#!/usr/bin/env node

import { spawn } from "node:child_process";
import { existsSync } from "node:fs";
import { createRequire } from "node:module";
import path from "node:path";
import { fileURLToPath } from "node:url";

const require = createRequire(import.meta.url);
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const TARGETS = {
  "darwin-arm64": {
    triple: "aarch64-apple-darwin",
    packageName: "maleme-darwin-arm64",
  },
  "darwin-x64": {
    triple: "x86_64-apple-darwin",
    packageName: "maleme-darwin-x64",
  },
  "linux-arm64": {
    triple: "aarch64-unknown-linux-musl",
    packageName: "maleme-linux-arm64",
  },
  "linux-x64": {
    triple: "x86_64-unknown-linux-musl",
    packageName: "maleme-linux-x64",
  },
  "win32-x64": {
    triple: "x86_64-pc-windows-msvc",
    packageName: "maleme-win32-x64",
  },
};

function detectTarget() {
  return TARGETS[`${process.platform}-${process.arch}`] ?? null;
}

function localVendorPath(target) {
  return path.resolve(
    __dirname,
    "..",
    "..",
    "platforms",
    target.packageName,
    "vendor",
  );
}

function installedVendorPath(target) {
  const packageJsonPath = require.resolve(`${target.packageName}/package.json`);
  return path.join(path.dirname(packageJsonPath), "vendor");
}

function resolveBinaryPath(target) {
  let vendorRoot;

  try {
    vendorRoot = installedVendorPath(target);
  } catch {
    const repoVendorRoot = localVendorPath(target);
    if (existsSync(repoVendorRoot)) {
      vendorRoot = repoVendorRoot;
    }
  }

  if (!vendorRoot) {
    throw new Error(
      `Missing optional dependency ${target.packageName}. Reinstall with: npm install -g maleme@latest`,
    );
  }

  const binaryName = process.platform === "win32" ? "maleme.exe" : "maleme";
  return path.join(vendorRoot, target.triple, "maleme", binaryName);
}

const target = detectTarget();
if (!target) {
  throw new Error(`Unsupported platform: ${process.platform} (${process.arch})`);
}

const binaryPath = resolveBinaryPath(target);
if (!existsSync(binaryPath)) {
  throw new Error(`Binary not found at ${binaryPath}`);
}

const child = spawn(binaryPath, process.argv.slice(2), {
  stdio: "inherit",
  env: process.env,
});

child.on("error", (error) => {
  console.error(error);
  process.exit(1);
});

const forwardSignal = (signal) => {
  if (!child.killed) {
    try {
      child.kill(signal);
    } catch {
      // Ignore signal forwarding errors from already-exiting children.
    }
  }
};

["SIGINT", "SIGTERM", "SIGHUP"].forEach((signal) => {
  process.on(signal, () => forwardSignal(signal));
});

const result = await new Promise((resolve) => {
  child.on("exit", (code, signal) => {
    if (signal) {
      resolve({ signal });
      return;
    }

    resolve({ code: code ?? 1 });
  });
});

if ("signal" in result) {
  process.kill(process.pid, result.signal);
} else {
  process.exit(result.code);
}
