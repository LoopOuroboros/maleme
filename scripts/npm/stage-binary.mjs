import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(scriptDir, "..", "..");

const PLATFORM_BY_TARGET = {
  "aarch64-apple-darwin": {
    packageName: "maleme-darwin-arm64",
    binaryName: "maleme",
    directory: "darwin-arm64",
  },
  "x86_64-apple-darwin": {
    packageName: "maleme-darwin-x64",
    binaryName: "maleme",
    directory: "darwin-x64",
  },
  "aarch64-unknown-linux-musl": {
    packageName: "maleme-linux-arm64",
    binaryName: "maleme",
    directory: "linux-arm64",
  },
  "x86_64-unknown-linux-musl": {
    packageName: "maleme-linux-x64",
    binaryName: "maleme",
    directory: "linux-x64",
  },
  "x86_64-pc-windows-msvc": {
    packageName: "maleme-win32-x64",
    binaryName: "maleme.exe",
    directory: "win32-x64",
  },
};

const targetTriple = process.argv[2];
const binaryPathArg = process.argv[3];

if (!targetTriple || !binaryPathArg) {
  throw new Error(
    "Usage: node scripts/npm/stage-binary.mjs <target-triple> <binary-path>",
  );
}

const config = PLATFORM_BY_TARGET[targetTriple];
if (!config) {
  throw new Error(`Unsupported target triple: ${targetTriple}`);
}

const sourcePath = path.resolve(repoRoot, binaryPathArg);
if (!fs.existsSync(sourcePath)) {
  throw new Error(`Binary does not exist: ${sourcePath}`);
}

const vendorRoot = path.join(
  repoRoot,
  "npm",
  "platforms",
  config.directory,
  "vendor",
  targetTriple,
  "maleme",
);

fs.rmSync(path.join(repoRoot, "npm", "platforms", config.directory, "vendor"), {
  recursive: true,
  force: true,
});
fs.mkdirSync(vendorRoot, { recursive: true });

const destinationPath = path.join(vendorRoot, config.binaryName);
fs.copyFileSync(sourcePath, destinationPath);

if (config.binaryName === "maleme") {
  fs.chmodSync(destinationPath, 0o755);
}

console.log(`Staged ${config.packageName} from ${sourcePath}`);
