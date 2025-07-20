# mdz-cli

[English](README.md) | 中文版本

用于创建和管理 MDZ（Markdown Zipped）文档的命令行工具。

## 概述

`mdz-cli` 是一个全面的命令行界面，用于处理 MDZ 文件。它提供创建、解压、验证和预览 MDZ 文档所需的所有基本操作。

## 安装

### 从源码安装

```bash
git clone https://github.com/mdz-format/mdz.git
cd mdz
cargo install --path mdz-cli
```

### 本地构建

```bash
cargo build --release
# 二进制文件位置: ./target/release/mdz-cli
```

## 快速开始

### 创建 MDZ 文件

```bash
# 从目录结构创建
mdz create document.mdz --from ./my-document

# 详细输出
mdz create document.mdz --from ./my-document --verbose

# 自定义压缩级别
mdz create document.mdz --from ./my-document --compression 9
```

### 解压 MDZ 文件

```bash
# 解压到目录
mdz extract document.mdz --to ./extracted

# 强制覆盖现有文件
mdz extract document.mdz --to ./extracted --force

# 详细输出
mdz extract document.mdz --to ./extracted --verbose
```

### 验证 MDZ 文件

```bash
# 基本验证
mdz validate document.mdz

# 详细验证报告
mdz validate document.mdz --detailed

# 严格模式（警告视为错误）
mdz validate document.mdz --strict
```

### 预览 MDZ 文件

```bash
# 生成 HTML 预览
mdz preview document.mdz

# 自动在浏览器中打开
mdz preview document.mdz --browser

# 保存到指定文件
mdz preview document.mdz --output preview.html

# 使用自定义 CSS
mdz preview document.mdz --css custom.css

# 禁用 CSS
mdz preview document.mdz --no-css
```

## 命令

### `mdz create`

从源目录创建 MDZ 文件。

```
mdz create <OUTPUT> --from <SOURCE>

参数:
  <OUTPUT>              输出 MDZ 文件路径

选项:
  -f, --from <SOURCE>   包含 markdown 和资源的源目录
  -c, --compression <LEVEL>  压缩级别 (0-9) [默认: 6]
  -v, --verbose         显示详细输出
  -h, --help            打印帮助
```

**源目录结构：**
```
my-document/
├── main.md           # 必需：主要 markdown 内容
├── img/              # 可选：图片资源
│   ├── photo.jpg
│   └── diagram.png
└── css/              # 可选：样式表
    └── style.css     # 自动加载的主样式表
```

**示例：**

```bash
# 基本创建
mdz create report.mdz --from ./report-source

# 高压缩用于分发
mdz create report.mdz --from ./report-source --compression 9

# 详细进度显示
mdz create report.mdz --from ./report-source --verbose
```

### `mdz extract`

将 MDZ 文件解压到目录。

```
mdz extract <INPUT> --to <OUTPUT>

参数:
  <INPUT>               要解压的 MDZ 文件

选项:
  -t, --to <OUTPUT>     输出目录
  -f, --force           覆盖现有文件
  -v, --verbose         显示详细输出
  -h, --help            打印帮助
```

**示例：**

```bash
# 解压到新目录
mdz extract document.mdz --to ./extracted

# 覆盖现有文件
mdz extract document.mdz --to ./extracted --force

# 显示解压进度
mdz extract document.mdz --to ./extracted --verbose
```

### `mdz validate`

验证 MDZ 文件格式和结构。

```
mdz validate <INPUT>

参数:
  <INPUT>               要验证的 MDZ 文件

选项:
  -d, --detailed        显示详细验证报告
  -s, --strict          将警告视为错误
  -h, --help            打印帮助
```

**验证检查：**
- ✅ ZIP 文件完整性
- ✅ 必需的 `main.md` 文件存在
- ✅ 目录结构合规性
- ✅ 文件命名约定
- ✅ 图片格式支持
- ✅ CSS 结构验证

**示例：**

```bash
# 快速验证
mdz validate document.mdz

# 详细报告和文件列表
mdz validate document.mdz --detailed

# 严格验证（警告导致失败）
mdz validate document.mdz --strict
```

**示例输出：**

```
🔍 验证 MDZ 文件...

📋 结构检查:
  ✓ main.md 存在
  ✓ img/ 目录
  ✓ css/ 目录
  ✓ css/style.css

📁 文件摘要:
  图片文件: 3
  CSS 文件: 1

  🖼️ 图片:
    img/hero.jpg
    img/diagram.png
    img/icon.svg

  🎨 CSS 文件:
    css/style.css

📊 总体状态:
  ✓ 有效的 MDZ 格式

✓ 验证通过
```

### `mdz preview`

生成 MDZ 文档的 HTML 预览。

```
mdz preview <INPUT>

参数:
  <INPUT>               要预览的 MDZ 文件

选项:
  -b, --browser         自动在浏览器中打开
  -o, --output <FILE>   输出 HTML 文件路径
  -c, --css <FILE>      要使用的自定义 CSS 文件
  -n, --no-css          禁用 CSS 样式
  -h, --help            打印帮助
```

**功能：**
- 🎨 完整的 CSS 样式支持
- 🖼️ 嵌入的 base64 图片
- 📱 响应式设计
- 🌐 跨平台浏览器打开
- 💾 临时或持久 HTML 输出

**示例：**

```bash
# 生成并在浏览器中打开
mdz preview document.mdz --browser

# 保存到指定文件
mdz preview document.mdz --output report.html

# 使用自定义样式
mdz preview document.mdz --css themes/dark.css --browser

# 无样式的纯 HTML
mdz preview document.mdz --no-css --output plain.html
```

## 全局选项

所有命令都支持这些全局选项：

- `-h, --help` - 显示帮助信息
- `-V, --version` - 显示版本信息
- `--verbose` - 启用详细输出（如适用）

## 文件格式

### 支持的输入结构

创建 MDZ 文件时，源目录应遵循此结构：

```
source-directory/
├── main.md           # 必需：主要 Markdown 内容
├── img/              # 可选：图片目录
│   ├── *.jpg, *.jpeg # JPEG 图片
│   ├── *.png         # PNG 图片  
│   ├── *.gif         # GIF 动画
│   ├── *.svg         # SVG 图形
│   └── *.webp        # WebP 图片
└── css/              # 可选：样式表目录
    ├── style.css     # 主样式表（自动加载）
    └── *.css         # 其他样式表
```

### Markdown 内容指南

您的 `main.md` 应使用标准 Markdown 语法：

```markdown
# 文档标题

## 介绍

这是一个带有嵌入资源的示例 MDZ 文档。

![示例图片](img/sample.png)

## 功能

- **粗体文本**
- *斜体文本*
- [链接](https://example.com)
- `代码片段`

### 代码块

```rust
fn main() {
    println!("Hello, MDZ!");
}
```

### 表格

| 功能 | 状态 | 优先级 |
|------|------|--------|
| 解析器 | ✅   | 高     |
| CLI   | ✅   | 高     |
| Web   | 🚧   | 中     |
```

### 图片指南

为获得最佳效果：

- **格式**：照片使用 JPG，带透明度的图形使用 PNG，可缩放图形使用 SVG
- **大小**：单个图片保持在 2MB 以下以获得更好性能
- **命名**：使用描述性名称如 `hero-image.jpg`，而不是 `img1.jpg`
- **组织**：为不同图片类型使用子目录：
  ```
  img/
  ├── photos/
  ├── diagrams/
  └── icons/
  ```

### CSS 指南

对于样式：

- **主样式表**：将主要样式放在 `css/style.css`（自动加载）
- **响应式**：为不同屏幕大小设计
- **打印友好**：考虑打印媒体查询
- **变量**：使用 CSS 自定义属性进行主题化

```css
/* css/style.css */
:root {
  --primary-color: #3498db;
  --text-color: #333;
  --background: #fff;
}

body {
  font-family: system-ui, sans-serif;
  color: var(--text-color);
  background: var(--background);
}

@media (max-width: 768px) {
  body {
    padding: 16px;
    font-size: 16px;
  }
}
```

## 工作流程

### 基本文档创建

1. **准备内容**：
   ```bash
   mkdir my-document
   cd my-document
   ```

2. **创建 main.md**：
   ```bash
   echo "# 我的文档" > main.md
   echo "这是我的 MDZ 文档。" >> main.md
   ```

3. **添加资源**：
   ```bash
   mkdir img css
   cp ~/photos/header.jpg img/
   echo "body { font-family: Arial; }" > css/style.css
   ```

4. **创建 MDZ**：
   ```bash
   cd ..
   mdz create my-document.mdz --from my-document
   ```

5. **验证和预览**：
   ```bash
   mdz validate my-document.mdz
   mdz preview my-document.mdz --browser
   ```

### 批量处理

处理多个文档：

```bash
#!/bin/bash
for dir in documents/*/; do
  name=$(basename "$dir")
  echo "处理 $name..."
  mdz create "output/${name}.mdz" --from "$dir"
  mdz validate "output/${name}.mdz"
done
```

## 许可证

MIT 许可证 - 详见 LICENSE 文件。