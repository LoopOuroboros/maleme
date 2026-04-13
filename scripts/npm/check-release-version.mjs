import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(scriptDir, "..", "..");
const cargoToml = fs.readFileSync(path.join(repoRoot, "Cargo.toml"), "utf8");
const versionMatch = cargoToml.match(/^version\s*=\s*"([^"]+)"/m);

if (!versionMatch) {
  throw new Error("Unable to read version from Cargo.toml");
}

const cargoVersion = versionMatch[1];
const gitRef = process.argv[2] ?? process.env.GITHUB_REF_NAME;

if (!gitRef) {
  console.log(`Cargo.toml version: ${cargoVersion}`);
  process.exit(0);
}

const normalizedTag = gitRef.startsWith("v") ? gitRef.slice(1) : gitRef;
if (normalizedTag !== cargoVersion) {
  throw new Error(`Tag ${gitRef} does not match Cargo.toml version ${cargoVersion}`);
}

console.log(`Release version OK: ${cargoVersion}`);
