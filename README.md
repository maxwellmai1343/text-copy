# Text Copy App

基于 Tauri 2 + Svelte 的文本管理桌面应用，支持添加、修改、删除文本，点击即可复制到剪贴板。

## 技术栈

### 前端
| 技术 | 用途 |
|------|------|
| Svelte 4 | UI 框架 |
| TypeScript | 类型安全 |
| Vite 5 | 构建工具 |
| @tauri-apps/api | Tauri 前后端通信 |

### 后端
| 技术 | 用途 |
|------|------|
| Rust | 系统编程语言 |
| Tauri 2 | 桌面应用框架 |
| serde/serde_json | JSON 序列化 |
| chrono | 时间处理 |

## 已安装依赖

### npm 依赖 (38个)
```json
{
  "@tauri-apps/api": "^2",
  "svelte": "^4.2.0",
  "@sveltejs/vite-plugin-svelte": "^3.0.0",
  "typescript": "^5.2.0",
  "vite": "^5.0.0"
}
```

### Rust 依赖 (443个)
```toml
tauri = "2"
tauri-build = "2"
serde = "1"
serde_json = "1"
chrono = "0.4"
```

## 运行方式

### 前端开发
```bash
# 启动开发服务器（支持热重载）
npm run dev

# 生产构建
npm run build

# 预览生产构建
npm run preview
```

### 后端开发
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

## 项目结构

```
text-copy/
├── src/                    # 前端源码
│   ├── main.ts             # 入口文件
│   ├── App.svelte          # 主组件
│   └── index.css           # 全局样式
├── src-tauri/              # 后端源码
│   ├── src/
│   │   └── main.rs         # Rust 后端
│   ├── Cargo.toml          # Rust 依赖
│   └── tauri.conf.json     # Tauri 配置
├── package.json            # npm 配置
├── vite.config.ts          # Vite 配置
├── svelte.config.js        # Svelte 配置
├── tsconfig.json           # TypeScript 配置
└── AGENTS.md               # 代码库规范
```

## 功能特性

- [x] 添加文本
- [x] 修改文本
- [x] 删除文本
- [x] 点击复制到剪贴板
- [x] 数据持久化（JSON格式）
- [x] 无障碍支持（键盘导航）

## 优势

- **打包体积小**: Svelte 编译后仅 ~9KB（React 约 145KB）
- **性能优秀**: 无虚拟 DOM，直接操作真实 DOM
- **语法简洁**: 响应式、条件渲染、循环都很简单

## 创建文件列表

- package.json
- vite.config.ts
- svelte.config.js
- tsconfig.json
- index.html
- src/main.ts
- src/App.svelte
- src/index.css
- src-tauri/Cargo.toml
- src-tauri/tauri.conf.json
- src-tauri/src/main.rs
- src-tauri/build.rs
- src-tauri/icons/icon.png
- src-tauri/icons/32x32.png
- .gitignore
- AGENTS.md
- README.md
