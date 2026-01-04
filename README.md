# UM 启动项管理器

[![Tauri](https://img.shields.io/badge/Tauri-2.0-blue)](https://tauri.app/)
[![Vue](https://img.shields.io/badge/Vue-3.5-green)](https://vuejs.org/)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.6-blue)](https://www.typescriptlang.org/)
[![Rust](https://img.shields.io/badge/Rust-2021-orange)](https://www.rust-lang.org/)

一个基于 Tauri 2.0 和 Vue 3 构建的现代化 Windows 启动项管理工具。

## 🎯 核心功能

- 查看和管理系统启动项
- 启用/禁用启动项
- 删除不需要的启动项
- 自动最小化启动程序

## 🛠️ 技术栈

| 技术           | 版本 | 用途               |
| -------------- | ---- | ------------------ |
| **Tauri**      | 2.0  | 跨平台桌面应用框架 |
| **Vue**        | 3.5  | 前端框架           |
| **TypeScript** | 5.6  | 类型安全           |
| **Rust**       | 2021 | 后端逻辑           |
| **Vite**       | 6.0  | 构建工具           |

## 🚀 快速开始

### 环境要求

- Node.js >= 18.0.0
- Rust >= 1.68
- Windows 10/11 (主要支持平台)

### 华为云镜像：

```bash
https://mirrors.huaweicloud.com/home
```

### NPM 加速

```bash
npm config set registry https://mirrors.huaweicloud.com/npm/
```

### Rust 安装加速

- 设置环境变量：

```bash
RUSTUP_DIST_SERVER https://mirrors.huaweicloud.com/rustup/
RUSTUP_UPDATE_ROOT https://mirrors.huaweicloud.com/rustup/rustup/
```

- 下载安装器安装：

```bash
https://mirrors.huaweicloud.com/rustup/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe
```

### Cargo 加速

- 新建文件 C:/Users/用户名/.cargo/config.toml 填入如下内容：

```bash
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = 'aliyun'

[source.aliyun]
registry = "sparse+https://mirrors.aliyun.com/crates.io-index/"
```

### 适用于 C++ 的 Visual Studio 生成工具

```bash
1.下载：https://visualstudio.microsoft.com/zh-hans/visual-cpp-build-tools/
2.运行并勾选：“Desktop development with C++”（必须）确保子组件中包含：
  - MSVC v14x - VS 2022 C++ x64/x86 生成工具（对应版本）
  - Windows 10/11 SDK（根据系统选择）
  - C++ 生成工具核心功能
```

### 安装依赖

```bash
# 克隆项目
git clone <repository-url>
cd um-startup-manager

# 安装前端依赖
npm install

# 安装 Rust 依赖
cd src-tauri
cargo fetch
cd ..
```

### 开发模式

```bash
# 启动开发服务器
npm run tauri dev
```

### 构建生产版本

```bash
# 构建应用
npm run tauri build

# 构建后的文件位于
src-tauri/target/release/bundle/
```

## 📁 项目结构

```
um-startup-manager/
├── 📁 src/                          # Vue 前端源码
│   ├── 📁 components/               # Vue 组件
│   │   ├── 📄 TitleBar.vue         # 自定义标题栏
│   │   ├── 📄 StartupList.vue      # 启动项列表
│   │   └── 📄 StartupItem.vue      # 启动项项组件
│   ├── 📁 assets/                  # 静态资源
│   ├── 📁 types/                   # TypeScript 类型定义
│   │   └── 📄 startup.ts           # 启动项类型定义
│   ├── 📄 App.vue                  # 主应用组件
│   ├── 📄 main.ts                  # 应用入口
│   └── 📄 vite-env.d.ts            # Vite 环境类型
├── 📁 src-tauri/                    # Tauri 后端源码
│   ├── 📁 capabilities/            # 权限配置
│   ├── 📁 icons/                   # 应用图标
│   ├── 📁 src/                     # Rust 源码
│   │   ├── 📄 main.rs              # Rust 入口文件
│   │   ├── 📄 lib.rs               # Tauri 命令定义
│   │   └── 📁 startup/             # 启动项管理模块
│   │       ├── 📄 mod.rs           # 模块入口
│   │       ├── 📄 scanner.rs       # 启动项扫描
│   │       ├── 📄 manager.rs       # 启动项管理
│   │       ├── 📄 monitor.rs       # 进程监控
│   │       ├── 📄 settings.rs      # 配置管理
│   │       └── 📄 icon.rs          # 图标处理
│   ├── 📄 build.rs                 # 构建脚本
│   ├── 📄 Cargo.toml               # Rust 依赖配置
│   ├── 📄 Cargo.lock               # Rust 依赖锁定
│   └── 📄 tauri.conf.json          # Tauri 配置文件
├── 📄 index.html                    # HTML 模板
├── 📄 package.json                  # NPM 依赖配置
├── 📄 package-lock.json            # NPM 依赖锁定
├── 📄 tsconfig.json                # TypeScript 配置
├── 📄 tsconfig.node.json           # Node TypeScript 配置
├── 📄 vite.config.ts               # Vite 配置
├── 📄 README.md                    # 项目文档
└── 📄 process_monitor_analysis.md  # 进程监控分析文档
```

## ⚙️ 配置说明

### 窗口配置

在 `src-tauri/tauri.conf.json` 中配置应用窗口属性：

```json
{
  "app": {
    "windows": [
      {
        "label": "main",
        "title": "UM启动项管理",
        "width": 800,
        "height": 600,
        "decorations": false, // 无边框设计
        "center": true, // 屏幕居中
        "resizable": false, // 禁止调整大小
        "maximizable": false // 禁止最大化
      }
    ]
  }
}
```

### 主题配置

支持深色模式自动切换：

```typescript
// 自动检测系统主题
const isDarkMode = ref(
  window.matchMedia("(prefers-color-scheme: dark)").matches
);

// 监听主题变化
window
  .matchMedia("(prefers-color-scheme: dark)")
  .addEventListener("change", (e) => {
    isDarkMode.value = e.matches;
    document.documentElement.classList.toggle("dark", isDarkMode.value);
  });
```

## 🔧 开发指南

### 添加新功能

1. **前端组件** (`src/components/`)

   - 创建新的 Vue 组件
   - 添加 TypeScript 类型定义
   - 实现响应式数据绑定

2. **后端命令** (`src-tauri/src/`)
   - 在 `lib.rs` 中定义新命令
   - 实现对应的 Rust 函数
   - 更新前端 API 调用

### 代码规范

- **Vue 组件**: 使用 Composition API
- **TypeScript**: 启用严格模式
- **Rust**: 遵循 Rust 最佳实践
- **CSS**: 使用 Tailwind CSS 样式

### 调试

```bash
# 前端调试
npm run dev

# 后端调试
npm run tauri dev

# 查看日志
# Windows: 查看应用事件日志
```

## 📦 构建和发布

### 构建配置

```bash
# 构建
npm run tauri build
```

### 发布检查清单

- [ ] 更新版本号
- [ ] 测试所有功能
- [ ] 检查界面适配
- [ ] 验证安全配置
- [ ] 更新文档

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

### 贡献流程

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 📄 许可证

本项目基于 MIT 许可证开源 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🙏 致谢

- [Tauri Team](https://tauri.app/) - 优秀的桌面应用框架
- [Vue Team](https://vuejs.org/) - 优雅的前端框架
- [Rust Community](https://www.rust-lang.org/) - 系统级编程语言

## 📞 支持

如果您觉得这个项目有用，请给个 ⭐️ Star！

如有问题或建议，请提交 [Issue](../../issues)。

---
