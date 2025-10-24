# IFLOW 上下文信息

## 项目概述

这是一个使用 Tauri、Vue 3 和 TypeScript 构建的桌面应用程序。项目模板旨在帮助开发者快速上手，利用 Vite 作为构建工具。前端使用 Vue 3 的 `<script setup>` 语法，后端逻辑通过 Tauri 调用 Rust 代码实现。

## 技术栈

- **前端框架**: Vue 3 (使用 `<script setup>` SFC)
- **构建工具**: Vite
- **语言**: TypeScript
- **后端/桌面应用框架**: Tauri (Rust)
- **UI 库**: naive-ui (开发依赖)
- **其他依赖**: 
  - `@tauri-apps/api`: Tauri API 的 JavaScript 绑定
  - `@tauri-apps/plugin-opener`: Tauri 插件，用于打开文件或链接

## 项目结构

```
elasitc_client/
├── src/                  # Vue 前端源代码
├── src-tauri/            # Tauri 后端源代码 (Rust)
├── public/               # 静态资源
├── package.json          # Node.js 依赖和脚本
├── vite.config.ts        # Vite 配置
├── tauri.conf.json       # Tauri 配置
└── README.md             # 项目说明
```

## 构建和运行

### 开发环境

1. **安装依赖**:
   ```bash
   npm install
   ```

2. **启动开发服务器**:
   ```bash
   npm run tauri dev
   ```
   这将启动 Vite 开发服务器并打开 Tauri 桌面应用窗口。

### 构建生产版本

1. **构建项目**:
   ```bash
   npm run tauri build
   ```
   这将编译 Vue 前端，调用 Rust 编译器构建 Tauri 后端，并打包成最终的桌面应用程序。

### 测试

- **前端类型检查**:
  ```bash
  npm run build # vue-tsc 会在构建前进行类型检查
  ```
  目前项目中没有配置专门的测试脚本，但可以通过 `vue-tsc` 进行类型检查。

## 开发约定

- **前端开发**: 使用 Vue 3 的 Composition API (`<script setup>`) 和 TypeScript。
- **后端开发**: 使用 Rust 和 Tauri 框架。通过 `#[tauri::command]` 宏暴露 Rust 函数给前端调用。
- **API 调用**: 前端通过 `@tauri-apps/api` 的 `invoke` 函数调用后端 Rust 命令。
- **样式**: 使用 scoped CSS 和全局 CSS 变量进行样式管理。
- **IDE**: 推荐使用 VS Code 配合 Vue - Official、Tauri 和 rust-analyzer 插件。

## 关键文件说明

- `src/App.vue`: 主要的 Vue 应用组件，包含前端界面和与后端的交互逻辑。
- `src-tauri/src/lib.rs`: Tauri 后端入口，定义了可以被前端调用的 Rust 命令 (如 `greet`)。
- `package.json`: 定义了项目的 Node.js 依赖和 npm 脚本。
- `vite.config.ts`: Vite 构建工具的配置文件，针对 Tauri 开发进行了优化。
- `src-tauri/Cargo.toml`: Rust 项目的依赖和配置。