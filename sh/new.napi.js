import { readdir, stat } from "node:fs/promises";
import { join } from "node:path";
import yargs from "yargs/yargs";
import { hideBin } from "yargs/helpers";
import { parse, stringify } from "smol-toml";
import read from "@3-/read";
import write from "@3-/write";

const argv = yargs(hideBin(process.argv)).parse(),
  project = argv._[0];

if (!project) {
  console.error("用法: node replace.js <项目名称>");
  process.exit(1);
}

import { execSync } from "node:child_process";
const github_repo = execSync("git remote get-url origin")
  .toString()
  .trim()
  .split(":")
  .pop()
  .replace(/\.git$/, "")
  .replace("443/", "");

const snake = project.replaceAll("-", "_"),
  root = join(import.meta.dirname, project),
  subP = (c) => c.replaceAll("_tmpl", project).replaceAll("talkto-me-dev/napi", github_repo),
  subS = (c) => c.replaceAll("_tmpl", snake).replaceAll("talkto-me-dev/napi", github_repo),
  MAP = {
    "package.json": subP,
    "test.sh": subP,
    "Cargo.toml": subS,
    "lib.rs": subS,
    "index.test.js": subS,
    "index.js": (c) =>
      subP(c).replaceAll("export default binding." + project, "export default binding." + snake),
  },
  proc = (path) => {
    const content = read(path),
      name = path.split("/").pop(),
      fn = MAP[name],
      new_content = fn ? fn(content) : content;

    if (new_content !== content) {
      write(path, new_content);
      console.log("已更新: " + path);
    }
  },
  walk = async (dir) => {
    const files = await readdir(dir);
    await Promise.all(
      files.map(async (f) => {
        const path = join(dir, f),
          s = await stat(path);
        if (s.isDirectory()) {
          if (f !== "node_modules" && f !== ".git" && f !== "target") {
            await walk(path);
          }
        } else {
          proc(path);
        }
      }),
    );
  },
  workspaceUp = () => {
    const path = join(import.meta.dirname, "Cargo.toml"),
      content = read(path),
      toml = parse(content),
      { workspace } = toml;

    if (!workspace.members.includes(project)) {
      workspace.members.push(project);
      workspace.members.sort();
      write(path, stringify(toml));
      console.log("已更新 workspace: " + project);
    }
  };

await walk(root);
workspaceUp();
