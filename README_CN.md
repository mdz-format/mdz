# MDZ - Markdown 压缩文档格式

[English](README.md) | 中文版本

MDZ（Markdown Zipped）是一种自包含的文档打包格式，旨在解决 Markdown 文档的资源管理问题。它将 Markdown 内容、图片和样式打包到单个文件中，同时完全兼容标准 Markdown 语法。

## 概述

- **自包含**: 所有资源都在一个文件中
- **Markdown 兼容**: 完全兼容标准 Markdown 语法
- **最小复杂度**: 只包含三个核心组件（main.md、img/、css/）
- **自动化优先**: 自动资源加载，最少手动配置
- **开放标准**: 基于开放的 ZIP 和 Markdown 标准

## 项目结构

```
MDZ/
├── mdz-core/                 # 核心解析和渲染库
├── mdz-cli/                 # 命令行工具
│   ├── src/                 # CLI 源代码
│   ├── Cargo.toml          # CLI 依赖
│   └── README.md           # CLI 文档
├── MDZ_SPEC.md             # 格式规范 v1.0
├── Cargo.toml              # 工作区配置
└── README.md               # 此文件
```

## 快速开始

### 安装

```bash
# 克隆仓库
git clone https://github.com/mdz-format/mdz.git
cd mdz

# 从源码安装
cargo install --path mdz-cli

# 或本地构建使用
cargo build --release
# 二进制文件位置: ./target/release/mdz-cli
```

### 基本用法

```bash
# 从包含 main.md、img/、css/ 的目录创建
mdz create document.mdz --from ./my-document

# 解压 MDZ 文件
mdz extract document.mdz --to ./extracted

# 验证 MDZ 格式
mdz validate document.mdz --detailed

# 在浏览器中预览
mdz preview document.mdz --browser

# 生成 HTML 输出预览
mdz preview document.mdz --output document.html
```

### MDZ 文件结构

```
example.mdz (ZIP 文件)
├── main.md        # 主要 Markdown 内容 (必需)
├── img/           # 图片资源 (可选)
│   ├── photo.jpg
│   └── diagram.png
└── css/           # 样式资源 (可选)
    └── style.css  # 自动加载的主样式表
```

## 功能特性

### 核心库 (mdz-core)

- ✅ ZIP 文件解析和验证
- ✅ 使用 pulldown-cmark 的 Markdown 解析
- ✅ 带嵌入资源的 HTML 渲染
- ✅ 自动 CSS 加载
- ✅ 图片资源处理（base64 嵌入）
- ✅ 全面的格式验证
- ✅ 错误处理和报告

### CLI 工具 (mdz-cli)

- ✅ 从目录创建 MDZ 文件
- ✅ 将 MDZ 文件解压到目录
- ✅ 验证 MDZ 格式合规性
- ✅ 生成带嵌入资源的 HTML 预览
- ✅ 跨平台浏览器打开
- ✅ 彩色输出和进度报告
- ✅ 详细模式和错误消息

## 支持的功能

### Markdown 语法
- 标题、段落、强调
- 列表（有序、无序、任务列表）
- 代码块与语法高亮
- 表格和引用块
- 链接和图片
- HTML 内联元素

### 图片格式
- JPEG、PNG、GIF、SVG、WebP
- HTML 输出中自动 base64 嵌入
- 支持子目录组织

### CSS 支持
- 自动加载 `css/style.css`
- 自定义 CSS 覆盖选项
- 响应式设计支持
- 打印友好样式

## 开发

### 构建

```bash
# 构建所有包
cargo build

# 发布模式构建
cargo build --release

# 构建特定包
cargo build -p mdz-core
cargo build -p mdz-cli
```

### 示例测试

```bash
# 创建测试 MDZ 文件
./target/release/mdz-cli create test.mdz --from test-document --verbose

# 验证
./target/release/mdz-cli validate test.mdz --detailed

# 浏览器预览
./target/release/mdz-cli preview test.mdz --browser
```

## 文档

- [**格式规范**](MDZ_SPEC.md) - 完整的 MDZ 格式规范 v1.0
- [**核心库 API**](mdz-core/README.md) - mdz-core 库文档
- [**CLI 工具指南**](mdz-cli/README.md) - mdz-cli 命令行工具指南

## 性能

- 支持高达 100MB 的文件
- 快速解析和渲染（100MB 文件 < 2 秒）
- 内存高效的流式处理
- 针对开发和生产使用优化

## 安全性

- ZIP 炸弹保护
- 路径遍历攻击防护
- 文件大小限制
- HTML 渲染中的 XSS 保护

## 兼容性

- **Rust**: 1.70+
- **平台**: macOS、Linux、Windows
- **格式**: MDZ 规范 v1.0
- **标准**: ZIP、Markdown、HTML5、CSS3

## 贡献

1. Fork 仓库
2. 创建功能分支
3. 进行更改
4. 如适用，添加测试
5. 提交 Pull Request

## 许可证

MIT 许可证 - 详见 LICENSE 文件。

## 状态

**当前版本**: 0.1.0  
**状态**: 生产就绪 ✅  
**最后更新**: 2025-07-21