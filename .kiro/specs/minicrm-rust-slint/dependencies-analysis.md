# MiniCRM Rust + Slint 依赖分析

## 核心依赖库分析

基于28个功能需求，以下是各功能模块所需的Rust依赖库：

## 1. GUI框架和界面相关

### Slint UI框架
```toml
slint = "1.3"
```
**用途**: 
- 需求9: 用户界面优化
- 需求10: 数据仪表盘
- 所有UI相关需求

### 图表和可视化
```toml
plotters = "0.3"
plotters-slint = "0.1"  # Slint集成
resvg = "0.37"          # SVG渲染
```
**用途**:
- 需求10: 数据仪表盘（各种图表）
- 需求11: 产品售后跟踪（统计图表）
- 需求13: 产品报价管理（价格趋势图）

## 2. 数据库和存储相关

### SQLite数据库
```toml
rusqlite = { version = "0.30", features = ["bundled"] }
r2d2 = "0.8"            # 连接池
r2d2_sqlite = "0.23"    # SQLite连接池
```
**用途**:
- 需求8: 数据库结构设计
- 需求1-28: 所有数据存储需求

### 数据库迁移
```toml
refinery = { version = "0.8", features = ["rusqlite"] }
```
**用途**:
- 需求8: 数据库版本升级和迁移

## 3. 序列化和数据处理

### JSON/配置处理
```toml
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
toml = "0.8"
```
**用途**:
- 需求5: 数据导入导出
- 配置文件管理
- API数据交换

### 数据验证
```toml
validator = { version = "0.16", features = ["derive"] }
```
**用途**:
- 需求1: 客户信息管理（数据验证）
- 需求18: 供应商信息管理（数据验证）
- 所有表单验证需求

## 4. 文档处理相关

### Excel文件处理
```toml
calamine = "0.22"       # 读取Excel
rust_xlsxwriter = "0.49" # 写入Excel
```
**用途**:
- 需求5: 数据导入导出（Excel格式）
- 需求14: 合同管理（Excel报表）

### Word文档处理
```toml
docx-rs = "0.4"
```
**用途**:
- 需求14: 合同管理（Word文档生成）
- 需求13: 产品报价管理（报价单生成）

### PDF文档处理
```toml
printpdf = "0.6"
```
**用途**:
- 需求13: 产品报价管理（PDF报价单）
- 需求24: 供应商询价管理（PDF询价单）

## 5. 日期时间处理

```toml
chrono = { version = "0.4", features = ["serde"] }
```
**用途**:
- 需求4: 日程跟踪和提醒管理
- 需求3: 互动记录管理
- 需求15: 客户交流事件跟踪
- 所有时间相关功能

## 6. 异步和并发处理

```toml
tokio = { version = "1.0", features = ["full"] }
futures = "0.3"
```
**用途**:
- 需求7: 数据安全和备份（后台备份）
- 需求4: 提醒管理（定时任务）
- UI响应性优化

## 7. 错误处理

```toml
thiserror = "1.0"
anyhow = "1.0"
```
**用途**:
- 所有需求的错误处理
- 用户友好的错误消息

## 8. 日志记录

```toml
tracing = "0.1"
tracing-subscriber = "0.3"
```
**用途**:
- 需求7: 数据安全（操作审计）
- 系统监控和调试

## 9. 唯一标识符

```toml
uuid = { version = "1.0", features = ["v4", "serde"] }
```
**用途**:
- 需求13: 产品报价管理（报价编号）
- 需求14: 合同管理（合同编号）
- 需求15: 交流事件跟踪（事件编号）

## 10. 正则表达式

```toml
regex = "1.0"
```
**用途**:
- 需求2: 客户搜索和筛选
- 需求19: 供应商搜索和筛选
- 数据验证

## 11. 图像处理

```toml
image = "0.24"
```
**用途**:
- 需求10: 数据仪表盘（图表导出）
- UI图标处理

## 12. 网络和HTTP（可选）

```toml
reqwest = { version = "0.11", features = ["json"] }
```
**用途**:
- 未来扩展：云端同步
- 第三方API集成

## 13. 加密和安全

```toml
sha2 = "0.10"
```
**用途**:
- 需求7: 数据安全（数据完整性校验）
- 备份文件校验

## 14. 文件系统操作

```toml
walkdir = "2.0"
```
**用途**:
- 需求7: 数据备份（文件管理）
- 需求5: 数据导入导出（文件处理）#
# 完整的Cargo.toml依赖配置

```toml
[package]
name = "minicrm"
version = "0.1.0"
edition = "2021"

[dependencies]
# GUI框架
slint = "1.3"

# 数据库
rusqlite = { version = "0.30", features = ["bundled"] }
r2d2 = "0.8"
r2d2_sqlite = "0.23"
refinery = { version = "0.8", features = ["rusqlite"] }

# 序列化和数据处理
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
toml = "0.8"
validator = { version = "0.16", features = ["derive"] }

# 文档处理
calamine = "0.22"
rust_xlsxwriter = "0.49"
docx-rs = "0.4"
printpdf = "0.6"

# 日期时间
chrono = { version = "0.4", features = ["serde"] }

# 异步处理
tokio = { version = "1.0", features = ["full"] }
futures = "0.3"

# 错误处理
thiserror = "1.0"
anyhow = "1.0"

# 日志
tracing = "0.1"
tracing-subscriber = "0.3"

# 工具库
uuid = { version = "1.0", features = ["v4", "serde"] }
regex = "1.0"

# 图表和可视化
plotters = "0.3"
resvg = "0.37"
image = "0.24"

# 安全
sha2 = "0.10"

# 文件系统
walkdir = "2.0"

[dev-dependencies]
# 测试框架
tokio-test = "0.4"
tempfile = "3.0"
```

## 按功能需求分组的依赖映射

### 客户管理功能 (需求1-17)

**需求1 - 客户信息管理**
- `rusqlite`, `r2d2_sqlite` (数据存储)
- `validator` (数据验证)
- `slint` (UI界面)

**需求2 - 客户搜索和筛选**
- `regex` (搜索匹配)
- `slint` (UI界面)

**需求3 - 互动记录管理**
- `chrono` (时间处理)
- `rusqlite` (数据存储)
- `slint` (时间线UI)

**需求4 - 日程跟踪和提醒管理**
- `chrono` (日期时间)
- `tokio` (定时任务)
- `slint` (提醒UI)

**需求5 - 数据导入导出**
- `calamine` (Excel读取)
- `rust_xlsxwriter` (Excel写入)
- `serde_json` (JSON处理)
- `docx-rs` (Word文档)

**需求6 - 跨平台兼容性**
- `slint` (跨平台UI)
- 所有依赖库的跨平台支持

**需求7 - 数据安全和备份**
- `sha2` (数据校验)
- `walkdir` (文件管理)
- `tokio` (后台备份)

**需求8 - 数据库结构设计**
- `rusqlite` (SQLite数据库)
- `refinery` (数据库迁移)
- `r2d2` (连接池)

**需求9 - 用户界面优化**
- `slint` (现代化UI)
- `image` (图标处理)

**需求10 - 数据仪表盘**
- `plotters` (图表生成)
- `slint` (仪表盘UI)
- `resvg` (SVG渲染)

**需求11 - 产品售后跟踪**
- `plotters` (统计图表)
- `chrono` (保修期计算)
- `rusqlite` (售后记录存储)

**需求12 - 货款授信和收款管理**
- `rusqlite` (财务数据存储)
- `chrono` (逾期计算)
- `plotters` (财务图表)

**需求13 - 产品报价管理**
- `uuid` (报价编号)
- `printpdf` (PDF报价单)
- `docx-rs` (Word报价单)
- `plotters` (价格趋势图)

**需求14 - 合同管理**
- `uuid` (合同编号)
- `docx-rs` (合同文档)
- `rust_xlsxwriter` (合同报表)
- `chrono` (合同日期管理)

**需求15 - 客户交流事件跟踪处理**
- `uuid` (事件编号)
- `chrono` (事件时间)
- `plotters` (事件统计图表)

**需求16 - 客户分级管理**
- `rusqlite` (等级数据存储)
- `plotters` (等级分析图表)

**需求17 - 产品客户分类管理**
- `rusqlite` (分类数据存储)
- `plotters` (分类统计图表)

### 供应商管理功能 (需求18-28)

**需求18-28** 使用与客户管理类似的依赖库组合，主要包括：
- `rusqlite` (供应商数据存储)
- `slint` (供应商管理UI)
- `plotters` (供应商分析图表)
- `chrono` (时间相关功能)
- `uuid` (询价单、合同编号)
- `printpdf`, `docx-rs` (文档生成)

## 依赖库版本兼容性说明

1. **Slint 1.3**: 最新稳定版本，支持现代化UI特性
2. **rusqlite 0.30**: 支持最新SQLite特性，内存安全
3. **tokio 1.0**: 成熟的异步运行时
4. **serde 1.0**: 稳定的序列化框架
5. **chrono 0.4**: 广泛使用的日期时间库

所有依赖库都选择了稳定版本，确保项目的可靠性和长期维护性。## 文档
模板生成详细方案

### Word文档模板生成

#### 方案1: 使用docx-rs + 模板引擎
```toml
docx-rs = "0.4"
tera = "1.19"           # 模板引擎
```

**实现方式**:
1. 创建Word模板文件(.docx)，使用占位符如`{{customer_name}}`
2. 使用`tera`模板引擎处理占位符替换
3. 使用`docx-rs`读取模板文件，替换内容，生成新文档

**优点**: 
- 支持复杂的Word格式
- 可以使用现有的Word模板文件
- 模板可视化编辑

#### 方案2: 使用docx-rs直接生成
```toml
docx-rs = "0.4"
```

**实现方式**:
1. 使用`docx-rs`的API直接构建文档结构
2. 程序化添加段落、表格、样式等
3. 适合简单格式的文档

### Excel文档模板生成

#### 方案1: 使用rust_xlsxwriter + 模板数据
```toml
rust_xlsxwriter = "0.49"
serde = { version = "1.0", features = ["derive"] }
```

**实现方式**:
1. 定义Excel模板的数据结构
2. 使用`rust_xlsxwriter`创建工作簿和工作表
3. 程序化填充数据、设置格式、添加图表

#### 方案2: 使用calamine读取 + rust_xlsxwriter写入
```toml
calamine = "0.22"      # 读取Excel模板
rust_xlsxwriter = "0.49" # 写入新Excel
```

**实现方式**:
1. 使用`calamine`读取Excel模板文件
2. 解析模板结构和占位符
3. 使用`rust_xlsxwriter`基于模板生成新文件

### 推荐的模板生成架构

```toml
# 文档模板处理
docx-rs = "0.4"
rust_xlsxwriter = "0.49"
calamine = "0.22"
tera = "1.19"           # 模板引擎
handlebars = "4.4"      # 备选模板引擎

# 模板数据处理
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### 具体功能需求的模板实现

#### 需求13 - 产品报价管理
**Word报价单模板**:
```rust
// 使用tera模板引擎
let template = r#"
报价单

客户名称: {{customer_name}}
报价日期: {{quote_date}}
有效期: {{valid_until}}

产品清单:
{% for item in items %}
- {{item.name}}: {{item.quantity}} x {{item.price}} = {{item.total}}
{% endfor %}

总计: {{total_amount}}
"#;
```

**Excel报价单模板**:
```rust
// 使用rust_xlsxwriter
let workbook = Workbook::new();
let worksheet = workbook.add_worksheet(Some("报价单"));

// 添加标题和数据
worksheet.write_string(0, 0, "报价单")?;
worksheet.write_string(1, 0, "客户名称")?;
worksheet.write_string(1, 1, &customer_name)?;
// ... 更多数据填充
```

#### 需求14 - 合同管理
**Word合同模板**:
```rust
// 合同模板结构
#[derive(Serialize)]
struct ContractTemplate {
    contract_number: String,
    customer_name: String,
    contract_date: String,
    amount: String,
    terms: Vec<String>,
}

// 使用docx-rs生成
let mut doc = Docx::new();
doc.add_paragraph().add_run().add_text(&format!("合同编号: {}", template.contract_number));
```

### 模板文件管理

#### 目录结构
```
templates/
├── word/
│   ├── quote_template.docx      # 报价单模板
│   ├── contract_template.docx   # 合同模板
│   └── report_template.docx     # 报表模板
├── excel/
│   ├── customer_export.xlsx     # 客户导出模板
│   ├── financial_report.xlsx    # 财务报表模板
│   └── supplier_analysis.xlsx   # 供应商分析模板
└── config/
    ├── template_config.toml     # 模板配置
    └── field_mappings.json      # 字段映射
```

#### 模板配置管理
```toml
# template_config.toml
[word_templates]
quote = "templates/word/quote_template.docx"
contract = "templates/word/contract_template.docx"

[excel_templates]
customer_export = "templates/excel/customer_export.xlsx"
financial_report = "templates/excel/financial_report.xlsx"

[template_fields]
quote = ["customer_name", "quote_date", "items", "total_amount"]
contract = ["contract_number", "customer_name", "contract_date", "amount"]
```

### 更新后的完整依赖配置

```toml
[dependencies]
# 文档处理和模板
docx-rs = "0.4"
rust_xlsxwriter = "0.49"
calamine = "0.22"
tera = "1.19"              # 主要模板引擎
handlebars = "4.4"         # 备选模板引擎

# 其他依赖保持不变...
```

这种方案提供了灵活的模板系统，支持：
1. **可视化模板编辑** - 使用Word/Excel直接编辑模板
2. **动态内容生成** - 基于数据动态填充模板
3. **格式保持** - 保持原有的文档格式和样式
4. **扩展性** - 易于添加新的模板类型