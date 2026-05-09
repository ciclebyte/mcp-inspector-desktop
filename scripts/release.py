#!/usr/bin/env python3
"""读取 .version 并执行发布。

用法：
    python scripts/release.py

前置条件：
    1. 已运行 gen_release_notes.py 生成 .version
    2. 已人工审核 CHANGELOG.md

执行流程：
    1. 读取 .version 获取版本号
    2. 校验 CHANGELOG 中包含对应版本条目
    3. 同步更新 package.json / tauri.conf.json / Cargo.toml 版本号
    4. git commit + tag + push
    5. 清理 .version
"""

import json
import os
import re
import subprocess
import sys


def run(cmd, check=True):
    result = subprocess.run(
        cmd, shell=True, capture_output=True, text=True, encoding="utf-8"
    )
    if check and result.returncode != 0:
        print(f"错误: {cmd}\n{result.stderr.strip()}")
        sys.exit(1)
    return result.stdout.strip()


def update_json_version(filepath, version):
    """用 json 模块更新 JSON 文件的 version 字段。"""
    with open(filepath, "r", encoding="utf-8") as f:
        data = json.load(f)
    if data.get("version") == version:
        print(f"  {filepath} 版本号已是 {version}，跳过")
        return False
    data["version"] = version
    with open(filepath, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=2, ensure_ascii=False)
        f.write("\n")
    print(f"  已更新 {filepath}")
    return True


def update_cargo_version(filepath, version):
    """正则替换 Cargo.toml 中 [package] 段的 version 字段。"""
    with open(filepath, "r", encoding="utf-8") as f:
        content = f.read()
    new_content = re.sub(
        r'^(version\s*=\s*)"[^"]*"',
        rf'\g<1>"{version}"',
        content,
        count=1,
        flags=re.MULTILINE,
    )
    if new_content == content:
        print(f"  {filepath} 版本号未变更")
        return False
    with open(filepath, "w", encoding="utf-8") as f:
        f.write(new_content)
    print(f"  已更新 {filepath}")
    return True


def main():
    root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    version_file = os.path.join(root, ".version")

    # 1. 读取 .version
    if not os.path.exists(version_file):
        print("未找到 .version 文件，请先运行：")
        print("  python scripts/gen_release_notes.py")
        sys.exit(1)

    with open(version_file, "r", encoding="utf-8") as f:
        version = f.read().strip()

    if not version:
        print(".version 文件为空")
        sys.exit(1)

    tag = f"v{version}"

    # 2. 校验 CHANGELOG
    changelog_path = os.path.join(root, "CHANGELOG.md")
    if not os.path.exists(changelog_path):
        print("CHANGELOG.md 不存在，请先运行 gen_release_notes.py")
        sys.exit(1)

    with open(changelog_path, "r", encoding="utf-8") as f:
        changelog = f.read()
    if f"## {version}" not in changelog:
        print(f"CHANGELOG.md 中未找到 {version} 的条目")
        sys.exit(1)

    # 3. 检查工作区（排除 .version）
    status = run("git status --porcelain", check=False)
    dirty = [l for l in status.splitlines() if not l.endswith(".version")]
    if dirty:
        print("存在未提交的更改（.version 除外），请先处理：")
        for l in dirty:
            print(f"  {l}")
        sys.exit(1)

    # 4. 检查 tag
    tags = run("git tag -l", check=False)
    if tag in tags.splitlines():
        print(f"标签 {tag} 已存在")
        sys.exit(1)

    print(f"准备发布 {tag} ...")

    # 5. 更新版本号
    update_json_version(os.path.join(root, "package.json"), version)
    update_json_version(os.path.join(root, "src-tauri", "tauri.conf.json"), version)
    update_cargo_version(os.path.join(root, "src-tauri", "Cargo.toml"), version)

    # 6. Git 操作
    run("git add package.json src-tauri/tauri.conf.json src-tauri/Cargo.toml CHANGELOG.md")
    run(f'git commit -m "chore: release v{version}"')
    run(f"git tag {tag}")

    print(f"推送代码和标签 {tag} ...")
    run("git push origin master")
    run(f"git push origin {tag}")

    # 7. 清理 .version
    os.remove(version_file)
    print(f"\n发布完成！{tag}")
    print(f"https://github.com/cicbyte/mcp-inspector-desktop/releases/tag/{tag}")


if __name__ == "__main__":
    main()
