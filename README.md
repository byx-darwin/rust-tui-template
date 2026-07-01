# Rust TUI Template

[![CI](https://github.com/byx-darwin/rust-tui-template/actions/workflows/build.yml/badge.svg)](https://github.com/byx-darwin/rust-tui-template/actions/workflows/build.yml)

一个现代化的 Rust 终端用户界面（TUI）项目模板，基于 `ratatui` + `crossterm`，包含完整的开发工具链和最佳实践。

## 快速开始

### 1. 生成新项目

```bash
cargo generate --git https://github.com/byx-darwin/rust-tui-template
```

按提示输入项目名称（如 `my-dashboard`）、作者信息和项目描述。

### 2. 进入项目目录

```bash
cd my-dashboard
```

### 3. 安装开发工具

```bash
make install-tools
```

### 4. 构建和测试

```bash
# 构建项目
make build

# 运行测试
make test

# 代码检查和格式化
make lint
```

### 5. 运行项目

```bash
# 直接运行
cargo run

# 使用模拟数据运行
cargo run -- --demo

# 或者使用 make
make run
make demo
```

## 项目结构

```
.
├── apps/
│   └── tui/          # TUI 应用入口
├── crates/
│   └── core/         # 核心业务逻辑库
├── docs/             # 项目文档
├── specs/            # 功能规格说明
├── Makefile          # 常用命令集合
└── Cargo.toml        # 工作区配置
```

## 开发工作流

```bash
# 监听文件变化自动测试
make test-watch

# 运行完整检查
make check

# 生成 API 文档
make doc
```

## 终端要求

| 特性          | 要求                             |
|---------------|----------------------------------|
| 真彩色        | 推荐（COLORTERM=truecolor）      |
| 终端尺寸      | 80x24 或更大                     |
| Unicode       | 必需（用于边框字符）             |

支持 `NO_COLOR` 环境变量，自动降级显示。

## 特性

- ✅ Rust 2024 Edition
- ✅ ratatui + crossterm 后端
- ✅ 严格的 lint 配置（clippy pedantic）
- ✅ 完整的测试框架（rstest, proptest）
- ✅ 自动 CI/CD（GitHub Actions）
- ✅ 依赖安全检查（cargo-audit, cargo-deny）
- ✅ 结构化日志（tracing）
- ✅ 错误处理最佳实践（thiserror, miette）

## 许可证

MIT — 详见 [LICENSE.md](LICENSE.md)。
