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

const version = versionMatch[1];

const packageFiles = [
  path.join(repoRoot, "npm", "main", "package.json"),
  path.join(repoRoot, "npm", "platforms", "darwin-arm64", "package.json"),
  path.join(repoRoot, "npm", "platforms", "darwin-x64", "package.json"),
  path.join(repoRoot, "npm", "platforms", "linux-arm64", "package.json"),
  path.join(repoRoot, "npm", "platforms", "linux-x64", "package.json"),
  path.join(repoRoot, "npm", "platforms", "win32-x64", "package.json"),
];

for (const packageFile of packageFiles) {
  const packageJson = JSON.parse(fs.readFileSync(packageFile, "utf8"));
  packageJson.version = version;

  if (packageJson.optionalDependencies) {
    for (const dependency of Object.keys(packageJson.optionalDependencies)) {
      packageJson.optionalDependencies[dependency] = version;
    }
  }

  fs.writeFileSync(packageFile, `${JSON.stringify(packageJson, null, 2)}\n`);
  console.log(`Synced ${path.relative(repoRoot, packageFile)} => ${version}`);
}
