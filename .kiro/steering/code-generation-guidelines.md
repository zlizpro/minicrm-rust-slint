# 全栈开发助手指导方针

> **全局上下文文件** - 适用于所有项目开发和技术栈

## 🎯 核心原则

### 交互目标
1. **中文优先** - 全部使用中文输出对话内容和代码注释
2. **技术适配** - 根据项目技术栈调整开发方式和最佳实践
3. **质量保证** - 保持技术方案的一致性和可维护性
4. **效率优化** - 优先使用本地工具，合理使用MCP工具

### 任务目标
生成满足专业开发标准且易于理解的代码，包含全面注释说明，适合团队协作和长期维护。支持多种编程语言和技术栈，确保代码质量和项目成功。

## 🔧 开发工作流程

### 1. 问题分析和方案设计（必须使用Sequential Thinking）

#### 必须深度思考的情况
- **复杂功能设计** - 涉及多个模块交互或复杂业务逻辑
- **技术方案选择** - 需要在多个实现方案中做出选择
- **架构设计决策** - 影响系统整体结构的重要决策
- **问题诊断分析** - 复杂的bug定位或性能问题分析
- **需求理解** - 需求不明确或存在多种理解可能性

#### 思考过程结构
```rust
struct ThinkingProcess {
    problem_analysis: String,           // 问题分析和需求理解
    solution_exploration: String,       // 多方案探索和对比
    risk_assessment: String,           // 风险识别和应对策略
    implementation_planning: String,    // 具体实施计划制定
    validation_strategy: String,       // 验证和测试策略
    technology_selection: String,      // 技术栈选择和理由
    performance_considerations: String, // 性能考虑和优化策略
}
```

### 2. 技术验证和工具使用（MCP工具优化策略）

#### 本地工具优先级（减少Token消耗）
```rust
// 本地工具使用优先级
const LOCAL_TOOLS_PRIORITY: &[(&str, &[&str])] = &[
    ("file_operations", &["readFile", "readMultipleFiles", "strReplace"]),
    ("search_tools", &["grepSearch", "fileSearch"]),
    ("code_analysis", &["listDirectory", "executeBash"]),
    ("git_operations", &["git_status", "git_diff", "git_log"]),
    ("build_tools", &["cargo", "npm", "swift build", "xcodebuild"]),
    ("test_tools", &["cargo test", "pytest", "jest", "swift test"]),
];
```

#### MCP工具使用策略
```rust
// MCP工具使用优先级和场景
enum MCPUsagePriority {
    High {
        context7: "验证框架API、语言特性、最佳实践",
        github: "开源库调研、代码示例参考",
        aws_docs: "云服务集成、部署方案验证",
        memory: "项目知识管理、决策记录",
    },
    Medium {
        code_generation: "复杂算法实现、样板代码生成",
        documentation: "API文档生成、用户手册编写",
    },
    Low {
        testing: "测试用例生成、Mock数据创建",
    }
}
```

#### Token优化原则
1. **批量查询** - 将相关问题合并为一次查询
2. **精确搜索** - 使用具体的技术栈和版本信息
3. **缓存复用** - 避免重复查询相同信息（24小时缓存）
4. **本地优先** - 优先使用本地工具验证和分析
5. **结果限制** - 限制查询结果数量，只获取高质量信息

### 3. 代码实现和质量保证

#### 通用开发规则
1. **架构设计** - 模块化设计、SOLID原则、分层架构、低耦合高内聚
2. **代码质量** - 详细注释、描述性命名、错误处理、可测试性
3. **语言特定** - 遵循各语言官方风格指南和最佳实践

#### 多语言支持配置
```rust
enum SupportedLanguage {
    Rust { cargo_version: String, edition: String },
    Swift { version: String, platform: String, package_manager: String },
    Python { version: String, package_manager: String },
    JavaScript { runtime: String, package_manager: String },
    TypeScript { version: String, compiler_options: String },
    Go { version: String, modules: bool },
    Java { version: String, build_tool: String },
    CSharp { framework: String, version: String },
}
```

## 📋 输出格式标准

### 文档结构（所有代码文档必须包含）
1. **技术概述** - 技术栈选择、架构设计、核心理念
2. **实现方案** - 完整代码，每个组件都有详细注释
3. **集成指南** - 如何集成到现有项目或独立部署
4. **测试策略** - 单元测试、集成测试、性能测试方案
5. **扩展路径** - 如何进一步扩展功能或优化性能
6. **维护指南** - 常见问题、调试方法、更新策略

### 代码注释要求
- **文件头注释** - 说明文件用途、主要功能、依赖关系
- **函数注释** - 使用语言标准格式（Rust doc comments、Python docstring、Swift documentation comments等）
- **复杂逻辑注释** - 解释算法思路、业务逻辑、设计决策
- **性能注释** - 标记性能关键代码和优化点

### 代码格式要求
- 遵循语言官方格式化标准（rustfmt、black、prettier、swift-format等）
- 使用一致的命名约定和代码结构
- 保持适当的代码密度和空白行使用

## 🛠️ 代码质量保证策略

### 1. 自动化工具配置

#### Rust项目质量配置
```toml
# Cargo.toml - 代码质量设置
[lints.rust]
unsafe_code = "forbid"
missing_docs = "warn"

[lints.clippy]
all = "warn"
pedantic = "warn"
unwrap_used = "deny"    # 禁止使用unwrap
expect_used = "deny"    # 禁止使用expect
panic = "deny"          # 禁止使用panic

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

#### Swift项目质量配置
```json
// .swift-format配置
{
  "lineLength": 100,
  "indentation": { "spaces": 4 },
  "rules": {
    "AlwaysUseLowerCamelCase": true,
    "DoNotUseSemicolons": true,
    "OrderedImports": true,
    "UseShorthandTypeNames": true
  }
}
```

### 2. Git Hooks集成
```bash
#!/bin/sh
# .git/hooks/pre-commit - 提交前质量检查

# Rust项目检查
if [ -f "Cargo.toml" ]; then
    cargo fmt --check || exit 1
    cargo clippy --all-targets -- -D warnings || exit 1
    cargo test || exit 1
    cargo audit || exit 1
fi

# Swift项目检查
if [ -f "Package.swift" ]; then
    swiftlint || exit 1
    swift-format lint --recursive Sources/ || exit 1
    swift build || exit 1
fi
```

### 3. CI/CD质量检查
```yaml
# .github/workflows/quality.yml
name: Code Quality
on: [push, pull_request]

jobs:
  quality-check:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
    - name: Multi-language quality check
      run: |
        # 根据项目类型运行相应的质量检查
        if [ -f "Cargo.toml" ]; then
          cargo fmt --check
          cargo clippy -- -D warnings
          cargo test
          cargo audit
        fi
```

## 🎯 项目类型适配

### 桌面应用
- **Rust+Slint** - 内存安全、高性能、跨平台
- **Swift+AppKit** - macOS原生体验
- **Electron** - Web技术栈、快速开发

### 移动应用
- **Swift+SwiftUI** - iOS原生、声明式UI
- **React Native** - 跨平台、Web技术复用
- **Flutter** - 高性能、统一UI

### Web应用
- **React/Vue/Angular** - 现代前端框架
- **Node.js** - JavaScript全栈
- **Rust+WASM** - 高性能Web应用

### 后端服务
- **Rust** - 高性能、内存安全
- **Node.js** - 快速开发、生态丰富
- **Python** - 数据处理、AI集成
- **Go** - 并发处理、微服务

## 🔍 错误排除和调试策略

### 常见错误分类
```rust
enum ErrorCategory {
    Compilation {
        types: "语法错误、类型错误、生命周期错误",
        tools: vec!["编译器提示", "语言服务器", "静态分析工具"],
    },
    Runtime {
        types: "panic、内存错误、逻辑错误",
        tools: vec!["调试器", "日志分析", "性能分析器"],
    },
    Performance {
        types: "内存泄漏、CPU占用高、响应慢",
        tools: vec!["性能分析器", "内存分析器", "基准测试"],
    },
    Dependencies {
        types: "版本冲突、缺失依赖、安全漏洞",
        tools: vec!["依赖管理器", "安全审计工具", "版本检查工具"],
    },
}
```

### 调试工具配置
```rust
struct DebuggingConfig {
    logging: LoggingConfig {
        level: "debug",
        format: "structured",
        output: vec!["stdout", "file"],
    },
    profiling: ProfilingConfig {
        cpu_profiler: "perf",
        memory_profiler: "valgrind",
        flame_graph: true,
    },
    error_tracking: ErrorTrackingConfig {
        stack_trace: true,
        error_context: true,
        crash_reporting: true,
    },
}
```

## 📏 文件大小和结构规范

### 文件大小限制
```rust
struct FileSizeRules {
    // 单文件行数限制
    max_lines: FileTypeLimit {
        rust_source: 500,           // Rust源文件最大500行
        swift_source: 400,          // Swift源文件最大400行
        python_source: 300,         // Python源文件最大300行
        typescript_source: 400,     // TypeScript源文件最大400行
        config_file: 100,          // 配置文件最大100行
        test_file: 600,            // 测试文件可以稍大
    },
    
    // 单文件复杂度限制
    max_complexity: ComplexityLimit {
        functions_per_file: 20,     // 每文件最大函数数
        classes_per_file: 5,        // 每文件最大类数
        interfaces_per_file: 10,    // 每文件最大接口数
        enums_per_file: 8,         // 每文件最大枚举数
    },
}
```

### 大文件拆分策略
```rust
// 当文件超过限制时的拆分原则
enum FileSplitStrategy {
    ByFunctionality {
        description: "按功能模块拆分",
        example: "user_service.rs -> user_crud.rs + user_validation.rs + user_analytics.rs",
    },
    ByLayer {
        description: "按架构层次拆分", 
        example: "user.rs -> user_model.rs + user_repository.rs + user_service.rs",
    },
    ByFeature {
        description: "按特性拆分",
        example: "auth.rs -> login.rs + register.rs + password_reset.rs",
    },
    ByDataType {
        description: "按数据类型拆分",
        example: "models.rs -> user_model.rs + order_model.rs + product_model.rs",
    },
}
```

### 文件组织最佳实践
```rust
// 推荐的文件结构模式
struct FileOrganization {
    // 模块文件结构
    module_structure: ModulePattern {
        mod_file: "mod.rs - 模块声明和公共接口",
        lib_file: "lib.rs - 库的根模块",
        main_file: "main.rs - 应用程序入口",
        types_file: "types.rs - 类型定义",
        errors_file: "errors.rs - 错误类型",
        utils_file: "utils.rs - 工具函数",
        tests_file: "tests.rs - 单元测试",
    },
    
    // 命名约定
    naming_convention: NamingRules {
        snake_case: "文件名使用snake_case",
        descriptive: "文件名要描述其主要功能",
        no_abbreviation: "避免使用缩写",
        max_filename_length: 50,
    },
}
```

## 🔄 重复代码规范和检测

### 重复代码检测配置
```rust
struct DuplicationDetection {
    // 检测工具配置
    tools: DetectionTools {
        rust: vec!["cargo-duplicates", "tokei --sort code"],
        swift: vec!["swiftlint", "periphery"],
        python: vec!["pylint", "flake8", "bandit"],
        typescript: vec!["jscpd", "eslint"],
        universal: vec!["sonarqube", "codeclimate"],
    },
    
    // 检测阈值
    thresholds: DuplicationThresholds {
        min_duplicate_lines: 6,        // 最少6行才算重复
        min_duplicate_tokens: 50,      // 最少50个token才算重复
        similarity_percentage: 80.0,   // 80%相似度算重复
        ignore_comments: true,         // 忽略注释
        ignore_whitespace: true,       // 忽略空白字符
    },
}
```

### 重复代码重构策略
```rust
enum RefactoringStrategy {
    ExtractFunction {
        description: "提取公共函数",
        example: r#"
        // 重复代码
        fn validate_user_email(email: &str) -> bool { /* ... */ }
        fn validate_admin_email(email: &str) -> bool { /* ... */ }
        
        // 重构后
        fn validate_email(email: &str) -> bool { /* ... */ }
        "#,
    },
    
    ExtractTrait {
        description: "提取公共trait",
        example: r#"
        // 重复的方法实现
        impl User { fn save(&self) { /* ... */ } }
        impl Product { fn save(&self) { /* ... */ } }
        
        // 重构后
        trait Saveable { fn save(&self); }
        "#,
    },
    
    ExtractModule {
        description: "提取公共模块",
        example: r#"
        // 重复的工具函数散布在各处
        // 重构后统一放在utils模块
        mod utils {
            pub fn format_date() { /* ... */ }
            pub fn validate_input() { /* ... */ }
        }
        "#,
    },
    
    UseGenerics {
        description: "使用泛型减少重复",
        example: r#"
        // 重复的类型特定代码
        fn process_users(users: Vec<User>) { /* ... */ }
        fn process_products(products: Vec<Product>) { /* ... */ }
        
        // 重构后使用泛型
        fn process_items<T: Processable>(items: Vec<T>) { /* ... */ }
        "#,
    },
}
```

### 重复代码预防措施
```rust
struct DuplicationPrevention {
    // 代码审查检查点
    code_review_checklist: Vec<&'static str> = vec![
        "是否有相似的函数可以合并？",
        "是否可以提取公共接口？",
        "是否可以使用泛型或宏减少重复？",
        "是否有重复的业务逻辑？",
        "是否有重复的错误处理模式？",
    ],
    
    // 自动化检查
    automation: AutomationRules {
        pre_commit_hooks: "提交前运行重复代码检测",
        ci_pipeline: "CI流水线中集成重复代码检查",
        quality_gates: "代码质量门禁阻止高重复度代码合并",
        periodic_scan: "定期扫描整个代码库",
    },
}
```

## 🧩 模块复用设计规范

### 模块设计原则
```rust
struct ModuleDesignPrinciples {
    // SOLID原则在模块设计中的应用
    single_responsibility: "每个模块只负责一个功能领域",
    open_closed: "模块对扩展开放，对修改封闭",
    liskov_substitution: "子模块可以替换父模块",
    interface_segregation: "不要强迫模块依赖不需要的接口",
    dependency_inversion: "高层模块不应依赖低层模块",
    
    // 模块内聚性
    cohesion_types: CohesionLevel {
        functional: "最佳 - 模块内所有元素协作完成单一任务",
        sequential: "良好 - 一个元素的输出是另一个的输入", 
        communicational: "可接受 - 元素操作相同数据",
        procedural: "较差 - 元素按特定顺序执行",
        temporal: "差 - 元素在同一时间执行",
        logical: "最差 - 元素逻辑相关但功能不同",
    },
}
```

### 模块接口设计
```rust
// 模块接口设计模式
trait ModuleInterface {
    type Config;
    type Error;
    type Result<T> = std::result::Result<T, Self::Error>;
    
    // 模块初始化
    fn new(config: Self::Config) -> Self::Result<Self>;
    
    // 核心功能接口
    fn execute(&self) -> Self::Result<()>;
    
    // 可选的生命周期管理
    fn start(&mut self) -> Self::Result<()> { Ok(()) }
    fn stop(&mut self) -> Self::Result<()> { Ok(()) }
    fn health_check(&self) -> bool { true }
}

// 具体实现示例
struct DatabaseModule {
    connection: Connection,
    config: DatabaseConfig,
}

impl ModuleInterface for DatabaseModule {
    type Config = DatabaseConfig;
    type Error = DatabaseError;
    
    fn new(config: Self::Config) -> Self::Result<Self> {
        let connection = Connection::new(&config.url)?;
        Ok(Self { connection, config })
    }
    
    fn execute(&self) -> Self::Result<()> {
        // 数据库操作逻辑
        Ok(())
    }
}
```

### 依赖注入和控制反转
```rust
// 依赖注入容器设计
struct DIContainer {
    services: HashMap<TypeId, Box<dyn Any>>,
    factories: HashMap<TypeId, Box<dyn Fn() -> Box<dyn Any>>>,
}

impl DIContainer {
    // 注册服务
    fn register<T: 'static>(&mut self, service: T) {
        self.services.insert(TypeId::of::<T>(), Box::new(service));
    }
    
    // 注册工厂函数
    fn register_factory<T: 'static, F>(&mut self, factory: F) 
    where 
        F: Fn() -> T + 'static,
        T: 'static,
    {
        let factory_fn = move || -> Box<dyn Any> {
            Box::new(factory())
        };
        self.factories.insert(TypeId::of::<T>(), Box::new(factory_fn));
    }
    
    // 解析依赖
    fn resolve<T: 'static>(&self) -> Option<&T> {
        self.services.get(&TypeId::of::<T>())
            .and_then(|service| service.downcast_ref::<T>())
    }
}

// 使用示例
trait UserRepository {
    fn find_by_id(&self, id: u32) -> Option<User>;
}

struct DatabaseUserRepository {
    db: Arc<DatabaseModule>,
}

impl UserRepository for DatabaseUserRepository {
    fn find_by_id(&self, id: u32) -> Option<User> {
        // 数据库查询逻辑
        None
    }
}

struct UserService {
    repository: Arc<dyn UserRepository>,
}

impl UserService {
    fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }
}
```

### 模块版本管理和兼容性
```rust
struct ModuleVersioning {
    // 语义化版本控制
    semantic_versioning: SemVerRules {
        major: "破坏性变更时递增",
        minor: "向后兼容的功能添加时递增", 
        patch: "向后兼容的bug修复时递增",
    },
    
    // 接口兼容性策略
    compatibility_strategy: CompatibilityRules {
        deprecation_policy: "标记过时接口，保留2个版本后移除",
        migration_guide: "提供详细的迁移指南",
        backward_compatibility: "至少保持一个主版本的向后兼容",
        feature_flags: "使用特性标志控制新功能",
    },
    
    // 模块依赖管理
    dependency_management: DependencyRules {
        version_pinning: "精确指定依赖版本",
        update_strategy: "定期更新依赖，测试兼容性",
        security_updates: "及时应用安全更新",
        dependency_audit: "定期审计依赖安全性",
    },
}
```

## ⚡ 性能优化策略

### 编译时优化
- **Rust** - LTO、代码生成单元优化、目标CPU优化
- **Swift** - 编译器优化级别、链接时优化
- **其他语言** - 相应的编译器优化选项

### 运行时优化
- **内存管理** - 对象池、智能指针、避免循环引用
- **算法优化** - 选择合适的数据结构和算法
- **并发处理** - 异步编程、并行计算、线程池

### 代码质量度量
```rust
struct QualityMetrics {
    complexity: ComplexityMetrics {
        cyclomatic_complexity: 10,    // 圈复杂度限制
        function_length: 50,          // 函数长度限制
        nesting_depth: 4,            // 嵌套深度限制
    },
    file_size: FileSizeMetrics {
        max_lines_per_file: 500,      // 单文件最大行数
        max_functions_per_file: 20,   // 单文件最大函数数
        max_structs_per_file: 10,     // 单文件最大结构体数
    },
    code_duplication: DuplicationMetrics {
        max_duplicate_lines: 6,       // 最大重复行数
        similarity_threshold: 0.8,    // 相似度阈值
        min_token_match: 50,         // 最小匹配token数
    },
    test_coverage: CoverageMetrics {
        line_coverage: 80.0,          // 行覆盖率目标
        branch_coverage: 70.0,        // 分支覆盖率目标
    },
    dependencies: DependencyMetrics {
        security_vulnerabilities: 0,   // 零安全漏洞
        outdated_threshold: 30,       // 过期依赖阈值（天）
        max_dependency_depth: 5,      // 最大依赖深度
    },
}
```

## 📚 语言特定指导

### Rust开发要点
- 利用所有权系统确保内存安全
- 使用类型系统和trait进行抽象
- 重视错误处理和Result类型
- 利用cargo生态系统和工具链

### Swift开发要点
- 协议导向编程和值类型优先
- 使用SwiftUI进行声明式UI开发
- 合理使用async/await和Actor
- 遵循Apple平台设计规范

### Python开发要点
- 遵循PEP规范和类型注解
- 使用虚拟环境管理依赖
- 重视代码可读性和文档
- 利用丰富的第三方库生态

### JavaScript/TypeScript开发要点
- 使用现代ES特性和严格模式
- TypeScript提供类型安全
- 合理使用异步编程模式
- 遵循前端最佳实践

## 🎯 实施检查清单

### 开发前检查
- [ ] 明确项目技术栈和目标平台
- [ ] 设置代码质量工具和CI/CD
- [ ] 配置开发环境和依赖管理
- [ ] 建立项目结构和编码规范

### 开发中检查
- [ ] 使用Sequential Thinking分析复杂问题
- [ ] 优先使用本地工具，合理使用MCP工具
- [ ] 遵循代码质量标准和格式要求
- [ ] 编写测试用例和文档注释

### 开发后检查
- [ ] 运行所有质量检查工具
- [ ] 执行完整的测试套件
- [ ] 进行性能分析和优化
- [ ] 更新文档和部署指南

---

**此文件作为全局上下文，适用于所有项目开发。请在每次开发任务开始前参考此指导方针。**