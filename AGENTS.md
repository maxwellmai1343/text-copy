# AGENTS.md - Text Copy App 代码库规范

## 重要提示

**所有回答必须使用中文**，包括代码中的注释也必须使用中文编写。

## 项目概述

这是一个基于 Tauri 2 + Svelte 的文本管理桌面应用，用于管理文本片段。项目包含：
- **前端**: Svelte 4 + TypeScript + Vite
- **后端**: Rust (Tauri 2) 用于文件 I/O 和命令处理

## 构建命令

### 前端（Svelte/TypeScript）

```bash
# 启动开发服务器（支持热重载）
npm run dev

# 生产构建
npm run build

# 预览生产构建
npm run preview
```

### 后端（Rust/Tauri）

```bash
# 开发运行（支持热重载）
cd src-tauri && cargo run

# 发布构建
cd src-tauri && cargo build --release

# 检查编译错误
cd src-tauri && cargo check

# 运行测试
cd src-tauri && cargo test

# 按名称运行单个测试
cd src-tauri && cargo test <test_name>
```

## 代码风格规范

### Svelte/TypeScript

#### 导入顺序
- 外部包使用绝对导入
- 内部组件/样式使用相对导入
- 分组导入：Tauri API → Svelte → 外部库 → 内部模块

```typescript
// 正确
import { invoke } from '@tauri-apps/api/core';
import { onMount } from 'svelte';
import './index.css';

// 错误
import './index.css';
import { invoke } from '@tauri-apps/api/core';
```

#### 命名规范
- **接口**: PascalCase，带描述性名称（`TextItem`、`UserConfig`）
- **变量/函数**: camelCase（`handleAddText`、`loadedTexts`）
- **常量**: SCREAMING_SNAKE_CASE（`DATA_FILE`）
- **CSS 类名**: kebab-case，BEM 风格（`text-item`、`text-actions`）

#### 类型规范
- 启用严格 TypeScript 模式（tsconfig.json 中 strict: true）
- 函数参数和返回值使用显式类型
- 避免使用 `any` 类型；使用 `unknown` 并配合类型守卫
- 为所有数据结构定义接口

```typescript
// 正确
interface TextItem {
  id: number;
  content: string;
  created_at: string;
}

async function handleAddText(): Promise<void> {
  // 实现
}

// 错误 - 松散类型
async function handleAddText() {
  // 实现
}
```

#### 格式化
- 使用 2 空格缩进
- 分号可选（Svelte/TypeScript 不强制要求）
- 字符串使用单引号

#### 组件结构
- 使用 Svelte 4 语法（`<script lang="ts">`）
- 响应式变量使用普通赋值（`count = count + 1`）
- 复杂逻辑提取到独立函数
- 条件判断使用 `{#if}` / `{#each}` 块

### Rust

#### 导入规范
- 按 crate 分组导入
- 同一 crate 的多个项目使用嵌套导入

```rust
// 正确
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

// 错误
use std::fs::read_to_string;
use serde::Serialize;
use std::path::PathBuf;
```

#### 命名规范
- **结构体/枚举**: PascalCase（`TextItem`、`CommandError`）
- **函数**: snake_case（`load_texts`、`get_data_path`）
- **常量**: SCREAMING_SNAKE_CASE（`DATA_FILE`）
- **变量**: snake_case（`new_item`、`data_path`）

#### 错误处理
- Tauri 命令使用 `Result<T, String>`（不用 anyhow::Error）
- 使用 `?` 操作符传播错误
- 使用 `.map_err(|e| e.to_string())` 转换为字符串

```rust
// 正确
#[tauri::command]
async fn load_texts() -> Result<Vec<TextItem>, String> {
    let content = fs::read_to_string(data_path)
        .map_err(|e| e.to_string())?;
    serde_json::from_str(&content).map_err(|e| e.to_string())
}

// 错误 - 使用 panic
#[tauri::command]
async fn load_texts() -> Vec<TextItem> {
    fs::read_to_string(data_path).unwrap()
}
```

#### 代码结构
- Tauri 命令标记为 `async`
- 保持函数专注且小型（建议不超过 50 行）
- 为常用 trait 使用派生宏（Debug、Clone、Serialize、Deserialize）

## 项目结构

```
text-copy/
├── src/
│   ├── main.ts                # 入口文件
│   ├── App.svelte             # 主 Svelte 组件
│   └── index.css              # 全局样式
├── src-tauri/
│   ├── src/
│   │   └── main.rs            # Rust 后端（Tauri 命令）
│   ├── Cargo.toml             # Rust 依赖
│   └── tauri.conf.json        # Tauri 配置
├── package.json               # npm 脚本和依赖
├── vite.config.ts             # Vite 配置
├── svelte.config.js           # Svelte 配置
└── tsconfig.json              # TypeScript 配置
```

## 关键约定

1. **中文注释**: 所有代码注释必须使用中文
2. **无注释原则**: 除非绝对必要，否则避免添加注释
3. **数据持久化**: 文本项以 JSON 格式存储在项目根目录（data.json）
4. **剪贴板功能**: 使用浏览器的 `navigator.clipboard` API（Web API）
5. **Tauri 命令**: 所有暴露给前端的 Rust 函数使用 `#[tauri::command]` 属性
6. **无外部 CDN**: 所有依赖都是本地 npm 包
7. **事件处理**: 使用 `on:click`、`on:keydown` 等 Svelte 事件语法
8. **事件修饰符**: 使用 `|stopPropagation`、`|preventDefault` 等修饰符

## 测试

目前项目没有自动化测试。添加测试时：
- Rust: 在 `src-tauri/src/main.rs` 中使用 `#[cfg(test)]` 模块
- Svelte/TypeScript: 使用 Vitest 或 Testing Library for Svelte
