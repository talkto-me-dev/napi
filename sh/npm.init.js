import { mkdirSync, existsSync } from "node:fs";
import { join } from "node:path";
import yargs from "yargs";
import { hideBin } from "yargs/helpers";
import read from "@3-/read";
import write from "@3-/write";
import { $ } from "@3-/zx";

// 解析参数
const argv = yargs(hideBin(process.argv))
    .option("cwd", {
      alias: "c",
      type: "string",
      description: "工作目录",
      default: ".",
    })
    // .option("trust", {
    //   type: "boolean",
    //   description: "配置 Trusted Publisher",
    //   default: true,
    // })
    .parseSync(),
  cwd = argv.cwd,
  pkg_path = join(cwd, "package.json");

if (!existsSync(pkg_path)) {
  console.error("未找到 package.json: " + pkg_path);
  process.exit(1);
}

const pkg = JSON.parse(read(pkg_path)),
  { version, name: main_name, napi } = pkg;

if (!napi?.targets) {
  console.log("未发现 napi 配置或目标，跳过。");
  process.exit(0);
}

const targets = napi.targets,
  // 平台解析规则
  OS_MAP = [
    ["darwin", "darwin"],
    ["linux", "linux"],
    ["windows", "win32"],
  ],
  ENV_MAP = [
    ["musl", "musl"],
    ["gnu", "gnu"],
    ["msvc", "msvc"],
  ],
  // 动态解析平台目录名
  platformDir = (target) => {
    const os = OS_MAP.find(([k]) => target.includes(k))?.[1] || "",
      env = ENV_MAP.find(([k]) => target.includes(k))?.[1] || "",
      cpu = target.split("-")[0].replace("aarch64", "arm64").replace("x86_64", "x64");

    return os + "-" + cpu + (env ? "-" + env : "");
  },
  npm_dir = join(cwd, "npm");

if (!existsSync(npm_dir)) {
  mkdirSync(npm_dir, { recursive: true });
}

// 检查 npm 包是否存在
const pkgExists = async (name) => {
  try {
    await $`npm view ${name} version`;
    return true;
  } catch {
    return false;
  }
};

// 获取 repo 路径
const repoPath = async () => {
  try {
    const url = await $({ cwd })`git remote get-url origin`;
    return url.stdout
      .trim()
      .split(":")
      .pop()
      .replace(/\.git$/, "")
      .replace("443/", "");
  } catch (err) {
    console.warn("无法获取 git 仓库地址:", err.message);
    return "";
  }
};

const repo = await repoPath(),
  workflow = main_name.split("/").pop() + "-publish.yml";

// 处理单个目标的初始化
const initTarget = async (target) => {
  const npm_dir_name = platformDir(target),
    sub_pkg_name = main_name + "-" + npm_dir_name,
    sub_pkg_dir = join(npm_dir, npm_dir_name);

  if (!(await pkgExists(sub_pkg_name))) {
    console.log("\n[INIT] 初始化缺失的包: " + sub_pkg_name);
    if (!existsSync(sub_pkg_dir)) mkdirSync(sub_pkg_dir, { recursive: true });

    const [os, cpu] = npm_dir_name.split("-"),
      sub_pkg = {
        name: sub_pkg_name,
        version,
        os: [os === "win32" ? "win32" : os],
        cpu: [cpu === "x64" ? "x64" : cpu === "arm64" ? "arm64" : cpu],
        files: ["*.node"],
        description: "Native binary package for " + sub_pkg_name,
        repository: pkg.repository,
        license: pkg.license,
        publishConfig: { access: "public" },
      },
      binary_name = napi.binaryName || "index";

    sub_pkg.main = binary_name + "." + npm_dir_name + ".node";
    sub_pkg.files = [sub_pkg.main];

    write(join(sub_pkg_dir, "package.json"), JSON.stringify(sub_pkg, null, 2) + "\n");

    const node_file = join(sub_pkg_dir, sub_pkg.main);
    if (!existsSync(node_file)) write(node_file, "");

    try {
      console.log("正在发布 " + sub_pkg_name + "...");
      await $({ cwd: sub_pkg_dir, stdio: "inherit" })`npm publish --access public`;
      await new Promise((r) => setTimeout(r, 2000));
    } catch (err) {
      console.error("发布失败: " + err.message);
    }
  }

  // // 始终尝试配置信任
  // if (argv.trust && repo) {
  //   try {
  //     console.log('正在为 ' + sub_pkg_name + ' 配置 Trusted Publisher...');
  //     // 在子包目录下运行，不传递包名参数，避免参数解析歧义
  //     await $({ cwd: sub_pkg_dir, stdio: 'inherit' })`npm trust github --repository ${repo} --file ${workflow} --yes`;
  //   } catch (err) {
  //     console.warn('配置信任失败: ' + err.message);
  //   }
  // }
};

// 遍历处理所有目标
for (const target of targets) {
  await initTarget(target);
}

console.log("\n初始化检查完成。");
