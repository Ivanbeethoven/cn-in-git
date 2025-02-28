# 中文代码检测器（Chinese Code Detector）

[(GitHub)](https://crates.io/crates/chinese-detector)


一个高效检测代码文件中中文字符的Rust工具，支持.gitignore规则和多种配置方式。

## ✨ 功能特性

- 🕵️ 精准检测中文 Unicode 字符（基本区+扩展A区）
- 📜 输出包含中文的具体行号
- 🛡️ 自动遵守.gitignore排除规则
- 📦 支持多种配置方式（命令行/配置文件）
- 🔍 智能编码检测（UTF-8/GBK/BIG5等）
- 📁 多扩展名支持（可配置检测文件类型）
- 🚀 高性能（基于Rust实现）

## 📦 安装

### 从Cargo安装
```bash
cargo install chinese-detector
```

### 从源码安装
```bash
git clone https://github.com/yourname/chinese-detector.git
cd chinese-detector
cargo build --release
```
## 🚀 使用方式
```bash
cdetector  # 检测当前目录
cdetector path/to/project  # 检测指定目录
cdetector --extensions rs,cpp,md #指定扩展名
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