# MDZ格式规范 v1.0

[English](MDZ_SPEC.md) | 中文

## 1. 概述

MDZ（Markdown Zipped）是一种自包含的文档打包格式，旨在解决Markdown文档中图片和样式资源管理的问题。MDZ格式基于ZIP压缩，保持与传统Markdown的最大兼容性。

### 1.1 设计原则

- **Markdown兼容**: 核心内容完全兼容标准Markdown语法
- **最小复杂度**: 只包含必要的三个核心组件
- **自包含**: 单文件包含所有依赖资源
- **自动化优先**: 减少手动配置，渲染器自动处理资源加载
- **开放标准**: 基于开放的ZIP和Markdown标准

### 1.2 文件扩展名

- **推荐**: `.mdz`
- **MIME类型**: `application/mdz+zip`

## 2. 核心结构

MDZ文件是一个ZIP压缩包，包含以下**三个核心部分**：

```
example.mdz (ZIP压缩包)
├── main.md        # 主要Markdown内容 (必需)
├── img/           # 图片资源目录 (可选)
└── css/           # 样式资源目录 (可选)
```

### 2.1 文件结构说明

#### 2.1.1 main.md (必需)
- **作用**: 包含文档的主要Markdown内容
- **格式**: 标准Markdown语法
- **编码**: UTF-8
- **命名**: 必须为`main.md`（大小写敏感）

#### 2.1.2 img/ (可选)
- **作用**: 存放文档中引用的所有图片
- **支持格式**: jpg, jpeg, png, gif, svg, webp
- **引用方式**: `![alt text](img/filename.ext)`
- **子目录**: 支持子目录结构

#### 2.1.3 css/ (可选)
- **作用**: 存放自定义样式文件
- **支持格式**: css
- **自动加载**: 渲染器自动查找并加载 `css/style.css`
- **默认样式**: 如果不提供CSS文件，使用系统默认Markdown样式

## 3. 详细规范

### 3.1 main.md 规范

#### 3.1.1 内容要求
```markdown
# 文档标题

这是一个标准的Markdown文档。

![示例图片](img/example.png)

## 章节标题

支持所有标准Markdown语法：
- 列表
- **粗体**
- *斜体*
- `代码`
- [链接](https://example.com)

```

#### 3.1.2 图片引用
```markdown
# 相对路径引用（推荐）
![图片描述](img/photo.jpg)
![带子目录](img/screenshots/screen1.png)

# 支持的图片格式
![JPEG图片](img/image.jpeg)
![PNG图片](img/image.png)
![GIF动图](img/animation.gif)
![SVG矢量图](img/icon.svg)
![WebP图片](img/modern.webp)
```


### 3.2 img/ 目录规范

#### 3.2.1 目录结构
```
img/
├── photo1.jpg
└── photo2.png

```

#### 3.2.2 文件命名
- **字符限制**: 使用ASCII字符、数字、连字符、下划线
- **避免空格**: 使用连字符或下划线替代空格
- **小写推荐**: 推荐使用小写文件名
- **有意义**: 文件名应该有描述性

**示例**:
```
✅ 推荐:
- hero-image.jpg
- user_avatar.png
- button-icon.svg

❌ 避免:
- 图片 1.jpg (包含空格和中文)
- IMG_001.JPG (全大写)
- a.png (无意义命名)
```

### 3.3 css/ 目录规范

#### 3.3.1 自动加载机制
- **核心文件**: `css/style.css` 为默认样式文件
- **自动加载**: MDZ渲染器自动查找并加载此文件
- **无需引用**: 不需要在main.md中手动引用CSS文件
- **优先级**: 如果存在style.css，自动应用；否则使用系统默认样式

#### 3.3.2 目录结构（v1.0基础版本）
```
css/
└── style.css       # 自动加载的主样式文件
```

#### 3.3.3 样式文件示例
```css
/* css/style.css */
body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Arial, sans-serif;
    line-height: 1.6;
    max-width: 800px;
    margin: 0 auto;
    padding: 20px;
}

h1, h2, h3 {
    color: #333;
}

img {
    max-width: 100%;
    height: auto;
}

code {
    background-color: #f5f5f5;
    padding: 2px 4px;
    border-radius: 3px;
}
```

## 4. 兼容性规范

### 4.1 Markdown兼容性

MDZ格式的main.md文件必须是完全有效的Markdown文档，可以被任何标准Markdown解析器正确解析。

#### 4.1.1 支持的Markdown特性
- **基础语法**: 标题、段落、强调、列表、链接、图片
- **扩展语法**: 表格、代码块、任务列表
- **HTML内联**: 支持内联HTML标签

#### 4.1.2 路径解析规则
```markdown
# 相对路径（MDZ格式内部）
![内部图片](img/photo.jpg)

# 绝对URL（外部资源）
![外部图片](https://example.com/image.jpg)

# 相对路径（传统Markdown兼容）
![同级目录](./photo.jpg)  # 解析为 img/photo.jpg
![上级目录](../images/photo.jpg)  # 不推荐，但支持
```

**CSS加载说明**:
- CSS样式文件不需要在Markdown中引用
- 渲染器自动查找 `css/style.css` 并应用
- 保持Markdown内容的纯净性

### 4.2 ZIP格式兼容性

#### 4.2.1 压缩要求
- **压缩方法**: 标准ZIP压缩（deflate）
- **压缩级别**: 推荐使用中等压缩（级别6）
- **文件编码**: UTF-8文件名编码

#### 4.2.2 解压兼容性
MDZ文件可以被任何标准ZIP工具解压：
```bash
# 解压MDZ文件
unzip document.mdz -d extracted/

# 查看内容
cd extracted/
cat main.md
ls img/
ls css/
```

## 5. 创建和使用

### 5.1 手动创建MDZ文件

#### 5.1.1 创建步骤
```bash
# 1. 创建工作目录
mkdir my-document
cd my-document

# 2. 创建main.md文件
echo "# 我的文档" > main.md
echo "![示例](img/example.png)" >> main.md

# 3. 创建资源目录
mkdir img css

# 4. 添加图片和样式
cp /path/to/image.png img/example.png
echo "body { font-family: Arial; }" > css/style.css

# 5. 打包为MDZ文件
zip -r ../my-document.mdz main.md img/ css/
```

#### 5.1.2 验证MDZ文件
```bash
# 验证ZIP结构
unzip -l document.mdz

# 应该显示类似：
# Archive:  document.mdz
#   Length      Date    Time    Name
# ---------  ---------- -----   ----
#      123  2025-01-01 10:00   main.md
#        0  2025-01-01 10:00   img/
#     5432  2025-01-01 10:00   img/example.png
#        0  2025-01-01 10:00   css/
#       89  2025-01-01 10:00   css/style.css
```

### 5.2 内容指南

#### 5.2.1 main.md 编写指南
```markdown
# 文档标题

## 介绍
这是文档的介绍部分。

## 主要内容
![重要图表](img/chart.png)

详细说明文字...

### 子章节
- 要点1
- 要点2

## 结论
总结内容。

---
*创建时间: 2025-01-01*
```

#### 5.2.2 图片优化建议
- **分辨率**: 根据用途选择合适分辨率
- **格式选择**: 照片用JPEG，图表用PNG，图标用SVG
- **文件大小**: 单个图片建议不超过2MB
- **总体大小**: 整个MDZ文件建议不超过50MB

## 6. 实现建议

### 6.1 解析器实现

#### 6.1.1 基本解析流程
```
1. 验证ZIP文件格式
2. 检查main.md是否存在
3. 解析main.md内容
4. 识别img/目录，解析图片引用关系
5. 自动查找css/style.css文件
6. 如果存在style.css，自动加载样式
7. 渲染最终输出（应用自定义样式或默认样式）
```

#### 6.1.2 错误处理
- **缺少main.md**: 抛出格式错误
- **损坏的ZIP**: 提示文件损坏
- **缺失资源**: 显示占位符或警告
- **编码问题**: 尝试自动检测编码

### 6.2 工具建设建议

#### 6.2.1 命令行工具
```bash
# 创建MDZ文件
mdz create my-document.mdz --from ./source-folder

# 解压MDZ文件
mdz extract document.mdz --to ./output-folder

# 验证MDZ文件
mdz validate document.mdz

# 预览MDZ文件
mdz preview document.mdz --browser
```

#### 6.2.2 编辑器集成
- **VS Code插件**: 提供MDZ文件预览和编辑
- **在线编辑器**: Web版MDZ编辑器
- **桌面应用**: 专用MDZ编辑器

## 7. 版本控制

### 7.1 规范版本

当前规范版本：**v1.0**

### 7.2 向后兼容性

MDZ格式承诺：
- **主版本升级**: 可能包含不兼容变更
- **次版本升级**: 保持向后兼容
- **修订版本**: 仅修复错误和澄清

### 7.3 扩展计划

未来可能的扩展（保持向后兼容）：
- **CSS扩展**: 支持多CSS文件、主题切换、打印样式
- 支持JavaScript文件（js/目录）
- 支持字体文件（fonts/目录）
- 支持多语言文档（i18n/目录）
- 支持元数据文件（metadata.json）

## 8. 安全考虑

### 8.1 内容安全

- **脚本执行**: 默认不执行JavaScript代码
- **外部资源**: 谨慎处理外部URL引用
- **文件大小**: 限制解压后的文件大小
- **ZIP炸弹**: 防护ZIP炸弹攻击

### 8.2 隐私保护

- **元数据**: 清理ZIP文件中的敏感元数据
- **本地优先**: 鼓励本地处理，减少云端依赖
- **透明性**: 所有内容都是可检查的文本和资源

## 9. 最佳实践

### 9.1 文档组织

```markdown
# 推荐的main.md结构
# 文档标题

## 概述
简短介绍文档目的

## 目录
- [第一章](#第一章)
- [第二章](#第二章)
- [附录](#附录)

## 第一章
![概念图](img/concept.png)

内容...

## 第二章
详细内容...

## 附录
补充材料...
```

### 9.2 资源管理

```
# 推荐的目录结构
img/
├── diagrams/     # 图表和示意图
├── screenshots/  # 截图
├── photos/       # 照片
└── icons/        # 图标

css/
└── style.css     # 自动加载的主样式文件

# 未来版本可能支持:
# ├── print.css   # 打印样式 (v2.0计划)
# └── themes/     # 主题样式 (v2.0计划)
```

### 9.3 性能优化

- **图片压缩**: 使用适当的压缩比
- **CSS精简**: 移除未使用的样式
- **文件命名**: 使用描述性的文件名
- **目录结构**: 合理组织资源文件

---

## 结语

MDZ格式旨在成为一个简单、实用、兼容的Markdown文档打包标准。通过main.md + img/ + css/的极简架构，既保持了与传统Markdown的完全兼容，又解决了资源管理的核心问题。
