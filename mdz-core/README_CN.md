# mdz-core

[English](README.md) | 中文版本

MDZ（Markdown Zipped）文档的核心解析和渲染库。

## 概述

`mdz-core` 提供处理 MDZ 文件的基础功能：
- 解析和验证 MDZ ZIP 压缩包
- 提取 Markdown 内容和嵌入资源
- 将 MDZ 文档渲染为 HTML，自动嵌入资源
- 格式验证和错误处理

## 功能特性

- **ZIP 压缩包处理**: 健壮的 ZIP 文件解析，完善的错误处理
- **Markdown 渲染**: 使用 `pulldown-cmark` 的完整 Markdown 渲染
- **资源嵌入**: HTML 输出中自动 base64 嵌入图片
- **CSS 自动加载**: 自动检测和应用 `css/style.css`
- **格式验证**: MDZ 格式合规性的全面验证
- **错误处理**: 带上下文的详细错误报告

## 安装

在您的 `Cargo.toml` 中添加：

```toml
[dependencies]
mdz-core = "0.1.0"
```

## 快速开始

### 基本用法

```rust
use mdz_core::{MdzParser, MdzRenderer};

// 解析 MDZ 文件
let document = MdzParser::parse_file("document.mdz")?;

// 渲染为 HTML
let renderer = MdzRenderer::new();
let html = renderer.render_html(&document)?;

println!("{}", html);
```

### 从字节数据解析

```rust
use mdz_core::MdzParser;

let mdz_data: Vec<u8> = std::fs::read("document.mdz")?;
let document = MdzParser::parse_bytes(&mdz_data)?;
```

### 格式验证

```rust
use mdz_core::MdzValidator;
use std::fs::File;

let file = File::open("document.mdz")?;
let result = MdzValidator::validate(file)?;

if result.is_valid() {
    println!("✅ 有效的 MDZ 格式");
} else {
    println!("❌ 验证错误: {:?}", result.errors);
}
```

### 自定义渲染选项

```rust
use mdz_core::{MdzRenderer, RenderOptions};

let options = RenderOptions {
    include_css: true,
    base64_images: true,
    custom_css: Some("body { background: #f0f0f0; }".to_string()),
    html_title: Some("我的文档".to_string()),
};

let renderer = MdzRenderer::with_options(options);
let html = renderer.render_html(&document)?;
```

## API 参考

### 核心类型

#### `MdzDocument`

表示已解析的 MDZ 文档：

```rust
pub struct MdzDocument {
    pub content: String,                           // Markdown 内容
    pub images: HashMap<String, Vec<u8>>,          // 图片资源
    pub css: Option<String>,                       // CSS 内容
    pub metadata: DocumentMetadata,                // 文档元数据
}
```

#### `DocumentMetadata`

解析过程中提取的文档元数据：

```rust
pub struct DocumentMetadata {
    pub title: Option<String>,                     // 从第一个标题提取
    pub created_at: Option<DateTime<Utc>>,         // 创建时间戳
    pub modified_at: Option<DateTime<Utc>>,        // 修改时间戳
    pub version: String,                           // 格式版本
}
```

#### `ValidationResult`

带详细信息的验证结果：

```rust
pub struct ValidationResult {
    pub has_main_md: bool,                         // main.md 存在性
    pub has_img_dir: bool,                         // img/ 目录存在性
    pub has_css_dir: bool,                         // css/ 目录存在性
    pub has_main_css: bool,                        // css/style.css 存在性
    pub image_files: Vec<String>,                  // 图片文件列表
    pub css_files: Vec<String>,                    // CSS 文件列表
    pub errors: Vec<String>,                       // 验证错误
    pub warnings: Vec<String>,                     // 验证警告
}
```

### 解析器 API

#### `MdzParser`

MDZ 文件的主解析器：

```rust
impl MdzParser {
    /// 从任何 Read + Seek 源解析
    pub fn parse<R: Read + Seek>(reader: R) -> Result<MdzDocument>;
    
    /// 从字节切片解析
    pub fn parse_bytes(data: &[u8]) -> Result<MdzDocument>;
    
    /// 从文件路径解析
    pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<MdzDocument>;
}
```

### 渲染器 API

#### `MdzRenderer`

带自定义选项的 HTML 渲染器：

```rust
impl MdzRenderer {
    /// 使用默认选项创建渲染器
    pub fn new() -> Self;
    
    /// 使用自定义选项创建渲染器
    pub fn with_options(options: RenderOptions) -> Self;
    
    /// 将文档渲染为 HTML
    pub fn render_html(&self, document: &MdzDocument) -> Result<String>;
    
    /// 渲染为纯文本（去除格式）
    pub fn render_text(&self, document: &MdzDocument) -> Result<String>;
}
```

#### `RenderOptions`

渲染自定义选项：

```rust
pub struct RenderOptions {
    pub include_css: bool,                         // 在输出中包含 CSS
    pub base64_images: bool,                       // 将图片嵌入为 base64
    pub custom_css: Option<String>,                // 覆盖 CSS
    pub html_title: Option<String>,                // HTML 文档标题
}
```

### 验证器 API

#### `MdzValidator`

MDZ 文件的格式验证器：

```rust
impl MdzValidator {
    /// 验证 MDZ 文件格式
    pub fn validate<R: Read + Seek>(reader: R) -> Result<ValidationResult>;
}
```

## 错误处理

所有操作返回 `Result<T, MdzError>`，其中 `MdzError` 提供详细的错误信息：

```rust
pub enum MdzError {
    Io(std::io::Error),                           // I/O 错误
    Zip(zip::result::ZipError),                   // ZIP 格式错误
    Json(serde_json::Error),                      // JSON 解析错误
    InvalidFormat(String),                        // 无效的 MDZ 格式
    MissingFile(String),                          // 缺少必需文件
    ResourceNotFound(String),                     // 资源未找到
    Validation(String),                           // 验证错误
    Parse(String),                                // 解析错误
    Render(String),                               // 渲染错误
}
```

## 完整示例

```rust
use mdz_core::{MdzParser, MdzRenderer, MdzValidator, RenderOptions};
use std::fs;

fn process_mdz_file(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 1. 验证文件
    let file = fs::File::open(path)?;
    let validation = MdzValidator::validate(file)?;
    
    if !validation.is_valid() {
        println!("验证错误:");
        for error in &validation.errors {
            println!("  ❌ {}", error);
        }
        return Ok(());
    }
    
    println!("✅ 验证通过");
    
    // 2. 解析文档
    let document = MdzParser::parse_file(path)?;
    
    println!("📄 文档: {}", 
        document.metadata.title.as_deref().unwrap_or("无标题"));
    println!("🖼️  图片: {}", document.images.len());
    println!("🎨 包含 CSS: {}", document.css.is_some());
    
    // 3. 渲染为 HTML
    let options = RenderOptions {
        include_css: true,
        base64_images: true,
        custom_css: None,
        html_title: document.metadata.title.clone(),
    };
    
    let renderer = MdzRenderer::with_options(options);
    let html = renderer.render_html(&document)?;
    
    // 4. 保存渲染的 HTML
    let output_path = path.replace(".mdz", ".html");
    fs::write(&output_path, html)?;
    
    println!("💾 HTML 已保存到: {}", output_path);
    
    Ok(())
}
```

## 依赖项

- `zip` - ZIP 压缩包处理
- `pulldown-cmark` - Markdown 解析和渲染
- `serde` + `serde_json` - 序列化支持
- `chrono` - 日期/时间处理
- `thiserror` - 错误处理
- `base64` - 图片的 Base64 编码
- `regex` - 文本处理
- `mime_guess` - MIME 类型检测

## 格式支持

支持 MDZ 格式规范 v1.0：

- **必需**: `main.md` 文件
- **可选**: 包含图片的 `img/` 目录（jpg, jpeg, png, gif, svg, webp）
- **可选**: 包含 `style.css` 的 `css/` 目录

## 性能

- 针对高达 100MB 的文件优化
- 大型压缩包的内存高效流式处理
- 资源的延迟加载
- 自动压缩检测

## 线程安全

所有公共 API 都是线程安全的，可以并发使用。

## 许可证

MIT 许可证 - 详见 LICENSE 文件。