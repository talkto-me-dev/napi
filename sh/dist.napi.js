import { readFileSync, writeFileSync } from "node:fs";

const pkg = JSON.parse(readFileSync("package.json", "utf8")),
  { version, optionalDependencies } = pkg;

// 同步版本到 optionalDependencies
// Sync version to optionalDependencies
if (optionalDependencies) {
  for (const name in optionalDependencies) {
    if (name.startsWith("@3-/") || name.startsWith("@napi-rs/")) {
      optionalDependencies[name] = version;
    }
  }
  writeFileSync("package.json", JSON.stringify(pkg, null, 2) + "\n");
}

// 同步版本到 Cargo.toml
// Sync version to Cargo.toml
const cargo_path = "Cargo.toml",
  cargo = readFileSync(cargo_path, "utf8").replace(
    /^version = ".*"/m,
    'version = "' + version + '"',
  );

writeFileSync(cargo_path, cargo);
