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
    .parseSync(),
  cwd = argv.cwd,
  pkg_path = join(cwd, "package.json");

if (!existsSync(pkg_path)) {
  console.error(`未找到 package.json: ${pkg_path}`);
  process.exit(1);
}

const pkg = JSON.parse(read(pkg_path)),
  { version, name: main_name, napi } = pkg;

if (!napi?.targets) {
  console.log("未发现 napi 配置或目标，跳过。");
  process.exit(0);
}

const targets = napi.targets,
  // 动态解析平台目录名
  platformDir = (target) => {
    const osMatch = ["darwin", "linux", "windows"].find((o) => target.includes(o));
    const os = osMatch === "windows" ? "win32" : osMatch || "";
    const env = ["musl", "gnu", "msvc"].find((e) => target.includes(e)) || "";
    const cpu = target.split("-")[0].replace("aarch64", "arm64").replace("x86_64", "x64");

    return `${os}-${cpu}${env ? `-${env}` : ""}`;
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

// 处理单个目标的初始化
const initTarget = async (target) => {
  const npm_dir_name = platformDir(target),
    sub_pkg_name = `${main_name}-${npm_dir_name}`,
    sub_pkg_dir = join(npm_dir, npm_dir_name);

  if (await pkgExists(sub_pkg_name)) return;

  console.log(`\n[INIT] 初始化缺失的包: ${sub_pkg_name}`);
  if (!existsSync(sub_pkg_dir)) mkdirSync(sub_pkg_dir, { recursive: true });

  const [os, cpu] = npm_dir_name.split("-"),
    binary_name = napi.binaryName || "index",
    main = `${binary_name}.${npm_dir_name}.node`,
    sub_pkg = {
      name: sub_pkg_name,
      version,
      os: [os],
      cpu: [cpu],
      main,
      files: [main],
      description: `Native binary package for ${sub_pkg_name}`,
      repository: pkg.repository,
      license: pkg.license,
      publishConfig: { access: "public" },
    };

  write(join(sub_pkg_dir, "package.json"), JSON.stringify(sub_pkg, null, 2) + "\n");

  const node_file = join(sub_pkg_dir, main);
  if (!existsSync(node_file)) write(node_file, "");

  console.log(`[OK] 已初始化本地目录: ${sub_pkg_dir}`);
};

// 遍历处理所有目标
for (const target of targets) {
  await initTarget(target);
}

console.log("\n初始化检查完成。");
