# 中文代码检测器（CN in Git Repository）

[(GitHub)](https://github.com/Ivanbeethoven/cn-in-git/tree/master)


一个高效检测代码文件中中文字符的Rust工具，支持.gitignore规则和多种配置方式。

## ✨ 功能特性

- 🕵️ 精准检测中文 Unicode 字符（基本区+扩展A区）
- 📜 输出包含中文的具体行号
- 🛡️ 自动遵守.gitignore排除规则
- 📦 支持多种配置方式（命令行/配置文件）
- 🔍 智能编码检测（UTF-8/GBK/BIG5等）
- 📁 多扩展名支持（可配置检测文件类型）
- 🚀 高性能（基于Rust实现）

## 快速开始
在你项目集成`cn-in-git`,用于检测代码中是否含有中文字符，以方便不小心忽视注释中残留的中文内容。
在github任意仓库中新建文件`.github/workflows/cig.yaml`
```yaml
name: Run CIG in Docker Container

on:
  workflow_dispatch: # manual
  push: 
    branches: [ "master" ] # Need Customize.

jobs:
  run-cig:
    runs-on: ubuntu-latest
    steps:
    - name: Checkout repository
      uses: actions/checkout@v4
      with:
        fetch-depth: 0 # pull the full commit.

    - name: Pull Docker image
      run: docker pull ghcr.io/ivanbeethoven/cn-in-git:master

    - name: Run CIG command with code mount
      run: |
        docker run \
          -v "${{ github.workspace }}:/workspace" \
          -w /workspace \
          --rm \
          ghcr.io/ivanbeethoven/cn-in-git:master \
          cig
```
## 📦 安装

### 从Github Release下载
[Linux-v0.1](https://github.com/Ivanbeethoven/cn-in-git/releases/download/v0.1.0/cig-linux-v0.1.0)
[Windows-v0.1](https://github.com/Ivanbeethoven/cn-in-git/releases/download/v0.1.0/cn-in-git-win-v0.1.0.exe)

### 从源码安装
```bash
git clone https://github.com/yourname/chinese-detector.git
cd chinese-detector
cargo build --release
```
## 🚀 使用方式
```bash
cig  # 检测当前目录
cig path/to/project  # 检测指定目录
cig --extensions rs,cpp,md #指定扩展名
```
### 排除文件
将需要排除的路径添加到.gitignore文件


### 📌 示例输出
```
发现中文时
Scanning directory: /projects/sample

Chinese characters found in the following files:
src/main.rs : Line numbers [3, 7, 12]
docs/README.md : Line numbers [5]

Error: Chinese characters are included in the code files
```

PS: 本项目大部分代码由DeepSeek/R1生成.,具体prompt请查看[网页快照](https://github.com/Ivanbeethoven/cn-in-git/blob/master/doc/SiliconCloud.html)